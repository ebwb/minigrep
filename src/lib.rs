use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
	if args.len() < 3 {
	    return Err("not enough arguments");
	}

	let query = args[1].clone();
	let filename = args[2].clone();
	
	Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n\n{}", contents);

    Ok(())
}

// disabling snake case for more legible tes names
#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {

    }

    #[test]
    fn fewerThan3Args_returnsErr() {
	let not_enough_args : Vec<String> = vec![String::from("test")];
	
	if let Ok(config) = Config::new(&not_enough_args) {
	    panic!("this should not have been Ok(()): {:?}", config);
	}
    }

    #[test]
    fn threeArgs_returnsConfig() {
	let arg1 = String::from("arg1");
	let arg2 = String::from("arg2");
	let arg3 = String::from("arg3");

	let just_right_args : Vec<String> = vec![arg1, arg2, arg3];

	if let Err(e) = Config::new(&just_right_args) {
	    panic!("this should have been Ok(()): {}", e);
	}
    }

    #[test]
    fn newConfig_usesSecondAndThirdArgs() {
	let arg1 = String::from("arg1");
	let arg2 = String::from("arg2");
	let arg3 = String::from("arg3");

	let just_right_args : Vec<String> = vec![arg1, arg2.clone(), arg3.clone()];

	assert_eq!(
	    Config {query: arg2, filename: arg3},
	    Config::new(&just_right_args).unwrap());
    }
}
