use std::io::Write;
use std::fs::File;

extern crate reqwest;
extern crate dirs;

pub fn update_fonts() {
    let links = download_index();
    for link in links {
        let response: reqwest::blocking::Response = reqwest::blocking::get(&link).unwrap();
        let filename = link.split("/").last().unwrap();
        let mut path = dirs::config_dir().unwrap();
        path.push("rust_figlet");
        path.push(&filename);
        if response.status().is_success() {
            let mut file = File::create(path).unwrap();
            let data: String = response.text().unwrap();
            file.write(data.as_bytes()).unwrap();
            println!("Downloaded {}",&filename);
        } else {
          println!("Error with downloading from {}", response.url());
        }
    }
    println!("Done!");
}

fn download_index() -> Vec<String> {
    let raw_body: String = reqwest::blocking::get("https://gist.githubusercontent.com/KanarekLife/3de34e50e08aa20f11f24d39a6fd2fff/raw").unwrap().text().unwrap();
    raw_body.split_ascii_whitespace().map(|x| x.to_string()).collect()
}