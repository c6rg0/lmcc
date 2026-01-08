// use std::io;
use std::env;
use std::fs;
use regex::Regex;

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
    lexical_analysis();
}

fn lexical_analysis(){

    // ARCH:
    // LDR RResult <memory ref>
    // STR RResult <memory ref>
    // ADD RResult RAgainst <operand>
    // SUB RResult RAgainst <operand>
    // MOV RResult <operand>
    // CMP RAgainst <operand>
    // B <label>
    // B <condition> <label>
    // HALT
    //
    // AND RResult RAgainst <operand>
    // ORR RResult RAgainst <operand>
    // EOR RResult RAgainst <operand>
    // MVN RResult RAgainst <operand>
    // LSL RResult RAgainst <operand>
    // LSR RResult RAgainst <operand>
    //
    // The analyzer will look at the first keyword (ex. ADD) 
    // and then will pull in the expected length of the 
    // instruction until HOLT is met. 

    let contents = fs::read_to_string(file_path)
        .expect("Error (I think anyway lol)");
    
    println!("{contents}");

    let instr = "ADD R2 R1 10 HALT";
    let HALT = false;

    while HALT == false{    
        // Look at first keyword
        // Determine expression length
        // Tokenization occurs
        // Loop
        // HALT

        

    }

}

fn parser(){

}



