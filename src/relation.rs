use crate::env::CLIENT;
use twitter_v2::{id::NumericId, query::TweetField, ApiPayload, Error, User};

pub enum Relation {
    Following,
    Followers,
}

pub async fn get_related_ids(id: NumericId, rel: Relation) -> Result<Vec<User>, Error> {
    let mut users = Vec::new();
    let mut next_token: Option<String> = None;
    loop {
        let mut request = match rel {
            Relation::Following => CLIENT.get_user_following(id),
            Relation::Followers => CLIENT.get_user_followers(id),
        };
        let mut request = request.max_results(1000);
        if let Some(token) = next_token {
            request = request.pagination_token(token.as_str());
        }
        let ApiPayload {
            data,
            meta,
            includes: _,
            errors: _,
        } = request.send().await?.into_payload();
        if let Some(current_users) = data {
            if current_users.is_empty() {
                break;
            }
            users.extend(current_users);
        } else {
            break;
        }
        if let Some(meta) = meta {
            if meta.next_token.is_none() {
                break;
            }
            let temp_next_token = meta.next_token.expect("there exists next_token");
            next_token = Some(temp_next_token);
        } else {
            break;
        }
    }
    Ok(users)
}

pub async fn get_replying_users(user_name: String) -> Result<Vec<NumericId>, Error> {
    let tweets = CLIENT
        .get_tweets_search_recent(format!("to:{}", user_name))
        .tweet_fields([TweetField::AuthorId])
        .max_results(100)
        .send()
        .await?
        .into_data();
    if tweets.is_none() {
        return Ok(vec![]);
    }
    let author_ids = tweets
        .expect("there exists tweets")
        .iter()
        .map(|t| t.author_id.unwrap())
        .collect::<Vec<_>>();
    Ok(author_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "this test will print"]
    async fn sample_get_follows_and_following() -> Result<(), twitter_v2::Error> {
        let user = crate::env::get_self_user().await?;
        println!("user = {:#?}", user);
        let followings = get_related_ids(user.id, Relation::Following).await?;
        println!("followings = {:#?}", followings.len());
        let followers = get_related_ids(user.id, Relation::Followers).await?;
        println!("followings = {:#?}", followers.len());
        Ok(())
    }

    #[tokio::test]
    #[ignore = "this test will print"]
    async fn sample_get_replying() -> Result<(), twitter_v2::Error> {
        let user_name = crate::env::get_self_user().await?.username;
        println!("user_name = {:#?}", user_name);
        let replying_users = get_replying_users(user_name).await?;
        println!("replying_users = {:#?}", replying_users);
        let first_id = replying_users
            .first()
            .expect("there exists replying users")
            .to_owned();
        let first_user = CLIENT.get_user(first_id).send().await?.into_data().unwrap();
        println!("first_user = {:#?}", first_user);
        Ok(())
    }
}
