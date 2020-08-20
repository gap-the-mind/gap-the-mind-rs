use gap_the_mind::Store;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let path = Path::new(path);
    let store = Store::new(path);

    let res = store.query();
    println!("{:?}", res)
}
