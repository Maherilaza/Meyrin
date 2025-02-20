use clap::{value_parser, Arg};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

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

pub fn init_args() {
    ascii_init();
    let data: Merge = get_data!();
    let mut new_app = clap::Command::new(data.prog.name)
        .about(data.prog.about)
        .author(data.author.name)
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("set port number")
                .value_parser(value_parser!(u16))
                .default_value("5000")
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
    println!("{}", port);
}

#[macro_export]
macro_rules! get_data {
    () => {{
        let _rjson = match File::open(".config/config.json") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1);
                // to_do : if (_rjson == NULL) ? ret json_online : json_manual
            }
        };

        let read = std::io::BufReader::new(_rjson);

        let data: Merge = match serde_json::from_reader(read) {
            Ok(merge) => merge,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1);
            }
        };
        data
    }};
}

fn ascii_init() {
    print!(
        "{}",
        "         •  
┏┳┓┏┓┓┏┏┓┓┏┓
┛┗┗┗ ┗┫┛ ┗┛┗
      ┛      
"
        .purple()
    );
}
