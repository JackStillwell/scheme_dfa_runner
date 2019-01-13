use std::io;
use std::io::Write;

fn my_read_line() -> Result<String, String> {
    let mut temp = String::new();
    match io::stdin().read_line(&mut temp) {
        Ok(_) => return Ok(temp),
        Err(error) => return Err(error.to_string()),
    };
}

fn my_flush() -> Result<bool, String> {
    match io::stdout().flush() {
        Ok(_) => return Ok(true),
        Err(error) => return Err(error.to_string()),
    }
}

fn my_print(s: String) -> Result<bool, String> {
    print!("{}", s);
    my_flush()
}

pub fn my_prompt_response(s: String) -> Result<String, String> {
    match my_print(s) {
        Ok(_) => {}
        Err(error) => return Err(error),
    }
    my_read_line()
}
