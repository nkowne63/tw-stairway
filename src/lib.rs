use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use twitter_v2::{authorization::BearerToken, id::NumericId, ApiPayload, Error, User};

static ENV: Lazy<HashMap<String, String>> = Lazy::new(|| {
    dotenv().ok();
    std::env::vars().collect()
});

static APP_BEARER_TOKEN: Lazy<String> = Lazy::new(|| {
    ENV.get("APP_BEARER_TOKEN")
        .expect("APP_BEARER_TOKEN is not set")
        .to_string()
});

static CLIENT: Lazy<twitter_v2::TwitterApi<BearerToken>> =
    Lazy::new(|| twitter_v2::TwitterApi::new(BearerToken::new(APP_BEARER_TOKEN.as_str())));

enum Relation {
    Following,
    Followers,
}

async fn get_related_ids(id: NumericId, rel: Relation) -> Result<Vec<NumericId>, Error> {
    let mut ids = Vec::new();
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
        if let Some(users) = data {
            if users.is_empty() {
                break;
            }
            let current_ids = users
                .into_iter()
                .map(|user| user.id)
                .collect::<Vec<NumericId>>();
            ids.extend(current_ids);
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
    Ok(ids)
}

async fn get_self_user() -> Result<User, Error> {
    let user = CLIENT
        .get_user_by_username(ENV.get("SELF_USERNAME").unwrap().as_str())
        .send()
        .await?
        .into_data()
        .expect("this user should exist");
    Ok(user)
}

async fn get_self_lv2() -> Result<Vec<NumericId>, Error> {
    let self_user = get_self_user().await?;
    let followings = get_related_ids(self_user.id, Relation::Following).await?;
    for following in followings {
        let following_lv2 = get_related_ids(following, Relation::Following).await?;
        println!("count: {}", following_lv2.len());
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use twitter_v2::authorization::BearerToken;
    use twitter_v2::query::TweetField;
    use twitter_v2::TwitterApi;

    #[test]
    #[ignore]
    fn it_works() {
        println!("APP_BEARER_TOKEN = {}", APP_BEARER_TOKEN.as_str());
    }

    #[tokio::test]
    async fn sample_twitter_v2() -> Result<(), twitter_v2::Error> {
        let auth = BearerToken::new(APP_BEARER_TOKEN.as_str());
        let tweet = TwitterApi::new(auth)
            .get_tweet(1261326399320715264)
            .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        assert_eq!(tweet.id, 1261326399320715264);
        assert_eq!(tweet.author_id.unwrap(), 2244994945);
        assert_eq!(
            tweet.created_at.unwrap(),
            datetime!(2020-05-15 16:03:42 UTC)
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore = "this test is printing something"]
    async fn sample_get_follows_and_following() -> Result<(), twitter_v2::Error> {
        let user = get_self_user().await?;
        println!("user = {:#?}", user);
        let followings = get_related_ids(user.id, Relation::Following).await?;
        println!("followings = {:#?}", followings.len());
        let followers = get_related_ids(user.id, Relation::Followers).await?;
        println!("followings = {:#?}", followers.len());
        Ok(())
    }

    #[tokio::test]
    #[ignore = "this test is printing something"]
    async fn get_lv2_followings() -> Result<(), twitter_v2::Error> {
        let _ = get_self_lv2().await?;
        Ok(())
    }
}
