use std::env;
use std::collections::HashMap;
use std::fs;
// use std::io;

// Goals:
// Read opcode, operand

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No command provided");
        options();
        return;
    }

    if !args[1].is_empty() {
        if !args[2].is_empty() {
            let f_path: String = args[2].clone();
            match args[1].as_str() {
                "-e" => emulate(f_path),
                "-a" => assemble(f_path),
                _ => {
                    println!("Unkown command: {}", args[1]);
                    options();
                }
            }
        } else {
            let f_path: String = String::new();
            match args[1].as_str() {
                "-e" => emulate(f_path),
                "-a" => assemble(f_path),
                _ => {
                    println!("Unkown command: {}", args[1]);
                    options();
                }
            }
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

// The underscore before "f_path" shows the compiler that 
// the unused variable is intentional.
fn emulate(_f_path: String){ 
    println!("Emulator");
}

fn assemble(_f_path: String){
    println!("Assembler");
    tokenization(_f_path);
}

fn tokenization(f_path: String){
    let f_contents = fs::read_to_string(f_path)
         .expect("Error (I think anyway lol)");

    let bin: String;

    if !f_contents.is_empty(){
        bin = f_contents;
    } 
    else {
        bin = "ADD R2 R1 10 HALT".to_string();
    }

    println!("{bin}");

    // Tokenization occurs

    enum _Token {
        Mnemonic(String),
        AdressStore(String),
        AdressToUse(String),
        ValueToUse(u32),
    }

    // let mut tokens = Vec::new();

    for word in bin.split_whitespace(){
        println!("{word}");
    }

    parser();

}

fn parser() {
    
    // SYMBOL TABLE
    let mut _s_table: HashMap<String, Vec<i32>> = HashMap::new();
    
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
    // The static analyzer will look at the first keyword 
    // (ex. ADD) and then will pull in the expected length 
    // of the instruction until HOLT is met. 
    //

    // loop through each instruction
    // compare against arch


}

