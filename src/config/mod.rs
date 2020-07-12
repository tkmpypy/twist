use dirs;
use egg_mode;
use std::fs;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader, ScanError};

use crate::constants::Constants;
use crate::service::twitter::Twist;
use crate::error::TwistError;

#[derive(Debug)]
pub struct Config {
    pub token:  egg_mode::Token,
    pub user_id:  u64,
    pub screen_name:  String,
}

impl Config {
    pub async fn load(conn_token: &egg_mode::KeyPair) -> Config {
        println!("load config");
        let conf = Config::load_inner(conn_token).await;
        if let Some(conf) = conf {
            return conf;
        }

        Config::load_inner(conn_token).await.unwrap()
    }

    async fn load_inner(conn_token: &egg_mode::KeyPair) -> Option<Config> {
        println!(
            "{}, {}",
            conn_token.key.to_string(),
            conn_token.secret.to_string()
        );
        if let Ok(conf_file) = Config::get_config_file().await {
            if let Ok(f) = fs::File::open(&conf_file) {
                let conf_str =
                    fs::read_to_string(conf_file).expect("Something went wrong reading the file");
                if let Ok(yaml) = YamlLoader::load_from_str(conf_str.as_str()) {
                    let conf = Config::read_config_from_yaml(conn_token, &yaml);
                    match conf {
                        Ok(c) => return Some(c),
                        Err(e) => {
                            Config::auth().await;
                        }
                    }
                } else {
                    // TODO: auth & renew config
                    Config::auth().await;
                    return None
                }
            }
            // TOOD 
            // Some(Config {})
            None
        } else {
            None
        }
    }

    async fn auth() {
        let request_token = Twist::get_request_token().await;
        println!("Go to the following URL, sign in, and give me the PIN that comes back:");
        println!("{}", egg_mode::auth::authorize_url(&request_token));
        
        let mut pin = String::new();
        std::io::stdin().read_line(&mut pin).unwrap();
        println!("");

    }

    fn read_config_from_yaml(
        conn_token: &egg_mode::KeyPair,
        yaml: &Vec<Yaml>,
    ) -> Result<Config, TwistError> {
        /// '''yaml
        ///  twist:
        ///    username: ""
        ///    userId: ""
        ///    key: ""
        ///    secret: ""
        /// '''
        let root = &yaml.get(0).ok_or(TwistError::EmptyConfig)?;
        let root = &root["twist"];
        println!("{:?}", root);
        let user_id = root["userId"][0].as_i64().unwrap();
        let user_id = u64::try_from(user_id);
        let user_name = root["username"][0].as_str().unwrap();
        let key = root["key"][0].as_str().unwrap();
        let secret = root["secret"][0].as_str().unwrap();

        let access_token = egg_mode::KeyPair::new(String::from(key), String::from(secret));
        let token = egg_mode::Token::Access {
            consumer: conn_token.clone(),
            access: access_token,
        };

        return Ok(Config {
            token,
            user_id: user_id.unwrap(),
            screen_name: String::from(user_name),
        })
    }

    async fn get_config_file() -> Result<PathBuf, &'static str> {
        let home_dir = dirs::home_dir();
        if let Some(home_dir) = home_dir {
            let dir_str = home_dir.to_str().unwrap();
            let file = format!("{}/{}", dir_str, Constants::CONFIG_FILE_NAME);
            let f_path = Path::new(file.as_str());
            if !f_path.exists() {
                println!("create setting file to {}", &file);
                let f = fs::File::create(f_path);
                match f {
                    Ok(f) => {
                        return Ok(f_path.to_owned())
                    },
                    Err(e) => return Err("can't create setting file"),
                }
            }
            Ok(f_path.to_owned())
        } else {
            return Err("can't get home directory.");
        }
    }
}
