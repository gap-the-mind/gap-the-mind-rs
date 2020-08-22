use gap_the_mind::{StorageContext, Store};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let path = Path::new(path);

    let ctx = StorageContext {};

    let store = Store::new(path, ctx);
    let store = store.unwrap();

    let res = store.query();
    let res = res.unwrap();
    println!("{:?}", toml::to_string_pretty(&res));
}
