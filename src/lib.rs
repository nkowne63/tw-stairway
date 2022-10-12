use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static ENV: Lazy<HashMap<String, String>> = Lazy::new(|| {
    dotenv().ok();
    std::env::vars().collect()
});

static APP_BEARER_TOKEN: Lazy<String> = Lazy::new(|| {
    ENV.get("APP_BEARER_TOKEN")
        .expect("APP_BEARER_TOKEN is not set")
        .to_string()
});

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
}
