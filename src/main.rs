extern crate csv;
extern crate rustc_serialize;

use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;

//static SITE_URL: &'static str = "http://www.netkeiba.com/horse/";

fn main() {
    let paths: Vec<String> = env::args().collect();
    if paths.len() > 1 {
        for i in 1..paths.len() {
            let path = &paths[i];
            let file = match File::open(path) {
                Ok(file) => file,
                Err(err) => {
                    println!("{}", err.to_string());
                    continue;
                }
            };
            let buf = BufReader::new(file);
            for line in buf.lines().filter_map(|result| result.ok()) {
                println!("{}", line);
            }
        }
    }
    else {
        println!("Usage: pog_status(.exe windows) [file path]");
    }
}
