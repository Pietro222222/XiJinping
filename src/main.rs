mod config;
use config::bot_config;
fn main() {
    let cfg = bot_config::Config::new();
    println!("{:?}", cfg);
}
