extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sign {
    None,
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    id: i32,
    player1: Sign,
    player2: Sign,
}

impl GameState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            id: 0,
            player1: Sign::None,
            player2: Sign::None,
        }
    }

    // this is good for testing
    #[allow(dead_code)]
    pub fn fake_valid_json() -> String {
        r#"{
            "id":3,
            "player1":"None",
            "player2":"Paper"
        }"#
        .to_string()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .expect("Unable to parse RockPaperScissors ClientRequest into json")
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
enum Player {
    Player1,
    Player2,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserAction {
    player: Player,
    sign: Sign,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClientRequest {
    user_action: Option<UserAction>,
    pub game_state: GameState,
}

impl ClientRequest {
    // this is good for testing
    #[allow(dead_code)]
    pub fn fake_valid_json() -> String {
        r#"{
            "game_state":
            {
                "id":3,
                "player1":"None",
                "player2":"Paper"
            },
            "user_action":
            {
                "player":"Player1",
                "sign":"Rock"
            }
        }"#
        .to_string()
    }

    pub fn from_json(json_input: &str) -> Option<ClientRequest> {
        if let Ok(result) = serde_json::from_str(json_input) {
            Some(result)
        } else {
            None
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .expect("Unable to parse RockPaperScissors ClientRequest into json")
    }

    pub fn process_request(&mut self) -> GameState {
        let mut state = GameState::new();
        state.clone_from(&self.game_state);
        if let Some(req) = &self.user_action {
            match req.player {
                Player::Player1 => state.player1 = req.sign,
                Player::Player2 => state.player2 = req.sign,
            }
        }

        state
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateGameRequest {
}

impl CreateGameRequest {
    // this is good for testing
    #[allow(dead_code)]
    pub fn fake_valid_json() -> String {
        r#"{
        }"#
        .to_string()
    }

    pub fn from_json(json_input: &str) -> Option<CreateGameRequest> {
        if let Ok(result) = serde_json::from_str(json_input) {
            Some(result)
        } else {
            None
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .expect("Unable to parse RockPaperScissors CreateGameRequest into json")
    }

    pub fn process_request(&mut self) -> GameState {
        let mut state = GameState::new();

        // TODO Jrutz, generate unique id for game here
        state.id = 3;

        state
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JoinGameRequest 
{
    id:i32
}

impl JoinGameRequest 
{
    // this is good for testing
    #[allow(dead_code)]
    pub fn fake_valid_json() -> String {
        r#"{
            "id":3
        }"#
        .to_string()
    }

    pub fn from_json(json_input: &str) -> Option<CreateGameRequest> {
        if let Ok(result) = serde_json::from_str(json_input) {
            Some(result)
        } else {
            None
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .expect("Unable to parse RockPaperScissors JoinGameRequest into json")
    }

    pub fn process_request(&mut self) -> GameState {
        let mut state = GameState::new();

        // TODO Jrutz, retrieve game to join from database
        state.id = self.id;

        state
    }
}