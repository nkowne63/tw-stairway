use std::collections::{BTreeMap, BTreeSet};

use futures::future::join_all;
use twitter_v2::{Error, User};

use crate::{
    blacklist::get_blacklist,
    env::get_self_user,
    relation::{get_related_ids, get_replying_users, get_user_by_id, Relation},
    utils::{pick_random_iter, vec2btree},
};

static SEARCH_SIZE: usize = 128;

pub async fn get_suggested_users() -> Result<Vec<User>, Error> {
    // 自身を取得
    let self_user = get_self_user().await?;
    println!("self user: {:#?}", self_user);
    // 自分のフォロワーを取得
    let self_followers = get_related_ids(self_user.id, Relation::Followers).await?;
    // フォロワーからランダムな数を選び、それらへのリプライをしているユーザーを取得
    let random_chunk = pick_random_iter(&self_followers, SEARCH_SIZE);
    println!("random_chunk.len() = {:#?}", random_chunk.len());
    let replying_users_distinct = join_all(
        random_chunk
            .iter()
            .map(|u| async move { get_replying_users(u.username.clone()).await }),
    )
    .await;
    // リプライをしているユーザーからフォロワーと自分を除く
    let mut remove_id_set = vec2btree(self_followers.iter().map(|u| u.id).collect());
    remove_id_set.insert(self_user.id);
    let replying_users_distinct = replying_users_distinct
        .iter()
        .flatten()
        .flat_map(|v| v.iter())
        .copied()
        .collect::<BTreeSet<_>>();
    let replying_users_distinct = replying_users_distinct.difference(&remove_id_set);
    // リプライをしているユーザーのユーザー情報を取得
    let replying_users_distinct =
        join_all(replying_users_distinct.map(|nid| async move { get_user_by_id(*nid).await }))
            .await;
    let replying_users_by_username = BTreeMap::from_iter(
        replying_users_distinct
            .iter()
            .flatten()
            .map(|u| (u.username.clone(), u.clone())),
    );
    let replying_usernames_distinct = replying_users_distinct
        .iter()
        .flatten()
        .map(|u| u.username.clone())
        .collect::<BTreeSet<_>>();
    let suggested_usernames = replying_usernames_distinct
        .difference(&get_blacklist())
        .cloned()
        .collect::<Vec<_>>();
    let suggested_users = suggested_usernames
        .iter()
        .map(|un| replying_users_by_username.get(un).unwrap().clone())
        .collect::<Vec<_>>();
    println!(
        "replying_users_distinct.len() = {:#?}",
        replying_usernames_distinct.len()
    );
    Ok(suggested_users)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "this test will print"]
    async fn sample_get_suggested_users() -> Result<(), twitter_v2::Error> {
        let suggested = get_suggested_users().await?;
        for user in suggested {
            println!("https://twitter.com/{}", user.username);
            println!(
                "description = {}",
                user.description.unwrap_or_else(|| "N#A".to_string())
            );
        }
        Ok(())
    }
}
