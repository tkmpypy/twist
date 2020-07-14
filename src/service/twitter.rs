use egg_mode;
use egg_mode::tweet::Tweet;

use crate::constants::{CONSUMER_KEY, CONSUMER_SECRET};
use crate::config::Config;
use crate::error::TwistError;

pub struct Twist {
    config: Config
}

impl Twist {
    pub async fn new() -> Self {
        let config = Twist::auth().await;
        if let Some(config) = config {
            return Twist {
                config
            }
        }
        panic!("can't authenticate")
    }

    pub async fn tweet(&self, text: String) -> Result<Tweet, crate::error::TwistError> {
        let tweet = egg_mode::tweet::DraftTweet::new(text);
        let result = tweet.send(&self.config.token).await;
        match result {
            Ok(res) => {
                println!("created_at {} id: {}", res.response.created_at, res.response.id);
                Ok(res.response)
            },
            Err(_) => Err(TwistError::TweetFailure)
        }
    }

    async fn auth() -> Option<Config> {
        let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

        Config::load(&con_token).await
    }

    pub async fn get_request_token() -> egg_mode::auth::KeyPair {
        let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

        egg_mode::auth::request_token(&con_token, "oob").await.unwrap()

    }
}
