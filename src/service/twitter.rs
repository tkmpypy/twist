use egg_mode;

use crate::constants::Constants;
use crate::config::Config;

pub struct Twist {
    config: Config
}

impl Twist {
    pub async fn new() -> Self {
        let config = Twist::auth().await;
        Twist {
            config
        }
    }

    pub fn tweet(self, text: &str) {
        println!("{}", String::from(text));
    }

    async fn auth() -> Config{
        let key = Constants::CONSUMER_KEY;
        let secret = Constants::CONSUMER_SECRET;
        let con_token = egg_mode::KeyPair::new(key, secret);

        Config::load(&con_token).await
    }

    pub async fn get_request_token() -> egg_mode::auth::KeyPair {
        let key = Constants::CONSUMER_KEY;
        let secret = Constants::CONSUMER_SECRET;
        let con_token = egg_mode::KeyPair::new(key, secret);

        egg_mode::auth::request_token(&con_token, "oob").await.unwrap()

    }
}
