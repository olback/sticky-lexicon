/*
 *  Sticky Lexicon
 *  https://github.com/olback/sticky-lexicon
 *  © olback 2018
 */
extern crate clipboard_win;
extern crate serde_json;
extern crate curl;

use clipboard_win::Clipboard;
use serde_json::Value;
use curl::http;

// const API_URL: &str = "https://api.pearson.com/v2/dictionaries/laad3/entries?search=";
const API_URL: &str = "https://api.pearson.com/v2/dictionaries/laad3/entries?headword=";

fn main() {

    println!("\n--------------------------------------------");
    println!("  Sticky Lexicon - word lookup on the fly!");
    println!("--------------------------------------------\n");

    // Get the current clipboard string
    let cbd = Clipboard::new().unwrap().get_string().unwrap();
    println!("-- old clipboard data --\n{}\n", cbd);

    let uri = format!("{}{}", API_URL, cbd);
    
    let resp = http::handle()
        .get(uri.as_str())
        .exec()
        .unwrap_or_else(|e| {
            panic!("Failed to get {}; error is {}", uri, e);
    });

    if resp.get_code() != 200 {
        println!("HTTP response code not OK, {}", resp.get_code());
        let result = Clipboard::new().unwrap().set_string("Something went wrong.\r\n");
        match result {
            Ok(_) => println!("Something went wrong\n"),
            Err(e) => println!("Error setting clipboard data: {:?}\n", e),
        }
        return;
    }

    let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Failed to parse response from {}; error is {}", uri, e);
    });

    let v: Value = serde_json::from_str(body).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    });

    let mut definitions = String::new();
    for i in 0..10 {
        if v["results"][i]["senses"][0]["definition"] != Value::Null {
            // println!(" - {}", v["results"][i]["senses"][0]["definition"]);
            definitions.push_str(format!(" - {} - {} - {}\r\n", v["results"][i]["headword"], v["results"][i]["part_of_speech"], v["results"][i]["senses"][0]["definition"]).as_str());
        }
    }

    if definitions.is_empty() {
        let result = Clipboard::new().unwrap().set_string("No definitions found.\r\n");
        match result {
            Ok(_) => println!("No definitions found\n"),
            Err(e) => println!("Error setting clipboard data: {:?}\n", e),
        }
        return;
    }

    let mut ncbd: String = format!("Definitions of \"{}\":\r\n", cbd);
    ncbd.push_str(definitions.as_str());

    // println!("{}", ncbd); define

    let result = Clipboard::new().unwrap().set_string(ncbd.as_str());
    match result {
        Ok(_) => println!("-- new clipboard data --\n{}.", ncbd),
        Err(e) => println!("Error setting clipboard data: {:?}", e),
    }
    println!("\n");
}
