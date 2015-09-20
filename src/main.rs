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
                        let doc = Html::from_string(html).parse();
                        // horse name
                        for h1 in doc.select("h1").unwrap() {
                            let h1_node = h1.as_node();
                            let text_node = h1_node.first_child().unwrap();
                            let text = text_node.as_text().unwrap().borrow();
                            println!("{}", text.to_string());
                        }
                        // profile
                        for table in doc.select("table.db_prof_table").unwrap() {
                            let trs = table.as_node().select("tr").unwrap();
                            for tr in trs {
                                let tr_node = tr.as_node();

                                let th = tr_node.select("th").unwrap().nth(0).unwrap();
                                let td = tr_node.select("td").unwrap().nth(0).unwrap();

                                let th_node = th.as_node();
                                let td_node = td.as_node();

                                if !th_node.first_child().is_none() && !td_node.first_child().is_none() {
                                    let th_text_node = th_node.first_child().unwrap();
                                    let td_text_node = td_node.first_child().unwrap();

                                    if !th_text_node.as_text().is_none() && !td_text_node.as_text().is_none() {
                                        let th_text = th_text_node.as_text().unwrap().borrow();
                                        let td_text = td_text_node.as_text().unwrap().borrow();
                                        println!("{} -> {}", th_text.to_string(), td_text.to_string());
                                    }
                                }

                            }
                        }
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
