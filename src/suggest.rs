use futures::future::join_all;
use twitter_v2::{Error, User};

use crate::{
    env::get_self_user,
    relation::{get_related_ids, get_replying_users, Relation},
    utils::get_user_by_id,
};

pub async fn get_suggested_users() -> Result<Vec<User>, Error> {
    let self_user = get_self_user().await?;
    println!("self user: {:#?}", self_user);
    let self_followers = get_related_ids(self_user.id, Relation::Followers).await?;
    let (first_chunk, _) = self_followers.split_at(32);
    println!("first_chunk.len() = {:#?}", first_chunk.len());
    let replying_users_distinct = join_all(
        first_chunk
            .iter()
            .map(|u| async move { get_replying_users(u.username.clone()).await }),
    )
    .await;
    let mut replying_users_distinct = replying_users_distinct
        .iter()
        .flatten()
        .flat_map(|v| v.iter())
        .filter(|nid| {
            !self_followers.iter().map(|u| u.id).any(|x| x == **nid) && nid != &&self_user.id
        })
        .collect::<Vec<_>>();
    replying_users_distinct.dedup();
    println!(
        "replying_users_distinct.len = {:#?}",
        replying_users_distinct.len()
    );
    let replying_users_distinct = join_all(
        replying_users_distinct
            .iter()
            .map(|nid| async move { get_user_by_id(**nid).await }),
    )
    .await;
    let replying_users_distinct = replying_users_distinct
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    Ok(replying_users_distinct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "this test will print"]
    async fn sample_get_suggested_users() -> Result<(), twitter_v2::Error> {
        let suggested_users = get_suggested_users().await?;
        for user in suggested_users {
            println!("https://twitter.com/{}", user.username);
        }
        Ok(())
    }
}
