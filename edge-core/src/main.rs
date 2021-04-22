mod store;

use store::rusqlite;

fn main() {
    // 1. init store
    let result = rusqlite::initial_store();
    match result {
        Ok(()) => println!(),
        Err(err) => println!("{}", err),
    }
    // 2. init mqtt conn to the edge-pub
    // 3. inti http server
}
