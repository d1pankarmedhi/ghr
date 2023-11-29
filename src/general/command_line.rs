use std::error::Error;
use std::{process::Command, io::Stdin};
use std::io::{stdout, stdin};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AIResponse,
    SYSResponse
}

impl PrintCommand{
    pub fn print_response(&self, statement: &str) {
        let mut stdout = stdout();
        let statement_color = match self{
            Self::AIResponse => Color::Cyan,
            Self::SYSResponse => Color::Magenta
        };


        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", statement);

        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_help_str(command: String) -> String {
    let output = Command::new(command).arg("--help").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();
    res
}

pub fn user_input(sys_message: &str) -> Result<String, Box<dyn Error>> {
    PrintCommand::SYSResponse.print_response(sys_message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read lines");
    Ok(input.trim().to_string())
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests_get_help_str() {
        let res = get_help_str("docker".to_string());
        dbg!(res);
    }

    #[test]
    fn test_print_ai_response() {
        PrintCommand::SYSResponse.print_response("AI Response below");
        PrintCommand::AIResponse.print_response("How are you");
    }

    
}
