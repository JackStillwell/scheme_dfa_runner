use std::fs;

mod cli_utils;
mod process_dfa_input;

fn main() {
    loop {
        println!(
            "Would you like to:\n [1] Load a File\n [2] Enter Data on the Command Line\n [3] Quit"
        );

        let choice_prompt = String::from("Your Choice: ");

        let choice = match cli_utils::my_prompt_response(choice_prompt) {
            Ok(s) => s,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result: Result<bool, String> = match choice {
            1 => file_load(),
            2 => cl_input(),
            3 => break,
            _ => {
                println!("Invalid Selection");
                continue;
            }
        };

        let result: bool = match result {
            Ok(b) => b,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        match result {
            true => println!("The input is a member of the language of the DFA."),
            false => println!("The input is not a member of the language of the DFA."),
        }
    }
}

fn file_load() -> Result<bool, String> {
    loop {
        let filename_prompt = String::from("Please enter the filename (or QUIT to exit): ");

        let filename = match cli_utils::my_prompt_response(filename_prompt) {
            Ok(s) => {
                if s.trim() == "QUIT" {
                    return Err("User Interrupt".to_string());
                } else {
                    s
                }
            }
            Err(error) => {
                println!("cli_utils error: {}", error);
                continue;
            }
        };

        let file_contents: String = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(error) => {
                println!("File Read Error: {}", error);
                continue;
            }
        };

        let lines: Vec<&str> = file_contents.split("\n").collect();

        let input: String = match lines.get(0) {
            Some(s) => s.to_string(),
            None => {
                println!("Formatting error in file");
                continue;
            }
        };

        let dfa: String = match lines.get(1) {
            Some(s) => s.to_string(),
            None => {
                println!("Formatting error in file");
                continue;
            }
        };

        return process_dfa_input::process_schema(input, dfa);
    }
}

fn cl_input() -> Result<bool, String> {
    let input_prompt = String::from("Please enter your input string: ");
    let dfa_prompt = String::from("Please enter your DFA: ");

    let input: String = match cli_utils::my_prompt_response(input_prompt) {
        Ok(s) => s.trim().to_string(),
        Err(error) => return Err(format!("cli_utils error: {}", error)),
    };

    let dfa: String = match cli_utils::my_prompt_response(dfa_prompt) {
        Ok(s) => s.trim().to_string(),
        Err(error) => return Err(format!("cli_utils error: {}", error)),
    };

    process_dfa_input::process_schema(input, dfa)
}
