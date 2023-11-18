mod api;
mod general;
mod models;

use crate::api::call_request::call_gpt;
use crate::general::command_line::command_line;
use crate::models::command_line::Arguments;
use crate::models::llm::Message;


use clap::Parser;
use general::command_line::PrintCommand;


#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let command_output = command_line(args.command);

    let message1 = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    };
    let message2 = Message {
        role: "user".to_string(),
        content: format!("Take the context and covert it into user friendly instruction with code examples so that user can get a perfect idea of what the tool does.\n\nCONTEXT:{}\n\nANSWER:", command_output),
    };

    let messages = vec![message1, message2];

    
    // print the inital system message
    PrintCommand::SYSResponse.print_ai_response("Generating response...");

    let res = call_gpt(messages).await;
    match res {
        Ok(res_str) => {
            PrintCommand::AIResponse.print_ai_response(&res_str);
        }
        Err(_) => {
            println!("Something went wrong!")
        }
    }
}
