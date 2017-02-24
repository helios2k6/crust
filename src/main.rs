#[allow(dead_code)]

mod library;

use std::process::Command;
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;


use library::lexer;
use library::parser;

fn main() {
    let mut input = String::new();
    print!("Enter the C/C++ file to be converted to Rust : ");
    io::stdout().flush().ok().expect("Buffer cleaning error");
    io::stdin().read_line(&mut input).expect("Unable to read");

    let file = match File::open(String::from("./test_cases/unit_tests/") + input.trim()) {
        Ok(f) => f,
        Err(..) => panic!("Unable to open input source file."),
    };
    // get the reader
    let mut reader = BufReader::new(&file);
    let mut text: String = String::new();
    let size = reader.read_to_string(&mut text).expect("Unable to read file.");

    println!("Input file size : {}bytes ", size);

    let mut tok = lexer::Tokenizer::new(&text);
    print!("Tokenizing");

    let mut out: Vec<String> = Vec::new();
    let tokens = tok.tokenize();
    // tok.tokenize();
    let mut ln = 0;
    for i in &tokens {
        // println!["[ {:?} ], ", i];
        // out = out + &(i.get_token_value()) as &str;
        let mut temp = i.get_token_value();
        if i.get_token_ln() != ln {
            temp = "\n".to_string() + &temp[..];
        }
        ln = i.get_token_ln();
        out.push(temp);
    }
    
    for _ in 0..7 {
       print!(".");
       io::stdout().flush().ok().expect("Buffer cleaning error");
       std::thread::sleep(std::time::Duration::from_millis(500));
       
    }

    //    file.write_all(output.as_bytes()).expect("Unable to write to file");
    
    println!("\t:DONE");
    print!("Invoking Parser .");

    for _ in 0..7 {
       print!(".");
       io::stdout().flush().ok().expect("Buffer cleaning error");
       std::thread::sleep(std::time::Duration::from_millis(600));
       
    }
    let s = parser::parse_program(&tokens);
    let mut o: String = String::new();
    for i in s {
        o = o + " ";
        o = o + &i[..];
    }

    println!("\t:DONE");

    //write to a output file
   
   let mut fname1 = String::new();
   for c in input.chars() {
        if c == '.' { break; }
        fname1.push(c);
   }
   fname1=fname1+".rs";
   let fname = "./test_cases/unit_tests/".to_string()+ &fname1[..];
    let mut file = File::create(&fname[..])
        .expect("Unable to open file to write");
    file.write_all(o.as_bytes()).expect("Unable to write to file");

    Command::new("rustfmt")
        .arg(&fname[..])
        .spawn()
        .expect("Failed to format the translated code");
    println!("Rust equivalent of source of `{}` is generated successfully, View the rust code in file : `{}`",
             input.trim(),fname1);
}
