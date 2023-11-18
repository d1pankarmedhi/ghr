use std::process::Command;
use std::io::stdout;

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
    pub fn print_ai_response(&self, statement: &str) {
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

pub fn command_line(command: String) -> String {
    let output = Command::new(command).arg("--help").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests_command_line() {
        let res = command_line("docker".to_string());
        dbg!(res);
    }

    #[test]
    fn test_print_ai_response() {
        PrintCommand::SYSResponse.print_ai_response("AI Response below");
        PrintCommand::AIResponse.print_ai_response("How are you");
    }
}
