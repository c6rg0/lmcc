// use std::io;
use std::env;
// use std::fs;

// Goals:
// Read opcode, operand

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        options();
    }

    match args[1].as_str() {
        "-e" => emulate(),
        "-a" => assemble(),
        _ => {
            println!("Unkown command: {}", args[1]);
            options();
        }
    }
}

fn options() {
    println!("!!!Little Man's Computer Collection!!!");
    println!();
    println!("Use example: $ lmcc -e program.bin");
    println!("Options:");
    println!("      -e : Emulate a program,");
    println!("      -a : Assemble a program,");
    println!("          Example: $ lmcc -a program.lmc.");
    println!("      -h : Print this message,");
    println!("      -v : Print version number.");
    println!();
}

fn emulate(){
    println!("Emulator");
}

fn assemble(){
    println!("Assembler");
}

