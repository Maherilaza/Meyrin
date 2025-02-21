use clap::{value_parser, Arg};
use colored::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

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
    let json_data = include_str!(".config/config.json");
    let data: Merge = serde_json::from_str(json_data).expect("Invalid JSON format");
    let mut new_app = clap::Command::new(data.prog.name)
        .about(data.prog.about)
        .author(data.author.name)
        .version(format!("{:.1}", data.prog.version))
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("set port number")
                .value_parser(value_parser!(u16))
                //.default_value("5000")
                .required(true),
        )
        .arg(
            Arg::new("ip")
                .short('i')
                .long("ip")
                .help("set ip address")
                .value_parser(value_parser!(String))
                .default_value("127.0.0.1"),
        );

    if std::env::args().len() == 1 {
        let _ = new_app.print_help();
        std::process::exit(-1);
    }

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