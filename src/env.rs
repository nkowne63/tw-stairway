use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use twitter_v2::{authorization::BearerToken, Error, TwitterApi, User};

static ENV: Lazy<HashMap<String, String>> = Lazy::new(|| {
    dotenv().ok();
    std::env::vars().collect()
});

pub static CLIENT: Lazy<TwitterApi<BearerToken>> = Lazy::new(|| {
    let app_bearer_token = ENV
        .get("APP_BEARER_TOKEN")
        .expect("APP_BEARER_TOKEN is not set");
    TwitterApi::new(BearerToken::new(app_bearer_token.as_str()))
});

pub async fn get_self_user() -> Result<User, Error> {
    let user = CLIENT
        .get_user_by_username(ENV.get("SELF_USERNAME").unwrap().as_str())
        .send()
        .await?
        .into_data()
        .expect("this user should exist");
    Ok(user)
}
