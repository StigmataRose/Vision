use serde::{Deserialize, Serialize};


pub static api_url: &str = "https://api.sportradar.com/nhl/trial/v7/en/games/2023/REG/schedule.json?api_key=VNiIRM9bxpey8jVjjndD8kIAjgUutIx2gxn9L4J5";



#[derive(Debug, Deserialize, Serialize)]
pub struct League {
    pub id: String,
    pub name: String,
    pub alias: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub id: String,
    pub year: i32,
    #[serde(rename = "type")]
    pub season_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    pub id: String,
    pub name: String,
    pub capacity: i32,
    pub address: String,
    pub city: String,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: String,
    pub time_zone: String,
    pub sr_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Broadcast {
    pub network: String,
    #[serde(rename = "type")]
    pub broadcast_type: String,
    pub locale: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub sr_id: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: String,
    pub status: String,
    pub coverage: String,
    pub scheduled: String,
    pub home_points: Option<i32>,
    pub away_points: Option<i32>,
    pub sr_id: Option<String>,
    pub reference: Option<String>,
    pub venue: Venue,
    pub broadcasts: Option<Vec<Broadcast>>,
    pub home: Team,
    pub away: Team,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameData {
    pub league: League,
    pub season: Season,
    pub games: Vec<Game>,
}
impl GameData {
    fn into_iter(self) -> impl Iterator<Item = Game> {
        self.games.into_iter()
    }
}


pub fn get_nhl_schedule() -> Result<(), reqwest::Error> {

    let response = reqwest::blocking::get(api_url)?;
    //let data: GameData = serde_json::from_str(JSON_DATA).unwrap();

    if response.status().is_success() {
     // Get the body of the response as text
     let body = response.text()?;
     let game_data: GameData = serde_json::from_str(&body).unwrap();

     for game in &game_data.games {
        if game.status == "scheduled" {
            println!("{} vs {} at {}", game.home.name, game.away.name, game.scheduled);
            println!("Broadcasts:");
            if let Some(ref broadcasts) = game.broadcasts {
                for broadcast in broadcasts {
                    println!("Network: {}", broadcast.network);
                    println!("Broadcast Type: {}", broadcast.broadcast_type);
                }
            } else {
                println!("No broadcasts available");
            }
        }
    }
    

    }else{

    }
 Ok(())
    } 

    pub fn get_games() -> Result<Option<GameData>, reqwest::Error> {
        let response = reqwest::blocking::get(api_url)?;
    
        if response.status().is_success() {
            let body = response.text()?;
            let game_data: GameData = serde_json::from_str(&body).unwrap();
            Ok(Some(game_data))
        } else {
            Ok(None)
        }
    }