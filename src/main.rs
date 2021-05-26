use sqlite;

use reqwest::blocking::Client;
use reqwest::header;
use reqwest::header::HeaderMap;

use scraper::Html;
use scraper::Selector;

use std::env;
use std::process;

use url::Url;

fn main() {
    let mut args: env::Args = env::args();

    let mut db:String = String::from("");
    let mut url:String = String::from("");
    let (mut stride , mut start , mut max) : (usize, usize, usize) = (1,1,10);

    if args.len() < 3 {
        eprintln!("Not enough args");
        process::exit(1);
    }

    loop {
        let arg:String = match args.next(){
            Some(arg) => arg,
            None => break
        };
        let arg_eq_index:usize = match arg.find('=') {
            Some(position) => position ,
            None => continue
        };
        let arg_key:&str =  &arg[..arg_eq_index];
        let arg_val:&str =  &arg[(arg_eq_index+1)..];

        match arg_key {
            "db" => {
                db = arg_val.to_string()
            },
            "url" => {
                url = arg_val.to_string()
            },
            "stride" => {
                stride = arg_val.parse::<usize>().unwrap()
            },
            "start" => {
                start = arg_val.parse::<usize>().unwrap()
            },
            "max" => {
                max = arg_val.parse::<usize>().unwrap()
            },
            _ => {}
        }
    }

    let mut error_str:String = String::from("");
    if db == "" {
        error_str.push_str(" db=\"test.db\"");
    }
    if url == ""{
        error_str.push_str(" url=\"https://test.com/#d\"");
    }
    if error_str != ""{
        eprintln!("Required param missing: {}" , error_str);
        process::exit(1);
    }

    if url.find("#d") == None{
        error_str.push_str(" url missing #d page inserter(url=\"https://test.com/#d\") ");
    }
    if error_str != ""{
        eprintln!("Formatting error: {}" , error_str);
        process::exit(1);
    }

    let connection = sqlite::open(db).unwrap();
    match connection
        .execute(
            "
                CREATE TABLE domain_hunt (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    domain TEXT UNIQUE
                );
            ",
        ) {
            Ok(_a) => {}
            Err(e) => eprintln!("{}" , e)
        }

    let client = Client::new();
    let mut previous_request: String = String::from("");
    let mut headers = HeaderMap::new();
    // headers.insert(header::HOST, "www.bing.com".parse().unwrap());
    headers.insert(header::USER_AGENT, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0".parse().unwrap());
    for page in (start..max).step_by(stride){
        let hunt_url:String = url.replace("#d", &page.to_string());
        println!("Starting: {} Max: {} Stride: {}" , hunt_url , max , stride);
        let response = match client
            .get(hunt_url)
            .headers(headers.clone())
            .send() {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("{}" , e);
                    process::exit(1);
                }
            };

        if !response.status().is_success(){
            println!("Request failed: {:?}" , response.status());
            process::exit(1);
        }
        match response.text() {
            Ok(txt) => {
                if previous_request == txt{
                    eprintln!("Repeated Results");
                    process::exit(1);
                } else{
                    let fragment = Html::parse_fragment(&txt);
                    // let selector = Selector::parse("a").unwrap();
                    let selector = Selector::parse("li.b_algo > h2 > a").unwrap();
                    for sel in fragment.select(&selector){
                        match sel.value().attr("href"){
                            Some(href) => {
                                let write_url = match Url::parse(href) {
                                    Ok(h) => match h.host_str(){
                                        Some(hstr) => hstr.to_string(),
                                        None =>continue
                                    }
                                    Err(_e) => continue
                                };
                                println!("{:?}" , write_url);
                                let mut cursor = connection
                                    .prepare("INSERT INTO domain_hunt VALUES (NULL , ?)")
                                    .unwrap()
                                    .into_cursor();
                                cursor.bind(&[sqlite::Value::String(write_url)]).unwrap();
                                match cursor.next() {
                                    Ok(_dom) => {},
                                    Err(_e) => {}
                                };
                            },
                            None => {}
                        }
                    }
                    previous_request = txt;
                }
            },
            Err(e) => {
                eprintln!("Text Err: {:?}", e);
                process::exit(1);
            }
        }
    }
}
