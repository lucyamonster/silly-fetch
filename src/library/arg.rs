use std::env;

pub struct Args {
    pub config_path: Option<String>,
}

pub fn get_args() -> Args {
    let mut args_to_return = Args {
        config_path: None,
    };
    let args: Vec<String> = env::args().collect();
    let mut counter: usize = 1;
    while counter < args.len() {
        let arg = args[counter].as_str();
        match arg {
            "-c" => {
                counter += 1;
                if counter >= args.len() {
                    panic!("no value supplied to arg: {}", arg)
                }
                args_to_return.config_path = Some(args[counter].clone());
            }
            _ => {
                panic!("unknown launch argument: {}", arg);
            }
        }
        counter += 1;
    }

    return args_to_return;
}