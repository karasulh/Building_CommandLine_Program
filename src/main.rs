use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect(); //args are given arguments when trying to run program by "cargo run pair poem.txt" 
                                                //(btw:args[0]=target\debug\minigrep.exe,args[1]=pair,args[2]=poem.txt)

    //Config::new is a method to parse arguments of args
    let config=Config::new(&args).unwrap_or_else(|err|{//use closure to handle error
        eprintln!("Problem parsing arguments: {}",err); //eprintln:prints the standard error stream(stderr)
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e) = minigrep::run(config){ //it the return is Err, then set it to work by "if let"
        eprintln!("Application error:{}",e); //eprintln:prints the standard error stream(stderr)
        process::exit(1);
    }
}

//to unset environment variable: $env:CASE_INSENSITIVE="" 
//to set environment variable: $env:CASE_INSENSITIVE=1 
//run it by: "cargo run body poem.txt"
//run it by: "cargo run body poem.txt > output.txt" to see results with standard output stream(stdout) in output.txt file
