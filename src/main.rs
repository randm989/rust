mod rps;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let program_name = args[0].clone();
    //let json_input = args[1].clone();
    let json_input = rps::ClientRequest::fake_valid_json();
    eprintln!("Received input of {}", json_input);
    if let Some(mut req) = rps::ClientRequest::from_json(&json_input) 
    {
        req.process_request();
        let return_json = req.game_state.to_json();
        eprintln!("serialized to {}", return_json);
        println!("{}", return_json);
    } 
    if let Some(mut req) = rps::CreateGameRequest::from_json(&json_input)
    {
        let state = req.process_request();
        let return_json = state.to_json();
        eprintln!("serialized to {}", return_json);
        println!("{}", return_json);
    }
    else 
    {
        println!("{{\"error\": \"Invalid json for rock paper scissors object\"}}");
    }
}