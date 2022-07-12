use std::{fs, process};

pub fn parse_args(args: Vec<String>) -> String {
    let tab = "    ";
    let mut iter = 0;
    let mut file_loc = "";

    for arg in &args {
        match arg.as_ref() {
            "-help" => { println!("River help\n{0}-bin\n{0}{0}Gives you relitive path to binary location", tab); process::exit(0); }
            "-file" | "-f" => { file_loc = args[iter+1].as_str(); }
            "-bin" => { println!("Binary is located at:\n{}", &args[0]); }
            _ => {}
        }
        iter += 1;
    }
    if str::is_empty(file_loc) {
        println!("You must provide a file location");
        process::exit(1);
    }

    return fs::read_to_string(file_loc).expect("Error read in file");
}