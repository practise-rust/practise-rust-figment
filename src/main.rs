mod config;

use config::{load_config, Config, Database};
fn main() {
    let config = load_config().unwrap();
    println!("{:?}", config);
}
