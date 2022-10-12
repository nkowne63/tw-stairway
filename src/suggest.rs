use futures::future::join_all;
use twitter_v2::{Error, User};

use crate::{
    env::get_self_user,
    relation::{get_related_ids, get_replying_users, Relation},
};

pub async fn get_suggested_users() -> Result<Vec<User>, Error> {
    let self_user = get_self_user().await?;
    let self_followers = get_related_ids(self_user.id, Relation::Followers).await?;
    let (first_chunk, _) = self_followers.split_at(2);
    println!("first_chunk = {:#?}", first_chunk);
    let replying_users_distinct = join_all(
        first_chunk
            .iter()
            .map(|u| async move { get_replying_users(u.username.clone()).await }),
    )
    .await;
    println!("replying_users_distinct = {:#?}", replying_users_distinct);
    // let replying_users_distinct = replying_users_distinct
    //     .iter()
    //     .flatten()
    //     .flat_map(|v| v.iter())
    //     .filter(|nid| !self_followers.iter().map(|u| u.id).any(|x| x == **nid))
    //     .collect::<Vec<_>>();
    // println!("replying_users_distinct = {:#?}", replying_users_distinct);
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "this test will print"]
    async fn sample_get_suggested_users() -> Result<(), twitter_v2::Error> {
        let suggested_users = get_suggested_users().await?;
        println!("suggested_users = {:#?}", suggested_users);
        Ok(())
    }
}
