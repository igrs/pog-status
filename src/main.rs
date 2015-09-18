extern crate csv;
extern crate rustc_serialize;

use std::env;

static SITE_URL: &'static str = "http://www.netkeiba.com/horse/";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        for a in &args {
            println!("{:?}", a);
        }
    }
    else {
        println!("need arguments");
    }

    //let mut rdr = csv::Reader::from_file(arg).unwrap();
    //let mut client = Client::new();
}
