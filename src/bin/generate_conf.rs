use std::fs::File;

use ggez;

pub fn main() -> ggez::GameResult {
    let conf = ggez::conf::Conf::new();
    let mut config_file = File::create("conf_defaults.toml")?;
    conf.to_toml_file(&mut config_file)?;
    println!("Generated conf_defaults.toml");
    Ok(())
}
