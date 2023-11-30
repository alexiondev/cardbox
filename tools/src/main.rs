use std::env;

use anyhow;
use simple_logger::SimpleLogger;

mod import;

fn main() -> anyhow::Result<()>{
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(AsRef::as_ref).collect();

    match args[1..] {
        ["import", path] => import::import_named(path),
        _ => {
            println!("Unknown utility");
            Ok(())
        }
    }
}
