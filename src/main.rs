extern crate regex;
extern crate walkdir;
// LOC-core-loading-toml:<use the toml library>
extern crate toml;
extern crate strfmt;
extern crate time;

#[macro_use]
extern crate lazy_static;

pub mod core;

fn main() {
    println!("importing");
    // match core::load::recursive_raw_load("docs") {
    //     Ok(_) => println!("success"),
    //     Err(err) => println!("error: {}", err),
    // }
}
