pub struct Constants<'a> {
    CONSUMER_KEY: &'a str,
    CONSUMER_SECRET: &'a str,
    CONFIG_FILE_NAME: &'a str,
}

impl<'a> Constants<'a> {
    pub const CONSUMER_KEY: &'a str = "RijkoGPeWHx8FBwEGAF8DGjFR";
    pub const CONSUMER_SECRET: &'a str = "yozQWrZeb12Bc5kSELgeu0XXYf7GPurIfFlKhrRw0yKnh6TZIm";
    pub const CONFIG_FILE_NAME: &'a str = ".twist.yaml";
}
