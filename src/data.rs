use clap::{value_parser, Arg};
use colored::*;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, net::IpAddr};

#[derive(Serialize, Deserialize, Debug)]
struct Prog {
    name: String,
    version: f32,
    about: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Merge {
    author: Author,
    prog: Prog,
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

pub fn init_args() -> (u16, String) {
    crate::ascii_init();
    let _ = check_folder_handler();
    let json_data = include_str!(".config/config.json");
    let data: Merge = serde_json::from_str(json_data).expect("Invalid JSON format");
    let new_app = clap::Command::new(data.prog.name)
        .about(data.prog.about)
        .author(data.author.name)
        .version(format!("{:.1}", data.prog.version))
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("set port number")
                .value_parser(value_parser!(u16))
                .default_value("5000")
                .required(false),
        )
        .arg(
            Arg::new("ip")
                .short('i')
                .long("ip")
                .help("set ip address")
                .value_parser(value_parser!(String))
                .default_value("127.0.0.1"),
        );

    let matches = new_app.get_matches();
    let port = *matches.get_one::<u16>("port").unwrap();

    #[allow(unused_comparisons)]
    if !(port >= 1024 && port <= 65535) {
        eprintln!("{}", "Port number must be between 1024..=65535".red());
        std::process::exit(-1);
    }
    let ip = matches.get_one::<String>("ip").unwrap();
    if is_valid_ip(&ip) == false {
        eprintln!("{}", "Invalid ip address".red());
        std::process::exit(-1);
    }
    (port, ip.clone())
}

fn check_folder_handler() -> Result<(), Box<dyn std::error::Error>> {
    let folder_name = crate::FOLDER_HANDLER;
    if !std::path::Path::new(folder_name).is_dir() {
        let index = include_str!("../static/index.html");
        let css = include_str!("../static/style.css");

        std::fs::DirBuilder::new()
            .create(folder_name).expect("An error occurred while trying to create the source folder");

        let mut file_index = fs::File::create(format!("{}/index.html", folder_name)).expect("An error occurred while trying to create index file");
        file_index
            .write(&index.as_bytes())
            .expect("An error occurred while trying to write file");

        let mut style_css = fs::File::create(format!("{}/style.css", folder_name))
            .expect("An error occurred while trying to create index file");
        style_css
            .write(&css.as_bytes())
            .expect("An error occurred while trying to write file");
    }
    Ok(())
}
