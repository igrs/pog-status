extern crate csv;
extern crate encoding;
extern crate hyper;
extern crate kuchiki;
extern crate rustc_serialize;

use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;

use encoding::{Encoding, DecoderTrap};
use encoding::all::EUC_JP;

use hyper::Client;

use kuchiki::Html;

static SITE_URL: &'static str = "http://db.netkeiba.com/horse/";

fn main() {
    let paths: Vec<String> = env::args().collect();

    if paths.len() > 1 {

        let client = Client::new();

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
                if !&line.is_empty() {
                    let url = SITE_URL.to_string() + &line;
                    let mut res = client.get(&url)
                                        .send()
                                        .unwrap();
                    let mut body_bytes = vec![];
                    if res.read_to_end(&mut body_bytes).is_ok() {
                        let html = EUC_JP.decode(&body_bytes, DecoderTrap::Strict).unwrap();
                        println!("{}", &html);
                        let doc = Html::from_string(html).parse();
                    }
                    else {
                        println!("Can not read response body.");
                    }

                }
            }
        }
    }
    else {
        println!("Usage: pog_status(.exe windows) [file path]");
    }
}
