use serde::{Deserialize, Serialize};


pub static api_url: &str = "https://api.sportradar.com/nhl/trial/v7/en/games/2023/REG/schedule.json?api_key=VNiIRM9bxpey8jVjjndD8kIAjgUutIx2gxn9L4J5";



#[derive(Debug, Deserialize, Serialize)]
struct League {
    id: String,
    name: String,
    alias: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Season {
    id: String,
    year: i32,
    #[serde(rename = "type")]
    season_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Venue {
    id: String,
    name: String,
    capacity: i32,
    address: String,
    city: String,
    state: Option<String>,
    zip: Option<String>,
    country: String,
    time_zone: String,
    sr_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Broadcast {
    network: String,
    #[serde(rename = "type")]
    broadcast_type: String,
    locale: Option<String>,
    channel: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Team {
    id: String,
    name: String,
    alias: String,
    sr_id: Option<String>,
    reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Game {
    id: String,
    status: String,
    coverage: String,
    scheduled: String,
    home_points: Option<i32>,
    away_points: Option<i32>,
    sr_id: Option<String>,
    reference: Option<String>,
    venue: Venue,
    broadcasts: Option<Vec<Broadcast>>,
    home: Team,
    away: Team,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameData {
    league: League,
    season: Season,
    games: Vec<Game>,
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

    static JSON_DATA: &'static str = r#"{
        "league": {
          "id": "fd560107-a85b-4388-ab0d-655ad022aff7",
          "name": "NHL",
          "alias": "NHL"
        },
        "season": {
          "id": "9a5fb1fb-5cd9-4b9b-a957-c1dc5eb1a0f3",
          "year": 2023,
          "type": "REG"
        },
        "games": [
          {
            "id": "6c1cdff0-cb14-4ad1-9c91-9463c7c703ac",
            "status": "closed",
            "coverage": "full",
            "scheduled": "2023-10-10T21:30:00Z",
            "home_points": 5,
            "away_points": 3,
            "sr_id": "sr:match:41970907",
            "reference": "20001",
            "venue": {
              "id": "05aa49b2-f72d-4d42-ab30-f219d32ed97b",
              "name": "Amalie Arena",
              "capacity": 19092,
              "address": "401 Channelside Drive",
              "city": "Tampa",
              "state": "FL",
              "zip": "33602",
              "country": "USA",
              "time_zone": "US/Eastern",
              "sr_id": "sr:venue:6036"
            },
            "broadcasts": [
              {
                "network": "ESPN",
                "type": "TV",
                "locale": "National",
                "channel": "206"
              },
              {
                "network": "ESPN+",
                "type": "Internet",
                "locale": "National"
              }
            ],
            "home": {
              "id": "4417d3cb-0f24-11e2-8525-18a905767e44",
              "name": "Tampa Bay Lightning",
              "alias": "TB",
              "sr_id": "sr:team:3694",
              "reference": "14"
            },
            "away": {
              "id": "441643b7-0f24-11e2-8525-18a905767e44",
              "name": "Nashville Predators",
              "alias": "NSH",
              "sr_id": "sr:team:3705",
              "reference": "18"
            }
          }
        ]
      }"#;