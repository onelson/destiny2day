extern crate reqwest;
#[macro_use]
extern crate hyper;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use serde_json::Value;
use hyper::header::Headers;

use std::io::Read;

header! { (XAPIKey, "X-API-Key") => [String] }
const BASE: &'static str = "https://www.bungie.net/Platform/Destiny2/";

fn make_headers() -> Headers {
    let api_key = env::var("BUNGIE_API_KEY").expect("missing bungie api env var");
    let mut headers = Headers::new();
    headers.set(XAPIKey(api_key.to_owned()));
    headers
}

fn make_request(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let headers = make_headers();
    let mut resp = client.get(url).headers(headers).send()?;
    let mut content = String::new();
    let _ = resp.read_to_string(&mut content);
    Ok(content)
}

fn get_milestones() -> Result<Value, serde_json::Error> {
    let url = format!("{}/Milestones/", BASE);
    let resp = make_request(&url).unwrap();
    let data: Value = serde_json::from_str(&content)?;
    Ok(data["Response"].clone())
}

fn get_milestone_details(milestone_hash: &str) -> Result<Value, serde_json::Error> {
    let url = format!("{}/Milestones/{}/Content/", BASE, milestone_hash);
    let content = make_request(&url).unwrap();
    let data: Value = serde_json::from_str(&content)?;
    Ok(data["Response"].clone())
}


//fn main() {
//
//    let data = get_milestones().unwrap();
//    let milestones = data.as_object().unwrap();
//
//    for key in milestones.keys() {
//        let details = get_milestone_details(&key).unwrap();
//        let description = details["about"].to_string();
//
//        println!("\n{}:\n{}", key, description);
//
//        if let Some(quests) = milestones[key]["availableQuests"].as_array() {
//            for quest in quests {
//                println!("{}", quest.to_string());
//            }
//        } else {
//            println!("No quests.")
//        }
//    }
//}
