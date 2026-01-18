use std::collections::HashMap;
use std::env;
use std::str::FromStr;

#[derive(Debug)]
pub struct Arguments {
    pub search_root: String,
    pub text: String,
    pub output_path: Option<String>,
    pub threads: Option<u32>,
}

impl Arguments {
    pub fn new() -> Result<Self, String> {
        let args: Vec<String> = env::args().skip(1).collect();
        let size = args.len();
        if size < 2 {
            Err("incorrect number of arguments".to_string())
        } else {
            let search_root: String = args[0].clone();
            let text: String = args[1].clone();

            let args_slice = &args[2..];
            let optional_args: HashMap<String, String> = args_slice
                .chunks(2)
                .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
                .collect();

            Ok(Self {
                search_root,
                text,
                output_path: extract(&optional_args, ArgType::OutputPath),
                threads: extract(&optional_args, ArgType::Threads),
            })
        }
    }
}

enum ArgType {
    OutputPath,
    Threads,
}

fn extract<T: FromStr>(_args: &HashMap<String, String>, _key_type: ArgType) -> Option<T> {
    let key = match _key_type {
        ArgType::OutputPath => "--output_path",
        ArgType::Threads => "--threads",
    };

    _args.get(key).and_then(|s| s.parse::<T>().ok())
}
