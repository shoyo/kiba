use log::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone)]
struct Config {
    pub bind: String,
    pub cbound: usize,
}

const DEFAULT: Config = Config {
    bind: "127.0.0.1".to_string(),
    cbound: 128,
};

fn parse_kv(path: &str) -> HashMap<String, String> {
    if !path.ends_with("kiba.conf") {
        warn!("Was the correct path specified?");
        warn!("The config file should be named \"kiba.conf\"");
        warn!("Attempting to initialize settings with: {}", path);
    }
    let f_open = File::open(path);
    let lines;
    match f_open {
        Ok(file) => {
            let reader = BufReader::new(file);
            lines = reader.lines();
        }
        Err(_) => {
            error!("Could not open specified config file");
            std::process::exit(1);
        }
    }

    let mut kv = HashMap::new();
    for (i, line) in lines.enumerate() {
        let text = line.unwrap();
        if text.starts_with('#') {
            continue;
        }
        let tup: Vec<&str> = text.split_whitespace().collect();
        if tup.len() == 0 {
            continue;
        }
        if tup.len() != 2 {
            error!("Could not parse {}, line {}: \"{}\"", path, i + 1, text);
            std::process::exit(1);
        }
        kv.insert(tup[0].to_string(), tup[1].to_string());
    }
    kv
}

pub fn parse_config(path: Option<&str>) -> Config {
    match path {
        Some(p) => {
            let kv = parse_kv(p);
            let mut config = DEFAULT.clone();
            if let Some(bind) = kv.get("bind") {
                config.bind = bind.to_string();
            }
            if let Some(cbound) = kv.get("cbound") {
                match cbound.parse::<usize>() {
                    Ok(cb) => config.cbound = cb,
                    Err(_) => {
                        error!(
                            "Channel size `cbound` must be a valid integer, found \"{}\"",
                            cbound
                        );
                        std::process::exit(1);
                    }
                }
            }
            config
        }
        None => DEFAULT.clone(),
    }
}
