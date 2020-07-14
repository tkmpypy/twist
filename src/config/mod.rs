use dirs;
use egg_mode;
use std::convert::TryFrom;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

use crate::error::TwistError;
use crate::service::twitter::Twist;
use crate::constants::CONFIG_FILE_NAME;

#[derive(Debug)]
pub struct Config {
    pub token: egg_mode::Token,
    pub user_id: u64,
    pub screen_name: String,
}

impl Config {
    
    pub async fn load(conn_token: &egg_mode::KeyPair) -> Option<Config> {
        println!("load config");
        Config::load_inner(conn_token).await
    }

    async fn load_inner(conn_token: &egg_mode::KeyPair) -> Option<Config> {
        println!(
            "{}, {}",
            conn_token.key.to_string(),
            conn_token.secret.to_string()
        );
        if let Ok(conf_file) = Config::get_config_file().await {
            if let Ok(_) = fs::File::open(&conf_file) {
                let config_result = Config::get_config(conn_token, &conf_file);
                match config_result {
                    Some(config_result) => return Some(config_result),
                    _ => {
                        Config::auth(&conf_file, conn_token).await;
                        let config = Config::get_config(conn_token, &conf_file).unwrap();
                        return Some(config);
                    }
                }
            }
            // TOOD
            // Some(Config {})
            return None;
        }

        None
    }

    fn get_config(conn_token: &egg_mode::KeyPair, conf_file: &PathBuf) -> Option<Config> {
        let conf_str =
            fs::read_to_string(conf_file).expect("Something went wrong reading the file");
        if let Ok(yaml) = YamlLoader::load_from_str(conf_str.as_str()) {
            let conf = Config::read_config_from_yaml(conn_token, &yaml);
            if let Ok(conf) = conf {
                return Some(conf);
            }
        }

        None
    }

    async fn auth(file: &PathBuf, conn_token: &egg_mode::KeyPair) {
        let request_token = Twist::get_request_token().await;
        println!("Go to the following URL, sign in, and give me the PIN that comes back:");
        println!("{}", egg_mode::auth::authorize_url(&request_token));

        let mut pin = String::new();
        std::io::stdin().read_line(&mut pin).unwrap();
        println!("");

        let tok_result = egg_mode::auth::access_token(conn_token.to_owned(), &request_token, pin)
            .await
            .unwrap();

        let token = tok_result.0;
        let user_id = tok_result.1;
        let username = tok_result.2;

        match token {
            egg_mode::Token::Access {
                access: ref access_token,
                ..
            } => {
                if let Ok(_) = Config::write_config_to_yaml(file, access_token, &user_id, &username) {
                    println!("sccess create config file.");
                }
            }
            _ => unreachable!(),
        }
    }

    fn write_config_to_yaml(
        file: &PathBuf,
        access_token: &egg_mode::KeyPair,
        user_id: &u64,
        username: &String,
    ) -> std::result::Result<(), std::io::Error> {
        let y = format!(
            "
            twist:
                username: {}
                userId: {}
                key: {}
                secret: {}
            ",
            username, user_id, access_token.key, access_token.secret
        );
        let docs = YamlLoader::load_from_str(&y).unwrap();
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&docs[0]).unwrap();

        let mut file = fs::File::create(file)?;
        file.write_all(out_str.as_bytes())?;

        Ok(())
    }

    fn read_config_from_yaml(
        conn_token: &egg_mode::KeyPair,
        yaml: &Vec<Yaml>,
    ) -> Result<Config, TwistError> {
        let root = &yaml.get(0).ok_or(TwistError::EmptyConfig)?;
        let root = &root["twist"];
        println!("{:?}", root);
        let user_id = root["userId"].as_i64().unwrap();
        let user_id = u64::try_from(user_id);
        let user_name = root["username"].as_str().unwrap();
        let key = root["key"].as_str().unwrap();
        let secret = root["secret"].as_str().unwrap();

        let access_token = egg_mode::KeyPair::new(String::from(key), String::from(secret));
        let token = egg_mode::Token::Access {
            consumer: conn_token.clone(),
            access: access_token,
        };

        return Ok(Config {
            token,
            user_id: user_id.unwrap(),
            screen_name: String::from(user_name),
        });
    }

    async fn get_config_file() -> Result<PathBuf, &'static str> {
        let home_dir = dirs::home_dir();
        if let Some(home_dir) = home_dir {
            let dir_str = home_dir.to_str().unwrap();
            let file = format!("{}/{}", dir_str, CONFIG_FILE_NAME);
            let f_path = Path::new(file.as_str());
            if !f_path.exists() {
                println!("create setting file to {}", &file);
                let f = fs::File::create(f_path);
                match f {
                    Ok(f) => return Ok(f_path.to_owned()),
                    Err(e) => return Err("can't create setting file"),
                }
            }
            Ok(f_path.to_owned())
        } else {
            return Err("can't get home directory.");
        }
    }
}
