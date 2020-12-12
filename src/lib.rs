use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Debug)]
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

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n\n{}", contents);

    Ok(())
}


// The return value is a collection of references of str slices; therefore, the references within the collection must have the same lifetime as the input contents.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
	if line.contains(query) {
	    results.push(line);
	}
    }

    results
}

// disabling snake case for more legible test > names
#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
	Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn multiple_results() {
	let query = "nice";
	let contents = "\
You can have me if you want me
All I need is a little street money
I need a place for all the shit in my closet
I need a place to put all my electronics
Ain't nice
I ain't nice
You ain't that nice but you got a nice face
Hope I can fit all my shit at your place
Got a collection of vintage calculators
If you don't like it then babe I'll see you later";

	let expected = vec!["Ain't nice", "I ain't nice", "You ain't that nice but you got a nice face"];

	assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn no_results() {
	let query = "gazebo";
	let contents = "\
A vision I had in my sleep last night - as distinguished from a dream which is mere sorting and cataloguing of the day's events by the subconscious.
This was a vision, fresh and clear as a mountain stream - the mind revealing itself to itself.
In my vision, I was on the veranda of a vast estate, a palazzo of some fantastic proportion.";

	// well, that's pretty ugly looking, isn't it? Vec::<String>::new() ...
	assert_eq!(Vec::<String>::new(), search(query, contents));
    }

    #[test]
    fn fewerThan3Args_returnsErr() {
        let not_enough_args: Vec<String> = vec![String::from("test")];

        if let Ok(config) = Config::new(&not_enough_args) {
            panic!("this should not have been Ok(()): {:?}", config);
        }
    }

    #[test]
    fn threeArgs_returnsConfig() {
        let arg1 = String::from("arg1");
        let arg2 = String::from("arg2");
        let arg3 = String::from("arg3");

        let just_right_args: Vec<String> = vec![arg1, arg2, arg3];

        if let Err(e) = Config::new(&just_right_args) {
            panic!("this should have been Ok(()): {}", e);
        }
    }

    #[test]
    fn newConfig_usesSecondAndThirdArgs() {
        let arg1 = String::from("arg1");
        let arg2 = String::from("arg2");
        let arg3 = String::from("arg3");

        let just_right_args: Vec<String> = vec![arg1, arg2.clone(), arg3.clone()];

        assert_eq!(
            Config {
                query: arg2,
                filename: arg3
            },
            Config::new(&just_right_args).unwrap()
        );
    }
}
