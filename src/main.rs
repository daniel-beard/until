extern crate regex;

use regex::Regex;

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::env;

fn run_until(matcher: &str, command: &str, args: &Vec<String>) {
    let regex = Regex::new(matcher).unwrap();

    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let out = BufReader::new(child.stdout.take().unwrap());
    let err = BufReader::new(child.stderr.take().unwrap());

    let thread = thread::spawn(move || {
        err.lines().for_each(|line| {
            eprintln!("{}", line.unwrap());
            io::stderr().flush().ok().expect("Could not flush stderr");
        });
    });

    out.lines().for_each(|line| {
        let line_string = String::from(line.unwrap());
        println!("{}", line_string);
        io::stdout().flush().ok().expect("Could not flush stdout");
        if regex.is_match(&line_string) {
            std::process::exit(0)
        }
    });

    thread.join().unwrap();

    // Return the status, if we didn't match and the 
    // underlying command fails, we want to forward the exit status
    let status = child.wait().unwrap();
    match status.code() {
      Some(code) => std::process::exit(code),
      None       => std::process::exit(0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
      println!("Incorrect number of arguments!");
      println!("Usage: ");
      println!("{} REGEX COMMAND [ARG] [ARG] ...", args[0]);
      std::process::exit(0);
    }
  
    // Run the sub command, exiting when we match the passed in regex
    let m: &str = &args[1];
    let command: &str = &args[2];
    let remaining_args: Vec<String> = args[3..].to_vec();
    run_until(m, command, &remaining_args);
}
