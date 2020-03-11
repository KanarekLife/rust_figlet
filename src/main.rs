use std::env;
use std::path::{PathBuf};
use std::process::exit;
use rand::Rng;

extern crate rand;

mod figlet;
mod network;

fn main() {
    std::fs::create_dir(get_directory());
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--help".to_string()) {
        display_app_help();
    }else if args.contains(&"--version".to_string()) {
        display_app_info();
    }else if args.contains(&"--list-fonts".to_string()) {
        list_fonts();
    }else if args.contains(&"--update".to_string()) {
        network::update_fonts();
    }else {
        let mut font = "big";
        let mut random = false;
        if args.contains(&"--font".to_string()) {
            let position = args.iter().position(|r| r == "--font").unwrap();
            if args.len() > (position+1) {
                font = args.iter().nth(position+1).unwrap();
            } else {
                println!("No font provided!!!");
                exit(1);
            }

        }
        if args.contains(&"--random".to_string()) {
            random = true;
        }
        display_output("sample", font, random)
    }
}

fn display_app_info() {
    println!("{} - {} - created by {} @ 2020 {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_REPOSITORY"));
}

fn display_app_help() {
    display_app_info();
    println!("-----------------------------------");
    println!("Available options:");
    println!(" --help -> show help");
    println!(" --version -> show app version");
    println!(" --font {{font name}} -> selects font to be used");
    println!(" --random -> selects random font");
    println!(" --list-fonts -> list all available fonts");
    println!(" --update -> update fonts");
    println!("-----------------------------------");
}

fn list_fonts() {
    println!("Available fonts:");
    for font in std::fs::read_dir(get_directory()).unwrap() {
        println!("{}",font.unwrap().file_name().to_str().unwrap())
    }
    println!("Use --update to make sure you have as much as you can!");
}
fn get_directory() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("rust_figlet");
    dir
}

fn display_output(text: &str, font: &str, random : bool) {
    let mut filename: String;
    let mut path: PathBuf;
    filename = font.to_string();
    filename.push_str(".flf");
    path = get_directory();
    path.push(filename);
    if random {
        let mut rng = rand::thread_rng();
        let dir = std::fs::read_dir(get_directory()).unwrap();
        let n = rng.gen_range(0,dir.count()-1);
        for (i, file) in std::fs::read_dir(get_directory()).unwrap().enumerate() {
            if i == n {
                path = file.unwrap().path();
                break;
            }
        }
    }
    let font_file = std::fs::File::open(path).expect("Font file not found! Update fonts using --update!");
    let reader = std::io::BufReader::new(font_file);
    let font = figlet::Font::new(reader);
    println!("{}",font.parse_text(text));
}