use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
//take care of the unimproved versions at the bottom

//this is improved version
impl Config{
    pub fn new(mut args:std::env::Args)->Result<Config,&'static str>{ //we use iterators, so dont need to clone as in unimproved
        
        args.next();//first argument is name of the program with path.

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didnot get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didnot get a file name"),
        };
        
        //use enviroment variable to set by using terminal with $env:CASE_INSENSITIVE=1
        //use enviroment variable to unset by using terminal with $env:CASE_INSENSITIVE="" 
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); //is_err returns false if CASE_INSENSITIVE is set to anything
        //println!("{:?}",env::var("CASE_INSENSITIVE"));

        Ok(Config{query,filename,case_sensitive})
    }
}
//take care of the unimproved versions at the bottom
//improved version
pub fn run(config:Config)->Result<(),Box<dyn Error>>{ //If no return value: put "()", for dynamic Error use Box<dyn Error>>

    let contents = fs::read_to_string(config.filename)?; //returns Result<String,Err> //if it is Error, it returns Err
    println!("With the text:\n {:?}",contents);
    println!("Searching result:\n");

    let results = if config.case_sensitive{
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}",line);
    }
    println!("");

    Ok(())
}

//functions generally take &str input instead of &String because &str includes other.
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{//return reference lifetime is equal to 'contents' input reference lifetime

    contents.lines()
        .filter(|line|line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    
    let query = query.to_lowercase(); //return String
    contents.lines()
    .filter(|line|line.to_lowercase().contains(&query))
    .collect()
    
}


//TESTS
//Adding test first before implementing functions
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents ="\
Rust:
safe,productive,fast.
Pick three.
Duct tape";

        assert_eq!(vec!["safe,productive,fast."],search(query,contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "DuCT";
        let contents ="\
Rust:
safe,productive,fast.
Pick three.
Duct tape";

        assert_eq!(vec!["safe,productive,fast.","Duct tape"],search_case_insensitive(query,contents));
    } 
    
}

//4-Another Version:  This is another solution without using iterator
/*
impl Config{
    pub fn new(args:&[String])->Result<Config,&'static str>{ 
        
        if args.len()<3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        //use enviroment variable to set by using terminal with $env:CASE_INSENSITIVE=1
        //use enviroment variable to unset by using terminal with $env:CASE_INSENSITIVE="" 
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); //is_err returns false if CASE_INSENSITIVE is set to anything
        //println!("{:?}",env::var("CASE_INSENSITIVE"));

        Ok(Config{query,filename,case_sensitive})
    }
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{//return reference lifetime is equal to 'contents' input reference lifetime
    let mut results = Vec::new();
    for line in contents.lines(){ //lines method return iterator
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let query = query.to_lowercase(); //return String
    let mut results = Vec::new();

    for line in contents.lines(){ //lines method return iterator
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
    
}

*/


//3-Unimproved Version : Not satisfy "error transmission to main" and not a good way of handling error
/*
pub fn run(config:Config){

    let contents = fs::read_to_string(config.filename).expect("file couldnot be read"); //returns Result<String>
    println!("With the text:\n {:?}",contents);

}
*/

//2-Unimproved Version : Not satisfy "error transmission to main" and not a good way of handling error
/*
    pub fn new(args:&[String])->Config{
        
        if(args.len()<3){
            panic!("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Config{query,filename}
    } 
*/

//1-Unimproved Version : it only satisfies args with 3 argument
/* 
fn parse_config(args:&[String])->(&str,&str){ //if we have a 1 input argument, return values lifetime is the same with input reference lifetime.
    let query = &args[1];
    let filename = &args[2];
    
    (query,filename)
}
*/