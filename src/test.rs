extern crate dotenv;

use chatgpt::prelude::{gpt_function, ChatGPT, ChatGPTEngine, ModelConfigurationBuilder};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

mod common;
mod server;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let prompt = "read me the contents of `./dummyfile.txt` and write them to a new `dummyfile1.txt`";
    // let prompt = "hey man hows it going";
    let prompt = "call the function to read ./dummyfile.txt and tell me the contents";
    if let Ok(response) = go_get_remote_response(prompt).await {
        println!("response: {}", response);
    }
}

pub async fn go_get_remote_response(prompt: &str) -> Result<String, Box<dyn Error>> {
    // send to remote api
    let openai_api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env file");
    // let client = ChatGPT::new(openai_api_key)?;
    let client = ChatGPT::new_with_config(
        openai_api_key,
        ModelConfigurationBuilder::default()
            .temperature(1.0)
            .engine(ChatGPTEngine::Gpt35Turbo)
            .build()
            .unwrap(),
    )?;
    let mut conversation = client.new_conversation();
    conversation.always_send_functions = true;
    let _ = conversation.add_function(read_file());
    // let _ = conversation.add_function(write_file());

    const MAX_TIMES: u32 = 3;
    let mut loops = 0;
    if let Ok(response) = conversation.send_message_functions(prompt).await {
        let message_choices = response.message_choices;
        while loops < MAX_TIMES {
            if let Some(message_choice) = message_choices.first() {
                let message = message_choice.message.clone();
                println!("{:?}: {}", message.role, message.content);

                // might stop
                if message_choice.finish_reason == "stop" {
                    println!("stop trigger");
                    return Ok(message.content);
                }

                // didnt stop, wants function call
                if let Some(f) = message.function_call {
                    println!("function was called {:?}", f);
                }
            }
            loops += 1;
        }
    }
    Ok("test".to_string())
}

////////////////////////    FUNCTIONS AVAILABLE TO MODEL    ////////////////////////

/// Read the contents of a file and return them as a string
///
/// * filename - The name of the file to read
/// * Returns - The contents of the file as a string
#[gpt_function]
pub async fn read_file(filename: String) -> String {
    match _read_file(filename.clone()).await {
        Ok(contents) => contents,
        Err(_) => format!("Couldnt read file {}. Dont try again.", filename.clone()).to_string(),
    }
}

pub async fn _read_file(filename: String) -> Result<String, io::Error> {
    println!("read_file called: {:?}", filename);
    let mut file = File::open(filename).await?;
    println!("file opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("contents: {}", contents);
    Ok(contents)
}

/// Write the contents of a string to a file
///
/// * filename - The name of the file to be written
/// * contents - String to be written to file
#[gpt_function]
pub async fn write_file(filename: String, contents: String) {
    let mut file = File::create(filename).await?;
    file.write_all(contents.as_bytes()).await?;
}

// pub fn get_model_api() -> Vec<String> {
//     /* example
//     functions = [
//         {
//             "name": "get_current_weather",
//             "description": "Get the current weather in a given location",
//             "parameters": {
//                 "type": "object",
//                 "properties": {
//                     "location": {
//                         "type": "string",
//                         "description": "The city and state, e.g. San Francisco, CA",
//                     },
//                     "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
//                 },
//                 "required": ["location"],
//             },
//         }
//     ]
//      */
// }
