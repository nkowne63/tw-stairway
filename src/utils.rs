use twitter_v2::{id::NumericId, Error, User};

use crate::env::CLIENT;

pub async fn get_user_by_id(id: NumericId) -> Result<User, Error> {
    let user = CLIENT.get_user(id).send().await?.into_data().unwrap();
    Ok(user)
}
