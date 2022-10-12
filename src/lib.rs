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

    #[test]
    #[ignore]
    fn it_works() {
        println!("APP_BEARER_TOKEN = {}", APP_BEARER_TOKEN.as_str());
    }
}
