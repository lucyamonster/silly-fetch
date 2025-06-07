use crate::library::{arg::get_args, file::get_file};

pub struct Config {
    pub layout: Vec<String>,
    pub clear: bool,
} impl Config {
    pub fn default() -> Self {
        Config { 
            layout: vec![
                "userHostname".to_string(),
                "divider".to_string(),
                "os".to_string(),
                "host".to_string(),
                "kernel".to_string(),
                "uptime".to_string()
            ],
            clear: false
        }
    }
}

pub fn config() -> Config {
    let args = get_args();
    let config_to_return: Config;
    if args.config_path != None {
        config_to_return = read_config_file(&args.config_path.unwrap());
    } else {
        config_to_return = Config::default();
    }
    config_to_return
}

fn read_config_file(path: &String) -> Config {
    let config_file = get_file(path);
    let config_file = json::parse(&config_file).unwrap();
    
    let mut config_to_return = Config::default();
    
    // layout: What thing goes where
    if config_file["layout"].len() > 0 {
        config_to_return.layout = vec![];
    }
    for index in 0..config_file["layout"].len() {
        let item = config_file["layout"][index].as_str().expect("no `layout` in config file").to_string();
        config_to_return.layout.push(item);
    }

    // clear: should the term be cleared before printing fetch
    config_to_return.clear = config_file["clearTerm"].as_bool().expect("no `layout` in config file");

    config_to_return
}