use regex::Regex;
use std::fs::File;
use std::io::{self, Write};
use std::error::Error;

pub fn extract_code(input_str: String) -> Result<String, Box<dyn std::error::Error>>{

    // Define a regular expression pattern to match the string inside backticks
    let re = Regex::new(r"```.*?+\n([^`]+)```").map_err(|_| "Invalid regular expression")?;

    // Use the find method to get the captured text
    if let Some(captures) = re.captures(&input_str) {
        if let Some(inner_string) = captures.get(1) {
            dbg!(inner_string);
            // Extracted string inside backtickslet
            let result = inner_string.as_str();
            Ok(result.to_string())
        } else {
            Err("Not found".into())
        }
    } else {
        Err("Not found".into())
    }
}

pub fn write_to_file(content: &str, filename: &str) -> io::Result<()> {
    if filename.is_empty(){
        std::process::exit(0)
    } else {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_code() {
        let code = extract_code("```bash\nrm -r directory_name ```\nsdf sdf ```sdf\n  adsfs ``` s".to_string());
        dbg!(code);
    }

    #[test]
    fn test_write_to_file() {
        write_to_file("asdfsdf", "test.txt");
    }
}
