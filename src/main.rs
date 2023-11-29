mod api;
mod general;
mod models;


use crate::api::call_request::call_gpt;
use crate::general::command_line::{get_help_str,};
use crate::models::command_line::Arguments;
use crate::models::llm::Message;
use crate::general::process_res::write_to_file;

use clap::Parser;
use general::command_line::{PrintCommand, user_input};
use general::process_res::extract_code;


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    // let args = Arguments::parse();
    let mut input_query = String::new();
    let query = user_input("Hi! What code/command do you want to generate?");
    
    match query {
        Ok(q) => {
            input_query = q
        },
        Err(_) => {}
    }

    let message1 = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant who generates helpful commands for the tool and question asked by the user. Make sure to only generate the code inside ``` ```. Return 'Command not found` if the question is not related to developing.".to_string(),
    };
    let message2 = Message {
        role: "user".to_string(),
        content: input_query,
    };

    let messages = vec![message1, message2];
    
    PrintCommand::SYSResponse.print_response("Generating response...");

    // response 
    let res = call_gpt(messages).await;
    match res {
        Ok(r) => {
            let res_str = extract_code(r)?;
            PrintCommand::AIResponse.print_response(&res_str);

            let save_file = user_input("Enter a filename to save this into a file OR press ENTER")?;
            
            if save_file.is_empty() {
                PrintCommand::AIResponse.print_response("Exiting program!");
                std::process::exit(0);
            } else {
                let _ = write_to_file(&res_str, &save_file);
                PrintCommand::AIResponse.print_response(format!("File {} saved.", &save_file).as_str());
            }
        },
        Err(e) => {
            println!("Something went wrong {}", e)
        },
    }
    Ok(())
}


