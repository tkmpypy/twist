use clap::{App, Arg};


use twist::service::twitter::Twist;

#[tokio::main]
async fn main() {
    let matches = App::new("twist")
        .version("1.0.0")
        .author("tkmpypy <tkmpypy@gmail.com>")
        .about("tweet cli application")
        .arg(
            Arg::new("TWEET")
                .about("input your tweet")
                .required(true)
                .index(1),
        )
        .get_matches();

    match matches.value_of("TWEET") {
        Some(res) => {
            let twist = Twist::new().await;
            let _ = twist.tweet(res.to_string()).await;
        },
        None => panic!("required tweet."),
    }
}
