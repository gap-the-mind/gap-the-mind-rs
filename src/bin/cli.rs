use gap_the_mind::{StorageContext, Store, StoreError};
use std::env;
use std::path::Path;

fn main() -> Result<(), StoreError> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let path = Path::new(path);

    let ctx = StorageContext {};

    let store = Store::new(path, ctx)?;

    let res = store.query()?;
    println!("{:?}", toml::to_string_pretty(&res));

    store.list_all();

    Ok(())
}
