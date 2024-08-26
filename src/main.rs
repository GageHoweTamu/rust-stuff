use std::io::{self, Write};
use std::process::Command;
use std::env;

const BUILTIN_COMMANDS: [&str; 3] = ["cd", "help", "exit"];

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let input = read_line();
        let args: Vec<&str> = input.split_whitespace().collect();

        if !args.is_empty() {
            let status = execute(&args);
            if !status {
                break;
            }
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn execute(args: &[&str]) -> bool {
    match args[0] {
        "cd" => change_directory(args),
        "help" => print_help(),
        "exit" => return false,
        _ => launch_program(args),
    }
    true
}

fn change_directory(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("lsh: expected argument to \"cd\"");
    } else {
        if let Err(e) = env::set_current_dir(args[1]) {
            eprintln!("lsh: {}", e);
        }
    }
}

fn print_help() {
    println!("RSH");
    println!("Type program names and arguments, and hit enter.");
    println!("The following are built in:");
    for cmd in BUILTIN_COMMANDS.iter() {
        println!("  {}", cmd);
    }
    println!("Use the man command for information on other programs.");
}

fn launch_program(args: &[&str]) {
    let output = Command::new(args[0])
        .args(&args[1..])
        .output();

    match output {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => {
            eprintln!("lsh: {}", e);
        }
    }
}