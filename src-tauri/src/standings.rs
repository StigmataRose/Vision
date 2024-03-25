use serde::{Deserialize, Serialize};
use reqwest::{Error, Client};
use reqwest::get;
use serde_json::json;

pub static api_url: &str = "https://api.sportradar.com/nhl/trial/v7/en/seasons/2023/REG/standings.json?api_key=VNiIRM9bxpey8jVjjndD8kIAjgUutIx2gxn9L4J5";


#[derive(Debug, Deserialize)]
pub struct NHLData {
    league: League,
    season: Seasons,
    conferences: Vec<Conference>,
}


// Define your structs here
#[derive(Debug, Deserialize)]
pub struct League {
    pub id: String,
    pub name: String,
    pub alias: String,
}

#[derive(Debug, Deserialize)]
pub struct Seasons {
    pub id: String,
    pub year: i32,
}

#[derive(Debug, Deserialize)]
pub struct Conference {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub divisions: Vec<Division>,
}

// Iterator implementation for Conference divisions
pub struct DivisionIterator<'a> {
    divisions: &'a Vec<Division>,
    index: usize,
}

impl<'a> Iterator for DivisionIterator<'a> {
    type Item = &'a Division;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.divisions.len() {
            let division = &self.divisions[self.index];
            self.index += 1;
            Some(division)
        } else {
            None
        }
    }
}

impl Conference {
    // Function to create a new iterator over divisions
    pub fn iter_divisions(&self) -> DivisionIterator {
        DivisionIterator {
            divisions: &self.divisions,
            index: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Division {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub teams: Vec<Team>,
}

// Iterator implementation for Division teams
pub struct TeamIterator<'a> {
    teams: &'a Vec<Team>,
    index: usize,
}

impl<'a> Iterator for TeamIterator<'a> {
    type Item = &'a Team;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.teams.len() {
            let team = &self.teams[self.index];
            self.index += 1;
            Some(team)
        } else {
            None
        }
    }
}

impl Division {
    // Function to create a new iterator over teams
    pub fn iter_teams(&self) -> TeamIterator {
        TeamIterator {
            teams: &self.teams,
            index: 0,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub market: String,
    pub sr_id: String,
    pub games_played: i32,
    pub wins: i32,
    pub losses: i32,
    pub overtime_losses: i32,
    pub win_pct: f64,
    pub points: i32,
    pub shootout_wins: i32,
    pub shootout_losses: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub goal_diff: i32,
    pub powerplays: i32,
    pub powerplay_goals: i32,
    pub powerplay_pct: f64,
    pub powerplays_against: i32,
    pub powerplay_goals_against: i32,
    pub penalty_killing_pct: f64,
    pub points_pct: f64,
    pub points_per_game: f64,
    pub regulation_wins: i32,
    pub streak: Streak,
    pub records: Vec<Record>,
    pub rank: Rank,
    pub calc_rank: CalcRank,
}

#[derive(Debug, Deserialize)]
pub struct Streak {
    pub kind: String,
    pub length: i32,
}

// Iterator implementation for Team records
pub struct RecordIterator<'a> {
    records: &'a Vec<Record>,
    index: usize,
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = &'a Record;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.records.len() {
            let record = &self.records[self.index];
            self.index += 1;
            Some(record)
        } else {
            None
        }
    }
}

impl Team {
    // Function to create a new iterator over records
    pub fn iter_records(&self) -> RecordIterator {
        RecordIterator {
            records: &self.records,
            index: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub record_type: String,
    pub wins: i32,
    pub losses: i32,
    pub overtime_losses: i32,
    pub win_pct: f64,
    pub goals_for: i32,
    pub goals_against: i32,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct Rank {
    pub division: i32,
    pub conference: i32,
    pub wildcard: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CalcRank {
    pub div_rank: i32,
    pub conf_rank: i32,
    pub conf_tiebreak: Option<String>,
}


// Iterator implementation for NHLData
pub struct ConferenceIterator<'a> {
    conferences: &'a Vec<Conference>,
    index: usize,
}

impl<'a> Iterator for ConferenceIterator<'a> {
    type Item = &'a Conference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.conferences.len() {
            let conference = &self.conferences[self.index];
            self.index += 1;
            Some(conference)
        } else {
            None
        }
    }
}

impl NHLData {
    // Function to create a new iterator over conferences
    pub fn iter_conferences(&self) -> ConferenceIterator {
        ConferenceIterator {
            conferences: &self.conferences,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Standings_Team {
    name: String,
    rank: i32,
    wins: i32,
    losses: i32,
}
impl Standings_Team {
    fn new(name: &str, rank: i32, wins: i32, losses: i32) -> Self {
        Standings_Team {
            name: name.to_string(),
            rank,
            wins,
            losses,
        }
    }
}

fn order_teams_by_rank(mut Standings_Team: Vec<Standings_Team>) -> Vec<Standings_Team> {
    Standings_Team.sort_by_key(|Standings_Team| Standings_Team.rank);
    Standings_Team
}

pub fn get_nhl() -> Result<(), reqwest::Error> {

   let response = reqwest::blocking::get(api_url)?;


   if response.status().is_success() {
    // Get the body of the response as text
    let body = response.text()?;
    let nhl_data: NHLData = serde_json::from_str(&body).unwrap();


    let mut west_teams: Vec<Standings_Team> = Vec::new();
    let mut east_teams: Vec<Standings_Team> = Vec::new();

    // get west teams
    for division in &nhl_data.conferences[0].divisions {
        for team in division.iter_teams() {
            west_teams.push(Standings_Team::new(team.name.as_str(), team.rank.conference, team.wins, team.losses));
        }

    }

    west_teams = order_teams_by_rank(west_teams);

    // get east teams
    for division in &nhl_data.conferences[1].divisions {
        for team in division.iter_teams() {
            east_teams.push(Standings_Team::new(team.name.as_str(), team.rank.conference, team.wins, team.losses));
        }

    }

    east_teams = order_teams_by_rank(east_teams);

   
    // Output West teams
println!("West Conference Standings:");
for team in &west_teams {
    println!("{}: {} {}-{}", team.rank, team.name, team.wins, team.losses);
}

println!();
// Output East teams
println!("East Conference Standings:");
for team in &east_teams {
    println!("{}: {} {}-{}", team.rank, team.name, team.wins, team.losses);
}

  
} else {

}

//     // Check if the response was successful (status code 200)
//    if response.status().is_success() {
//        // Get the body of the response as text
//        let body = response.text()?;
//        let nhl_data: NHLData = serde_json::from_str(&body).unwrap();

//        // Iterate over conferences
//         for conference in nhl_data.iter_conferences() {
//             // Do something with each conference
//             println!("Conference name: {}", conference.name); // Assuming Conference has a 'name' field

//              for division in conference.iter_divisions() {
//                  // Do something with each division
//                  println!("Division name: {}", division.name); // Assuming Division has a 'name' field
//                     for team in division.iter_teams() {
//                      // Do something with each team
//                             println!("Team name: {}", team.name); // Assuming Team has a 'name' field
//                               println!("Rank: {}", team.rank.conference);
//                                  println!("Wins: {}", team.wins);
//                                     println!("Losses: {}", team.losses);
//                                      println!();
//                   }
//               }

//          }
//       // println!("Response body: {}", body);

//        // You can parse the body as needed, depending on the content
//    } else {
//        // Handle unsuccessful response
//        println!("Request was unsuccessful: {}", response.status());
//    }

//let data: NHLData = serde_json::from_str(JSON_DATA).unwrap();

 
// if let Some(data) = data.get("conferences") {
//     println!("field = {:?}", data);
// } else {
//     println!("field is missing");
// }

   Ok(())
}

#[derive(Debug)]
struct TeamID {
    name: String,
    id: String,
}

impl TeamID {
    fn new(name: &str, id: &str) -> Self {
        TeamID {
            name: name.to_string(),
            id: id.to_string(),
        }
    }
}


pub fn get_nhl_teams() -> Result<(), reqwest::Error> {

    let response = reqwest::blocking::get(api_url)?;
 
    let mut teams: Vec<TeamID> = Vec::new(); // Create an empty vector of TeamID structs

     // Check if the response was successful (status code 200)
    if response.status().is_success() {
        // Get the body of the response as text
        let body = response.text()?;
        let nhl_data: NHLData = serde_json::from_str(&body).unwrap();
 
        // Iterate over conferences
         for conference in nhl_data.iter_conferences() {
         
              for division in conference.iter_divisions() {
           
                     for team in division.iter_teams() {
                      // Do something with each team
                             println!("{} {}", team.name, team.id); // Assuming Team has a 'name' field
                           teams.push(TeamID::new(team.name.as_str(),team.id.as_str()));
                
                   }
               }
 
          }

           // Sort the teams alphabetically by name
    teams.sort_by(|a, b| a.name.cmp(&b.name));

    // Print the sorted teams
    for team in &teams {
        println!("{} {}", team.name, team.id);
    }
    
       // println!("Response body: {}", body);
 
        // You can parse the body as needed, depending on the content
    } else {
        // Handle unsuccessful response
        println!("Request was unsuccessful: {}", response.status());
    }
 
 
    Ok(())
 }
 
/*
let data: NHLData = serde_json::from_str(JSON_DATA).unwrap();
 
  
 if let Some(data) = data.get("conferences") {
     println!("field = {:?}", data);
 } else {
     println!("field is missing");
 }
 
*/
static JSON_DATA: &'static str = r#"{
    "league": {
        "id": "fd560107-a85b-4388-ab0d-655ad022aff7",
        "name": "NHL",
        "alias": "NHL"
    },
    "season": {
        "id": "9a5fb1fb-5cd9-4b9b-a957-c1dc5eb1a0f3",
        "year": 2023
    },
    "conferences": [
        {
            "id": "64901512-9ca9-4bea-aa80-16dbcbdae230",
            "name": "WESTERN CONFERENCE",
            "alias": "WESTERN",
            "divisions": [
                {
                    "id": "17101b65-e8b9-4cda-a963-eea874aed81f",
                    "name": "Pacific",
                    "alias": "PACIFIC",
                    "teams": [
                        {
                            "id": "4415b0a7-0f24-11e2-8525-18a905767e44",
                            "name": "Canucks",
                            "market": "Vancouver",
                            "sr_id": "sr:team:3692",
                            "games_played": 71,
                            "wins": 45,
                            "losses": 18,
                            "overtime_losses": 8,
                            "win_pct": 0.634,
                            "points": 98,
                            "shootout_wins": 0,
                            "shootout_losses": 2,
                            "goals_for": 249,
                            "goals_against": 189,
                            "goal_diff": 60,
                            "powerplays": 225,
                            "powerplay_goals": 50,
                            "powerplay_pct": 22.2,
                            "powerplays_against": 215,
                            "powerplay_goals_against": 44,
                            "penalty_killing_pct": 79.5,
                            "points_pct": 74.6,
                            "points_per_game": 1.4,
                            "regulation_wins": 45,
                            "streak": {
                                "kind": "win",
                                "length": 3
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 12,
                                    "losses": 3,
                                    "overtime_losses": 1,
                                    "win_pct": 0.75,
                                    "goals_for": 58,
                                    "goals_against": 38,
                                    "points": 25
                                },
                                {
                                    "record_type": "central",
                                    "wins": 11,
                                    "losses": 5,
                                    "overtime_losses": 4,
                                    "win_pct": 0.55,
                                    "goals_for": 62,
                                    "goals_against": 50,
                                    "points": 26
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 24,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.615,
                                    "goals_for": 135,
                                    "goals_against": 99,
                                    "points": 52
                                },
                                {
                                    "record_type": "division",
                                    "wins": 13,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.684,
                                    "goals_for": 73,
                                    "goals_against": 49,
                                    "points": 26
                                },
                                {
                                    "record_type": "east",
                                    "wins": 21,
                                    "losses": 7,
                                    "overtime_losses": 4,
                                    "win_pct": 0.656,
                                    "goals_for": 114,
                                    "goals_against": 90,
                                    "points": 46
                                },
                                {
                                    "record_type": "home",
                                    "wins": 24,
                                    "losses": 7,
                                    "overtime_losses": 4,
                                    "win_pct": 0.686,
                                    "goals_for": 127,
                                    "goals_against": 83,
                                    "points": 52
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 7,
                                    "losses": 2,
                                    "overtime_losses": 1,
                                    "win_pct": 0.7,
                                    "goals_for": 28,
                                    "goals_against": 19,
                                    "points": 15
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 5,
                                    "losses": 3,
                                    "overtime_losses": 2,
                                    "win_pct": 0.5,
                                    "goals_for": 29,
                                    "goals_against": 26,
                                    "points": 12
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 5,
                                    "losses": 4,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 27,
                                    "goals_against": 33,
                                    "points": 11
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 3,
                                    "win_pct": 0.563,
                                    "goals_for": 56,
                                    "goals_against": 52,
                                    "points": 21
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 13,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.684,
                                    "goals_for": 73,
                                    "goals_against": 49,
                                    "points": 26
                                },
                                {
                                    "record_type": "road",
                                    "wins": 21,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.583,
                                    "goals_for": 122,
                                    "goals_against": 106,
                                    "points": 46
                                },
                                {
                                    "record_type": "west",
                                    "wins": 24,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.615,
                                    "goals_for": 135,
                                    "goals_against": 99,
                                    "points": 52
                                }
                            ],
                            "rank": {
                                "division": 1,
                                "conference": 1
                            },
                            "calc_rank": {
                                "div_rank": 1,
                                "conf_rank": 1
                            }
                        },
                        {
                            "id": "4415ea6c-0f24-11e2-8525-18a905767e44",
                            "name": "Oilers",
                            "market": "Edmonton",
                            "sr_id": "sr:team:3686",
                            "games_played": 68,
                            "wins": 42,
                            "losses": 22,
                            "overtime_losses": 4,
                            "win_pct": 0.618,
                            "points": 88,
                            "shootout_wins": 2,
                            "shootout_losses": 1,
                            "goals_for": 245,
                            "goals_against": 196,
                            "goal_diff": 49,
                            "powerplays": 204,
                            "powerplay_goals": 55,
                            "powerplay_pct": 27.0,
                            "powerplays_against": 218,
                            "powerplay_goals_against": 43,
                            "penalty_killing_pct": 80.3,
                            "points_pct": 67.6,
                            "points_per_game": 1.3,
                            "regulation_wins": 40,
                            "streak": {
                                "kind": "loss",
                                "length": 1
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 8,
                                    "losses": 5,
                                    "overtime_losses": 2,
                                    "win_pct": 0.533,
                                    "goals_for": 55,
                                    "goals_against": 54,
                                    "points": 18
                                },
                                {
                                    "record_type": "central",
                                    "wins": 10,
                                    "losses": 5,
                                    "overtime_losses": 2,
                                    "win_pct": 0.588,
                                    "goals_for": 57,
                                    "goals_against": 48,
                                    "points": 22
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 23,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.622,
                                    "goals_for": 128,
                                    "goals_against": 107,
                                    "points": 48
                                },
                                {
                                    "record_type": "division",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 0,
                                    "win_pct": 0.65,
                                    "goals_for": 71,
                                    "goals_against": 59,
                                    "points": 26
                                },
                                {
                                    "record_type": "east",
                                    "wins": 19,
                                    "losses": 10,
                                    "overtime_losses": 2,
                                    "win_pct": 0.613,
                                    "goals_for": 117,
                                    "goals_against": 89,
                                    "points": 40
                                },
                                {
                                    "record_type": "home",
                                    "wins": 23,
                                    "losses": 8,
                                    "overtime_losses": 3,
                                    "win_pct": 0.676,
                                    "goals_for": 137,
                                    "goals_against": 93,
                                    "points": 49
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 6,
                                    "losses": 2,
                                    "overtime_losses": 2,
                                    "win_pct": 0.6,
                                    "goals_for": 39,
                                    "goals_against": 25,
                                    "points": 14
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 6,
                                    "losses": 2,
                                    "overtime_losses": 2,
                                    "win_pct": 0.6,
                                    "goals_for": 43,
                                    "goals_against": 31,
                                    "points": 14
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 5,
                                    "losses": 4,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 28,
                                    "goals_against": 31,
                                    "points": 11
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 11,
                                    "losses": 5,
                                    "overtime_losses": 0,
                                    "win_pct": 0.688,
                                    "goals_for": 62,
                                    "goals_against": 35,
                                    "points": 22
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 0,
                                    "win_pct": 0.65,
                                    "goals_for": 71,
                                    "goals_against": 59,
                                    "points": 26
                                },
                                {
                                    "record_type": "road",
                                    "wins": 19,
                                    "losses": 14,
                                    "overtime_losses": 1,
                                    "win_pct": 0.559,
                                    "goals_for": 108,
                                    "goals_against": 103,
                                    "points": 39
                                },
                                {
                                    "record_type": "west",
                                    "wins": 23,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.622,
                                    "goals_for": 128,
                                    "goals_against": 107,
                                    "points": 48
                                }
                            ],
                            "rank": {
                                "division": 2,
                                "conference": 5
                            },
                            "calc_rank": {
                                "div_rank": 2,
                                "conf_rank": 5,
                                "conf_tiebreak": "fewest_games_played"
                            }
                        },
                        {
                            "id": "44151f7a-0f24-11e2-8525-18a905767e44",
                            "name": "Kings",
                            "market": "Los Angeles",
                            "sr_id": "sr:team:3688",
                            "games_played": 70,
                            "wins": 37,
                            "losses": 22,
                            "overtime_losses": 11,
                            "win_pct": 0.529,
                            "points": 85,
                            "shootout_wins": 2,
                            "shootout_losses": 5,
                            "goals_for": 220,
                            "goals_against": 183,
                            "goal_diff": 37,
                            "powerplays": 211,
                            "powerplay_goals": 47,
                            "powerplay_pct": 22.3,
                            "powerplays_against": 222,
                            "powerplay_goals_against": 32,
                            "penalty_killing_pct": 85.6,
                            "points_pct": 68.6,
                            "points_per_game": 1.2,
                            "regulation_wins": 35,
                            "streak": {
                                "kind": "win",
                                "length": 3
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 8,
                                    "losses": 5,
                                    "overtime_losses": 3,
                                    "win_pct": 0.5,
                                    "goals_for": 45,
                                    "goals_against": 48,
                                    "points": 19
                                },
                                {
                                    "record_type": "central",
                                    "wins": 11,
                                    "losses": 9,
                                    "overtime_losses": 1,
                                    "win_pct": 0.524,
                                    "goals_for": 71,
                                    "goals_against": 59,
                                    "points": 23
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 20,
                                    "losses": 13,
                                    "overtime_losses": 5,
                                    "win_pct": 0.526,
                                    "goals_for": 125,
                                    "goals_against": 98,
                                    "points": 45
                                },
                                {
                                    "record_type": "division",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 4,
                                    "win_pct": 0.529,
                                    "goals_for": 54,
                                    "goals_against": 39,
                                    "points": 22
                                },
                                {
                                    "record_type": "east",
                                    "wins": 17,
                                    "losses": 9,
                                    "overtime_losses": 6,
                                    "win_pct": 0.531,
                                    "goals_for": 95,
                                    "goals_against": 85,
                                    "points": 40
                                },
                                {
                                    "record_type": "home",
                                    "wins": 17,
                                    "losses": 11,
                                    "overtime_losses": 7,
                                    "win_pct": 0.486,
                                    "goals_for": 109,
                                    "goals_against": 90,
                                    "points": 41
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 6,
                                    "losses": 3,
                                    "overtime_losses": 1,
                                    "win_pct": 0.6,
                                    "goals_for": 32,
                                    "goals_against": 21,
                                    "points": 13
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 7,
                                    "losses": 2,
                                    "overtime_losses": 1,
                                    "win_pct": 0.7,
                                    "goals_for": 34,
                                    "goals_against": 21,
                                    "points": 15
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 5,
                                    "losses": 5,
                                    "overtime_losses": 0,
                                    "win_pct": 0.5,
                                    "goals_for": 25,
                                    "goals_against": 29,
                                    "points": 10
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 3,
                                    "win_pct": 0.563,
                                    "goals_for": 50,
                                    "goals_against": 37,
                                    "points": 21
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 4,
                                    "win_pct": 0.529,
                                    "goals_for": 54,
                                    "goals_against": 39,
                                    "points": 22
                                },
                                {
                                    "record_type": "road",
                                    "wins": 20,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.571,
                                    "goals_for": 111,
                                    "goals_against": 93,
                                    "points": 44
                                },
                                {
                                    "record_type": "west",
                                    "wins": 20,
                                    "losses": 13,
                                    "overtime_losses": 5,
                                    "win_pct": 0.526,
                                    "goals_for": 125,
                                    "goals_against": 98,
                                    "points": 45
                                }
                            ],
                            "rank": {
                                "division": 3,
                                "conference": 7
                            },
                            "calc_rank": {
                                "div_rank": 3,
                                "conf_rank": 7
                            }
                        },
                        {
                            "id": "42376e1c-6da8-461e-9443-cfcf0a9fcc4d",
                            "name": "Golden Knights",
                            "market": "Vegas",
                            "sr_id": "sr:team:344158",
                            "games_played": 70,
                            "wins": 38,
                            "losses": 25,
                            "overtime_losses": 7,
                            "win_pct": 0.543,
                            "points": 83,
                            "shootout_wins": 4,
                            "shootout_losses": 2,
                            "goals_for": 226,
                            "goals_against": 208,
                            "goal_diff": 18,
                            "powerplays": 221,
                            "powerplay_goals": 42,
                            "powerplay_pct": 19.0,
                            "powerplays_against": 185,
                            "powerplay_goals_against": 36,
                            "penalty_killing_pct": 80.5,
                            "points_pct": 64.3,
                            "points_per_game": 1.2,
                            "regulation_wins": 34,
                            "streak": {
                                "kind": "win",
                                "length": 2
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 6,
                                    "losses": 9,
                                    "overtime_losses": 1,
                                    "win_pct": 0.375,
                                    "goals_for": 54,
                                    "goals_against": 67,
                                    "points": 13
                                },
                                {
                                    "record_type": "central",
                                    "wins": 10,
                                    "losses": 4,
                                    "overtime_losses": 2,
                                    "win_pct": 0.625,
                                    "goals_for": 56,
                                    "goals_against": 39,
                                    "points": 22
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 23,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.605,
                                    "goals_for": 122,
                                    "goals_against": 95,
                                    "points": 50
                                },
                                {
                                    "record_type": "division",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.591,
                                    "goals_for": 66,
                                    "goals_against": 56,
                                    "points": 28
                                },
                                {
                                    "record_type": "east",
                                    "wins": 15,
                                    "losses": 14,
                                    "overtime_losses": 3,
                                    "win_pct": 0.469,
                                    "goals_for": 104,
                                    "goals_against": 113,
                                    "points": 33
                                },
                                {
                                    "record_type": "home",
                                    "wins": 23,
                                    "losses": 11,
                                    "overtime_losses": 2,
                                    "win_pct": 0.639,
                                    "goals_for": 117,
                                    "goals_against": 91,
                                    "points": 48
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 5,
                                    "losses": 5,
                                    "overtime_losses": 0,
                                    "win_pct": 0.5,
                                    "goals_for": 30,
                                    "goals_against": 36,
                                    "points": 10
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 4,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.4,
                                    "goals_for": 29,
                                    "goals_against": 35,
                                    "points": 8
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 4,
                                    "losses": 5,
                                    "overtime_losses": 1,
                                    "win_pct": 0.4,
                                    "goals_for": 33,
                                    "goals_against": 39,
                                    "points": 9
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 9,
                                    "losses": 5,
                                    "overtime_losses": 2,
                                    "win_pct": 0.563,
                                    "goals_for": 50,
                                    "goals_against": 46,
                                    "points": 20
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.591,
                                    "goals_for": 66,
                                    "goals_against": 56,
                                    "points": 28
                                },
                                {
                                    "record_type": "road",
                                    "wins": 15,
                                    "losses": 14,
                                    "overtime_losses": 5,
                                    "win_pct": 0.441,
                                    "goals_for": 109,
                                    "goals_against": 117,
                                    "points": 35
                                },
                                {
                                    "record_type": "west",
                                    "wins": 23,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.605,
                                    "goals_for": 122,
                                    "goals_against": 95,
                                    "points": 50
                                }
                            ],
                            "rank": {
                                "division": 4,
                                "conference": 8,
                                "wildcard": 2
                            },
                            "calc_rank": {
                                "div_rank": 4,
                                "conf_rank": 8
                            }
                        },
                        {
                            "id": "44159241-0f24-11e2-8525-18a905767e44",
                            "name": "Flames",
                            "market": "Calgary",
                            "sr_id": "sr:team:3679",
                            "games_played": 69,
                            "wins": 33,
                            "losses": 31,
                            "overtime_losses": 5,
                            "win_pct": 0.478,
                            "points": 71,
                            "shootout_wins": 0,
                            "shootout_losses": 4,
                            "goals_for": 215,
                            "goals_against": 224,
                            "goal_diff": -9,
                            "powerplays": 214,
                            "powerplay_goals": 31,
                            "powerplay_pct": 14.5,
                            "powerplays_against": 207,
                            "powerplay_goals_against": 35,
                            "penalty_killing_pct": 83.1,
                            "points_pct": 55.1,
                            "points_per_game": 1.0,
                            "regulation_wins": 33,
                            "streak": {
                                "kind": "loss",
                                "length": 2
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 9,
                                    "losses": 5,
                                    "overtime_losses": 1,
                                    "win_pct": 0.6,
                                    "goals_for": 48,
                                    "goals_against": 47,
                                    "points": 19
                                },
                                {
                                    "record_type": "central",
                                    "wins": 10,
                                    "losses": 9,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 68,
                                    "goals_against": 65,
                                    "points": 21
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 19,
                                    "losses": 17,
                                    "overtime_losses": 2,
                                    "win_pct": 0.5,
                                    "goals_for": 126,
                                    "goals_against": 119,
                                    "points": 40
                                },
                                {
                                    "record_type": "division",
                                    "wins": 9,
                                    "losses": 8,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 58,
                                    "goals_against": 54,
                                    "points": 19
                                },
                                {
                                    "record_type": "east",
                                    "wins": 14,
                                    "losses": 14,
                                    "overtime_losses": 3,
                                    "win_pct": 0.452,
                                    "goals_for": 89,
                                    "goals_against": 105,
                                    "points": 31
                                },
                                {
                                    "record_type": "home",
                                    "wins": 18,
                                    "losses": 16,
                                    "overtime_losses": 1,
                                    "win_pct": 0.514,
                                    "goals_for": 104,
                                    "goals_against": 109,
                                    "points": 37
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 4,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.4,
                                    "goals_for": 30,
                                    "goals_against": 40,
                                    "points": 8
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 6,
                                    "losses": 4,
                                    "overtime_losses": 0,
                                    "win_pct": 0.6,
                                    "goals_for": 32,
                                    "goals_against": 33,
                                    "points": 12
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 6,
                                    "losses": 4,
                                    "overtime_losses": 0,
                                    "win_pct": 0.6,
                                    "goals_for": 34,
                                    "goals_against": 31,
                                    "points": 12
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 5,
                                    "losses": 9,
                                    "overtime_losses": 2,
                                    "win_pct": 0.313,
                                    "goals_for": 41,
                                    "goals_against": 58,
                                    "points": 12
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 9,
                                    "losses": 8,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 58,
                                    "goals_against": 54,
                                    "points": 19
                                },
                                {
                                    "record_type": "road",
                                    "wins": 15,
                                    "losses": 15,
                                    "overtime_losses": 4,
                                    "win_pct": 0.441,
                                    "goals_for": 111,
                                    "goals_against": 115,
                                    "points": 34
                                },
                                {
                                    "record_type": "west",
                                    "wins": 19,
                                    "losses": 17,
                                    "overtime_losses": 2,
                                    "win_pct": 0.5,
                                    "goals_for": 126,
                                    "goals_against": 119,
                                    "points": 40
                                }
                            ],
                            "rank": {
                                "division": 5,
                                "conference": 11,
                                "wildcard": 5
                            },
                            "calc_rank": {
                                "div_rank": 5,
                                "conf_rank": 11
                            }
                        },
                        {
                            "id": "1fb48e65-9688-4084-8868-02173525c3e1",
                            "name": "Kraken",
                            "market": "Seattle",
                            "sr_id": "sr:team:794340",
                            "games_played": 69,
                            "wins": 28,
                            "losses": 28,
                            "overtime_losses": 13,
                            "win_pct": 0.406,
                            "points": 69,
                            "shootout_wins": 3,
                            "shootout_losses": 4,
                            "goals_for": 184,
                            "goals_against": 201,
                            "goal_diff": -17,
                            "powerplays": 194,
                            "powerplay_goals": 41,
                            "powerplay_pct": 21.1,
                            "powerplays_against": 193,
                            "powerplay_goals_against": 40,
                            "penalty_killing_pct": 79.3,
                            "points_pct": 59.4,
                            "points_per_game": 1.0,
                            "regulation_wins": 25,
                            "streak": {
                                "kind": "loss",
                                "length": 7
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 7,
                                    "losses": 5,
                                    "overtime_losses": 3,
                                    "win_pct": 0.467,
                                    "goals_for": 46,
                                    "goals_against": 44,
                                    "points": 17
                                },
                                {
                                    "record_type": "central",
                                    "wins": 5,
                                    "losses": 8,
                                    "overtime_losses": 5,
                                    "win_pct": 0.278,
                                    "goals_for": 44,
                                    "goals_against": 58,
                                    "points": 15
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 13,
                                    "losses": 16,
                                    "overtime_losses": 9,
                                    "win_pct": 0.342,
                                    "goals_for": 96,
                                    "goals_against": 116,
                                    "points": 35
                                },
                                {
                                    "record_type": "division",
                                    "wins": 8,
                                    "losses": 8,
                                    "overtime_losses": 4,
                                    "win_pct": 0.4,
                                    "goals_for": 52,
                                    "goals_against": 58,
                                    "points": 20
                                },
                                {
                                    "record_type": "east",
                                    "wins": 15,
                                    "losses": 12,
                                    "overtime_losses": 4,
                                    "win_pct": 0.484,
                                    "goals_for": 88,
                                    "goals_against": 85,
                                    "points": 34
                                },
                                {
                                    "record_type": "home",
                                    "wins": 14,
                                    "losses": 15,
                                    "overtime_losses": 6,
                                    "win_pct": 0.4,
                                    "goals_for": 98,
                                    "goals_against": 104,
                                    "points": 34
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 2,
                                    "losses": 6,
                                    "overtime_losses": 2,
                                    "win_pct": 0.2,
                                    "goals_for": 19,
                                    "goals_against": 32,
                                    "points": 6
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 3,
                                    "losses": 6,
                                    "overtime_losses": 1,
                                    "win_pct": 0.3,
                                    "goals_for": 22,
                                    "goals_against": 32,
                                    "points": 7
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 4,
                                    "losses": 5,
                                    "overtime_losses": 1,
                                    "win_pct": 0.4,
                                    "goals_for": 21,
                                    "goals_against": 24,
                                    "points": 9
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 8,
                                    "losses": 7,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 42,
                                    "goals_against": 41,
                                    "points": 17
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 8,
                                    "losses": 8,
                                    "overtime_losses": 4,
                                    "win_pct": 0.4,
                                    "goals_for": 52,
                                    "goals_against": 58,
                                    "points": 20
                                },
                                {
                                    "record_type": "road",
                                    "wins": 14,
                                    "losses": 13,
                                    "overtime_losses": 7,
                                    "win_pct": 0.412,
                                    "goals_for": 86,
                                    "goals_against": 97,
                                    "points": 35
                                },
                                {
                                    "record_type": "west",
                                    "wins": 13,
                                    "losses": 16,
                                    "overtime_losses": 9,
                                    "win_pct": 0.342,
                                    "goals_for": 96,
                                    "goals_against": 116,
                                    "points": 35
                                }
                            ],
                            "rank": {
                                "division": 6,
                                "conference": 12,
                                "wildcard": 6
                            },
                            "calc_rank": {
                                "div_rank": 6,
                                "conf_rank": 12
                            }
                        },
                        {
                            "id": "441862de-0f24-11e2-8525-18a905767e44",
                            "name": "Ducks",
                            "market": "Anaheim",
                            "sr_id": "sr:team:3675",
                            "games_played": 70,
                            "wins": 24,
                            "losses": 43,
                            "overtime_losses": 3,
                            "win_pct": 0.343,
                            "points": 51,
                            "shootout_wins": 1,
                            "shootout_losses": 1,
                            "goals_for": 175,
                            "goals_against": 252,
                            "goal_diff": -77,
                            "powerplays": 209,
                            "powerplay_goals": 38,
                            "powerplay_pct": 18.2,
                            "powerplays_against": 281,
                            "powerplay_goals_against": 75,
                            "penalty_killing_pct": 73.3,
                            "points_pct": 38.6,
                            "points_per_game": 0.7,
                            "regulation_wins": 23,
                            "streak": {
                                "kind": "win",
                                "length": 1
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 7,
                                    "losses": 7,
                                    "overtime_losses": 1,
                                    "win_pct": 0.467,
                                    "goals_for": 39,
                                    "goals_against": 50,
                                    "points": 15
                                },
                                {
                                    "record_type": "central",
                                    "wins": 6,
                                    "losses": 16,
                                    "overtime_losses": 1,
                                    "win_pct": 0.261,
                                    "goals_for": 45,
                                    "goals_against": 79,
                                    "points": 13
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 11,
                                    "losses": 26,
                                    "overtime_losses": 2,
                                    "win_pct": 0.282,
                                    "goals_for": 86,
                                    "goals_against": 138,
                                    "points": 24
                                },
                                {
                                    "record_type": "division",
                                    "wins": 5,
                                    "losses": 10,
                                    "overtime_losses": 1,
                                    "win_pct": 0.313,
                                    "goals_for": 41,
                                    "goals_against": 59,
                                    "points": 11
                                },
                                {
                                    "record_type": "east",
                                    "wins": 13,
                                    "losses": 17,
                                    "overtime_losses": 1,
                                    "win_pct": 0.419,
                                    "goals_for": 89,
                                    "goals_against": 114,
                                    "points": 27
                                },
                                {
                                    "record_type": "home",
                                    "wins": 11,
                                    "losses": 24,
                                    "overtime_losses": 1,
                                    "win_pct": 0.306,
                                    "goals_for": 86,
                                    "goals_against": 121,
                                    "points": 23
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 2,
                                    "losses": 8,
                                    "overtime_losses": 0,
                                    "win_pct": 0.2,
                                    "goals_for": 14,
                                    "goals_against": 38,
                                    "points": 4
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 3,
                                    "losses": 7,
                                    "overtime_losses": 0,
                                    "win_pct": 0.3,
                                    "goals_for": 23,
                                    "goals_against": 38,
                                    "points": 6
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 3,
                                    "losses": 6,
                                    "overtime_losses": 1,
                                    "win_pct": 0.3,
                                    "goals_for": 23,
                                    "goals_against": 44,
                                    "points": 7
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 6,
                                    "losses": 10,
                                    "overtime_losses": 0,
                                    "win_pct": 0.375,
                                    "goals_for": 50,
                                    "goals_against": 64,
                                    "points": 12
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 5,
                                    "losses": 10,
                                    "overtime_losses": 1,
                                    "win_pct": 0.313,
                                    "goals_for": 41,
                                    "goals_against": 59,
                                    "points": 11
                                },
                                {
                                    "record_type": "road",
                                    "wins": 13,
                                    "losses": 19,
                                    "overtime_losses": 2,
                                    "win_pct": 0.382,
                                    "goals_for": 89,
                                    "goals_against": 131,
                                    "points": 28
                                },
                                {
                                    "record_type": "west",
                                    "wins": 11,
                                    "losses": 26,
                                    "overtime_losses": 2,
                                    "win_pct": 0.282,
                                    "goals_for": 86,
                                    "goals_against": 138,
                                    "points": 24
                                }
                            ],
                            "rank": {
                                "division": 7,
                                "conference": 14,
                                "clinched": "eliminated",
                                "wildcard": 8
                            },
                            "calc_rank": {
                                "div_rank": 7,
                                "conf_rank": 14
                            }
                        },
                        {
                            "id": "44155909-0f24-11e2-8525-18a905767e44",
                            "name": "Sharks",
                            "market": "San Jose",
                            "sr_id": "sr:team:3696",
                            "games_played": 70,
                            "wins": 16,
                            "losses": 46,
                            "overtime_losses": 8,
                            "win_pct": 0.229,
                            "points": 40,
                            "shootout_wins": 1,
                            "shootout_losses": 5,
                            "goals_for": 155,
                            "goals_against": 285,
                            "goal_diff": -130,
                            "powerplays": 173,
                            "powerplay_goals": 36,
                            "powerplay_pct": 20.8,
                            "powerplays_against": 220,
                            "powerplay_goals_against": 56,
                            "penalty_killing_pct": 74.5,
                            "points_pct": 34.3,
                            "points_per_game": 0.6,
                            "regulation_wins": 15,
                            "streak": {
                                "kind": "loss",
                                "length": 7
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 3,
                                    "losses": 12,
                                    "overtime_losses": 1,
                                    "win_pct": 0.188,
                                    "goals_for": 30,
                                    "goals_against": 64,
                                    "points": 7
                                },
                                {
                                    "record_type": "central",
                                    "wins": 2,
                                    "losses": 11,
                                    "overtime_losses": 5,
                                    "win_pct": 0.111,
                                    "goals_for": 37,
                                    "goals_against": 65,
                                    "points": 9
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 8,
                                    "losses": 23,
                                    "overtime_losses": 7,
                                    "win_pct": 0.211,
                                    "goals_for": 82,
                                    "goals_against": 151,
                                    "points": 23
                                },
                                {
                                    "record_type": "division",
                                    "wins": 6,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.3,
                                    "goals_for": 45,
                                    "goals_against": 86,
                                    "points": 14
                                },
                                {
                                    "record_type": "east",
                                    "wins": 8,
                                    "losses": 23,
                                    "overtime_losses": 1,
                                    "win_pct": 0.25,
                                    "goals_for": 73,
                                    "goals_against": 134,
                                    "points": 17
                                },
                                {
                                    "record_type": "home",
                                    "wins": 10,
                                    "losses": 20,
                                    "overtime_losses": 4,
                                    "win_pct": 0.294,
                                    "goals_for": 78,
                                    "goals_against": 136,
                                    "points": 24
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 1,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.1,
                                    "goals_for": 26,
                                    "goals_against": 50,
                                    "points": 4
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 1,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.1,
                                    "goals_for": 26,
                                    "goals_against": 49,
                                    "points": 4
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 1,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.1,
                                    "goals_for": 24,
                                    "goals_against": 40,
                                    "points": 4
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 5,
                                    "losses": 11,
                                    "overtime_losses": 0,
                                    "win_pct": 0.313,
                                    "goals_for": 43,
                                    "goals_against": 70,
                                    "points": 10
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 6,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.3,
                                    "goals_for": 45,
                                    "goals_against": 86,
                                    "points": 14
                                },
                                {
                                    "record_type": "road",
                                    "wins": 6,
                                    "losses": 26,
                                    "overtime_losses": 4,
                                    "win_pct": 0.167,
                                    "goals_for": 77,
                                    "goals_against": 149,
                                    "points": 16
                                },
                                {
                                    "record_type": "west",
                                    "wins": 8,
                                    "losses": 23,
                                    "overtime_losses": 7,
                                    "win_pct": 0.211,
                                    "goals_for": 82,
                                    "goals_against": 151,
                                    "points": 23
                                }
                            ],
                            "rank": {
                                "division": 8,
                                "conference": 16,
                                "clinched": "eliminated",
                                "wildcard": 10
                            },
                            "calc_rank": {
                                "div_rank": 8,
                                "conf_rank": 16
                            }
                        }
                    ]
                }
        
        },
         {
            "id": "64901512-9ca9-4bea-aa80-16dbcbdae230",
            "name": "WESTERN CONFERENCE",
            "alias": "WESTERN",
            "divisions": [
                {
                    "id": "17101b65-e8b9-4cda-a963-eea874aed81f",
                    "name": "Pacific",
                    "alias": "PACIFIC",
                    "teams": [
                        {
                            "id": "4415b0a7-0f24-11e2-8525-18a905767e44",
                            "name": "Canucks",
                            "market": "Vancouver",
                            "sr_id": "sr:team:3692",
                            "games_played": 71,
                            "wins": 45,
                            "losses": 18,
                            "overtime_losses": 8,
                            "win_pct": 0.634,
                            "points": 98,
                            "shootout_wins": 0,
                            "shootout_losses": 2,
                            "goals_for": 249,
                            "goals_against": 189,
                            "goal_diff": 60,
                            "powerplays": 225,
                            "powerplay_goals": 50,
                            "powerplay_pct": 22.2,
                            "powerplays_against": 215,
                            "powerplay_goals_against": 44,
                            "penalty_killing_pct": 79.5,
                            "points_pct": 74.6,
                            "points_per_game": 1.4,
                            "regulation_wins": 45,
                            "streak": {
                                "kind": "win",
                                "length": 3
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 12,
                                    "losses": 3,
                                    "overtime_losses": 1,
                                    "win_pct": 0.75,
                                    "goals_for": 58,
                                    "goals_against": 38,
                                    "points": 25
                                },
                                {
                                    "record_type": "central",
                                    "wins": 11,
                                    "losses": 5,
                                    "overtime_losses": 4,
                                    "win_pct": 0.55,
                                    "goals_for": 62,
                                    "goals_against": 50,
                                    "points": 26
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 24,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.615,
                                    "goals_for": 135,
                                    "goals_against": 99,
                                    "points": 52
                                },
                                {
                                    "record_type": "division",
                                    "wins": 13,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.684,
                                    "goals_for": 73,
                                    "goals_against": 49,
                                    "points": 26
                                },
                                {
                                    "record_type": "east",
                                    "wins": 21,
                                    "losses": 7,
                                    "overtime_losses": 4,
                                    "win_pct": 0.656,
                                    "goals_for": 114,
                                    "goals_against": 90,
                                    "points": 46
                                },
                                {
                                    "record_type": "home",
                                    "wins": 24,
                                    "losses": 7,
                                    "overtime_losses": 4,
                                    "win_pct": 0.686,
                                    "goals_for": 127,
                                    "goals_against": 83,
                                    "points": 52
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 7,
                                    "losses": 2,
                                    "overtime_losses": 1,
                                    "win_pct": 0.7,
                                    "goals_for": 28,
                                    "goals_against": 19,
                                    "points": 15
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 5,
                                    "losses": 3,
                                    "overtime_losses": 2,
                                    "win_pct": 0.5,
                                    "goals_for": 29,
                                    "goals_against": 26,
                                    "points": 12
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 5,
                                    "losses": 4,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 27,
                                    "goals_against": 33,
                                    "points": 11
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 3,
                                    "win_pct": 0.563,
                                    "goals_for": 56,
                                    "goals_against": 52,
                                    "points": 21
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 13,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.684,
                                    "goals_for": 73,
                                    "goals_against": 49,
                                    "points": 26
                                },
                                {
                                    "record_type": "road",
                                    "wins": 21,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.583,
                                    "goals_for": 122,
                                    "goals_against": 106,
                                    "points": 46
                                },
                                {
                                    "record_type": "west",
                                    "wins": 24,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.615,
                                    "goals_for": 135,
                                    "goals_against": 99,
                                    "points": 52
                                }
                            ],
                            "rank": {
                                "division": 1,
                                "conference": 1
                            },
                            "calc_rank": {
                                "div_rank": 1,
                                "conf_rank": 1
                            }
                        },
                        {
                            "id": "4415ea6c-0f24-11e2-8525-18a905767e44",
                            "name": "Oilers",
                            "market": "Edmonton",
                            "sr_id": "sr:team:3686",
                            "games_played": 68,
                            "wins": 42,
                            "losses": 22,
                            "overtime_losses": 4,
                            "win_pct": 0.618,
                            "points": 88,
                            "shootout_wins": 2,
                            "shootout_losses": 1,
                            "goals_for": 245,
                            "goals_against": 196,
                            "goal_diff": 49,
                            "powerplays": 204,
                            "powerplay_goals": 55,
                            "powerplay_pct": 27.0,
                            "powerplays_against": 218,
                            "powerplay_goals_against": 43,
                            "penalty_killing_pct": 80.3,
                            "points_pct": 67.6,
                            "points_per_game": 1.3,
                            "regulation_wins": 40,
                            "streak": {
                                "kind": "loss",
                                "length": 1
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 8,
                                    "losses": 5,
                                    "overtime_losses": 2,
                                    "win_pct": 0.533,
                                    "goals_for": 55,
                                    "goals_against": 54,
                                    "points": 18
                                },
                                {
                                    "record_type": "central",
                                    "wins": 10,
                                    "losses": 5,
                                    "overtime_losses": 2,
                                    "win_pct": 0.588,
                                    "goals_for": 57,
                                    "goals_against": 48,
                                    "points": 22
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 23,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.622,
                                    "goals_for": 128,
                                    "goals_against": 107,
                                    "points": 48
                                },
                                {
                                    "record_type": "division",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 0,
                                    "win_pct": 0.65,
                                    "goals_for": 71,
                                    "goals_against": 59,
                                    "points": 26
                                },
                                {
                                    "record_type": "east",
                                    "wins": 19,
                                    "losses": 10,
                                    "overtime_losses": 2,
                                    "win_pct": 0.613,
                                    "goals_for": 117,
                                    "goals_against": 89,
                                    "points": 40
                                },
                                {
                                    "record_type": "home",
                                    "wins": 23,
                                    "losses": 8,
                                    "overtime_losses": 3,
                                    "win_pct": 0.676,
                                    "goals_for": 137,
                                    "goals_against": 93,
                                    "points": 49
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 6,
                                    "losses": 2,
                                    "overtime_losses": 2,
                                    "win_pct": 0.6,
                                    "goals_for": 39,
                                    "goals_against": 25,
                                    "points": 14
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 6,
                                    "losses": 2,
                                    "overtime_losses": 2,
                                    "win_pct": 0.6,
                                    "goals_for": 43,
                                    "goals_against": 31,
                                    "points": 14
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 5,
                                    "losses": 4,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 28,
                                    "goals_against": 31,
                                    "points": 11
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 11,
                                    "losses": 5,
                                    "overtime_losses": 0,
                                    "win_pct": 0.688,
                                    "goals_for": 62,
                                    "goals_against": 35,
                                    "points": 22
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 0,
                                    "win_pct": 0.65,
                                    "goals_for": 71,
                                    "goals_against": 59,
                                    "points": 26
                                },
                                {
                                    "record_type": "road",
                                    "wins": 19,
                                    "losses": 14,
                                    "overtime_losses": 1,
                                    "win_pct": 0.559,
                                    "goals_for": 108,
                                    "goals_against": 103,
                                    "points": 39
                                },
                                {
                                    "record_type": "west",
                                    "wins": 23,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.622,
                                    "goals_for": 128,
                                    "goals_against": 107,
                                    "points": 48
                                }
                            ],
                            "rank": {
                                "division": 2,
                                "conference": 5
                            },
                            "calc_rank": {
                                "div_rank": 2,
                                "conf_rank": 5,
                                "conf_tiebreak": "fewest_games_played"
                            }
                        },
                        {
                            "id": "44151f7a-0f24-11e2-8525-18a905767e44",
                            "name": "Kings",
                            "market": "Los Angeles",
                            "sr_id": "sr:team:3688",
                            "games_played": 70,
                            "wins": 37,
                            "losses": 22,
                            "overtime_losses": 11,
                            "win_pct": 0.529,
                            "points": 85,
                            "shootout_wins": 2,
                            "shootout_losses": 5,
                            "goals_for": 220,
                            "goals_against": 183,
                            "goal_diff": 37,
                            "powerplays": 211,
                            "powerplay_goals": 47,
                            "powerplay_pct": 22.3,
                            "powerplays_against": 222,
                            "powerplay_goals_against": 32,
                            "penalty_killing_pct": 85.6,
                            "points_pct": 68.6,
                            "points_per_game": 1.2,
                            "regulation_wins": 35,
                            "streak": {
                                "kind": "win",
                                "length": 3
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 8,
                                    "losses": 5,
                                    "overtime_losses": 3,
                                    "win_pct": 0.5,
                                    "goals_for": 45,
                                    "goals_against": 48,
                                    "points": 19
                                },
                                {
                                    "record_type": "central",
                                    "wins": 11,
                                    "losses": 9,
                                    "overtime_losses": 1,
                                    "win_pct": 0.524,
                                    "goals_for": 71,
                                    "goals_against": 59,
                                    "points": 23
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 20,
                                    "losses": 13,
                                    "overtime_losses": 5,
                                    "win_pct": 0.526,
                                    "goals_for": 125,
                                    "goals_against": 98,
                                    "points": 45
                                },
                                {
                                    "record_type": "division",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 4,
                                    "win_pct": 0.529,
                                    "goals_for": 54,
                                    "goals_against": 39,
                                    "points": 22
                                },
                                {
                                    "record_type": "east",
                                    "wins": 17,
                                    "losses": 9,
                                    "overtime_losses": 6,
                                    "win_pct": 0.531,
                                    "goals_for": 95,
                                    "goals_against": 85,
                                    "points": 40
                                },
                                {
                                    "record_type": "home",
                                    "wins": 17,
                                    "losses": 11,
                                    "overtime_losses": 7,
                                    "win_pct": 0.486,
                                    "goals_for": 109,
                                    "goals_against": 90,
                                    "points": 41
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 6,
                                    "losses": 3,
                                    "overtime_losses": 1,
                                    "win_pct": 0.6,
                                    "goals_for": 32,
                                    "goals_against": 21,
                                    "points": 13
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 7,
                                    "losses": 2,
                                    "overtime_losses": 1,
                                    "win_pct": 0.7,
                                    "goals_for": 34,
                                    "goals_against": 21,
                                    "points": 15
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 5,
                                    "losses": 5,
                                    "overtime_losses": 0,
                                    "win_pct": 0.5,
                                    "goals_for": 25,
                                    "goals_against": 29,
                                    "points": 10
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 3,
                                    "win_pct": 0.563,
                                    "goals_for": 50,
                                    "goals_against": 37,
                                    "points": 21
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 9,
                                    "losses": 4,
                                    "overtime_losses": 4,
                                    "win_pct": 0.529,
                                    "goals_for": 54,
                                    "goals_against": 39,
                                    "points": 22
                                },
                                {
                                    "record_type": "road",
                                    "wins": 20,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.571,
                                    "goals_for": 111,
                                    "goals_against": 93,
                                    "points": 44
                                },
                                {
                                    "record_type": "west",
                                    "wins": 20,
                                    "losses": 13,
                                    "overtime_losses": 5,
                                    "win_pct": 0.526,
                                    "goals_for": 125,
                                    "goals_against": 98,
                                    "points": 45
                                }
                            ],
                            "rank": {
                                "division": 3,
                                "conference": 7
                            },
                            "calc_rank": {
                                "div_rank": 3,
                                "conf_rank": 7
                            }
                        },
                        {
                            "id": "42376e1c-6da8-461e-9443-cfcf0a9fcc4d",
                            "name": "Golden Knights",
                            "market": "Vegas",
                            "sr_id": "sr:team:344158",
                            "games_played": 70,
                            "wins": 38,
                            "losses": 25,
                            "overtime_losses": 7,
                            "win_pct": 0.543,
                            "points": 83,
                            "shootout_wins": 4,
                            "shootout_losses": 2,
                            "goals_for": 226,
                            "goals_against": 208,
                            "goal_diff": 18,
                            "powerplays": 221,
                            "powerplay_goals": 42,
                            "powerplay_pct": 19.0,
                            "powerplays_against": 185,
                            "powerplay_goals_against": 36,
                            "penalty_killing_pct": 80.5,
                            "points_pct": 64.3,
                            "points_per_game": 1.2,
                            "regulation_wins": 34,
                            "streak": {
                                "kind": "win",
                                "length": 2
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 6,
                                    "losses": 9,
                                    "overtime_losses": 1,
                                    "win_pct": 0.375,
                                    "goals_for": 54,
                                    "goals_against": 67,
                                    "points": 13
                                },
                                {
                                    "record_type": "central",
                                    "wins": 10,
                                    "losses": 4,
                                    "overtime_losses": 2,
                                    "win_pct": 0.625,
                                    "goals_for": 56,
                                    "goals_against": 39,
                                    "points": 22
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 23,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.605,
                                    "goals_for": 122,
                                    "goals_against": 95,
                                    "points": 50
                                },
                                {
                                    "record_type": "division",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.591,
                                    "goals_for": 66,
                                    "goals_against": 56,
                                    "points": 28
                                },
                                {
                                    "record_type": "east",
                                    "wins": 15,
                                    "losses": 14,
                                    "overtime_losses": 3,
                                    "win_pct": 0.469,
                                    "goals_for": 104,
                                    "goals_against": 113,
                                    "points": 33
                                },
                                {
                                    "record_type": "home",
                                    "wins": 23,
                                    "losses": 11,
                                    "overtime_losses": 2,
                                    "win_pct": 0.639,
                                    "goals_for": 117,
                                    "goals_against": 91,
                                    "points": 48
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 5,
                                    "losses": 5,
                                    "overtime_losses": 0,
                                    "win_pct": 0.5,
                                    "goals_for": 30,
                                    "goals_against": 36,
                                    "points": 10
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 4,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.4,
                                    "goals_for": 29,
                                    "goals_against": 35,
                                    "points": 8
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 4,
                                    "losses": 5,
                                    "overtime_losses": 1,
                                    "win_pct": 0.4,
                                    "goals_for": 33,
                                    "goals_against": 39,
                                    "points": 9
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 9,
                                    "losses": 5,
                                    "overtime_losses": 2,
                                    "win_pct": 0.563,
                                    "goals_for": 50,
                                    "goals_against": 46,
                                    "points": 20
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 13,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.591,
                                    "goals_for": 66,
                                    "goals_against": 56,
                                    "points": 28
                                },
                                {
                                    "record_type": "road",
                                    "wins": 15,
                                    "losses": 14,
                                    "overtime_losses": 5,
                                    "win_pct": 0.441,
                                    "goals_for": 109,
                                    "goals_against": 117,
                                    "points": 35
                                },
                                {
                                    "record_type": "west",
                                    "wins": 23,
                                    "losses": 11,
                                    "overtime_losses": 4,
                                    "win_pct": 0.605,
                                    "goals_for": 122,
                                    "goals_against": 95,
                                    "points": 50
                                }
                            ],
                            "rank": {
                                "division": 4,
                                "conference": 8,
                                "wildcard": 2
                            },
                            "calc_rank": {
                                "div_rank": 4,
                                "conf_rank": 8
                            }
                        },
                        {
                            "id": "44159241-0f24-11e2-8525-18a905767e44",
                            "name": "Flames",
                            "market": "Calgary",
                            "sr_id": "sr:team:3679",
                            "games_played": 69,
                            "wins": 33,
                            "losses": 31,
                            "overtime_losses": 5,
                            "win_pct": 0.478,
                            "points": 71,
                            "shootout_wins": 0,
                            "shootout_losses": 4,
                            "goals_for": 215,
                            "goals_against": 224,
                            "goal_diff": -9,
                            "powerplays": 214,
                            "powerplay_goals": 31,
                            "powerplay_pct": 14.5,
                            "powerplays_against": 207,
                            "powerplay_goals_against": 35,
                            "penalty_killing_pct": 83.1,
                            "points_pct": 55.1,
                            "points_per_game": 1.0,
                            "regulation_wins": 33,
                            "streak": {
                                "kind": "loss",
                                "length": 2
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 9,
                                    "losses": 5,
                                    "overtime_losses": 1,
                                    "win_pct": 0.6,
                                    "goals_for": 48,
                                    "goals_against": 47,
                                    "points": 19
                                },
                                {
                                    "record_type": "central",
                                    "wins": 10,
                                    "losses": 9,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 68,
                                    "goals_against": 65,
                                    "points": 21
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 19,
                                    "losses": 17,
                                    "overtime_losses": 2,
                                    "win_pct": 0.5,
                                    "goals_for": 126,
                                    "goals_against": 119,
                                    "points": 40
                                },
                                {
                                    "record_type": "division",
                                    "wins": 9,
                                    "losses": 8,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 58,
                                    "goals_against": 54,
                                    "points": 19
                                },
                                {
                                    "record_type": "east",
                                    "wins": 14,
                                    "losses": 14,
                                    "overtime_losses": 3,
                                    "win_pct": 0.452,
                                    "goals_for": 89,
                                    "goals_against": 105,
                                    "points": 31
                                },
                                {
                                    "record_type": "home",
                                    "wins": 18,
                                    "losses": 16,
                                    "overtime_losses": 1,
                                    "win_pct": 0.514,
                                    "goals_for": 104,
                                    "goals_against": 109,
                                    "points": 37
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 4,
                                    "losses": 6,
                                    "overtime_losses": 0,
                                    "win_pct": 0.4,
                                    "goals_for": 30,
                                    "goals_against": 40,
                                    "points": 8
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 6,
                                    "losses": 4,
                                    "overtime_losses": 0,
                                    "win_pct": 0.6,
                                    "goals_for": 32,
                                    "goals_against": 33,
                                    "points": 12
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 6,
                                    "losses": 4,
                                    "overtime_losses": 0,
                                    "win_pct": 0.6,
                                    "goals_for": 34,
                                    "goals_against": 31,
                                    "points": 12
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 5,
                                    "losses": 9,
                                    "overtime_losses": 2,
                                    "win_pct": 0.313,
                                    "goals_for": 41,
                                    "goals_against": 58,
                                    "points": 12
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 9,
                                    "losses": 8,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 58,
                                    "goals_against": 54,
                                    "points": 19
                                },
                                {
                                    "record_type": "road",
                                    "wins": 15,
                                    "losses": 15,
                                    "overtime_losses": 4,
                                    "win_pct": 0.441,
                                    "goals_for": 111,
                                    "goals_against": 115,
                                    "points": 34
                                },
                                {
                                    "record_type": "west",
                                    "wins": 19,
                                    "losses": 17,
                                    "overtime_losses": 2,
                                    "win_pct": 0.5,
                                    "goals_for": 126,
                                    "goals_against": 119,
                                    "points": 40
                                }
                            ],
                            "rank": {
                                "division": 5,
                                "conference": 11,
                                "wildcard": 5
                            },
                            "calc_rank": {
                                "div_rank": 5,
                                "conf_rank": 11
                            }
                        },
                        {
                            "id": "1fb48e65-9688-4084-8868-02173525c3e1",
                            "name": "Kraken",
                            "market": "Seattle",
                            "sr_id": "sr:team:794340",
                            "games_played": 69,
                            "wins": 28,
                            "losses": 28,
                            "overtime_losses": 13,
                            "win_pct": 0.406,
                            "points": 69,
                            "shootout_wins": 3,
                            "shootout_losses": 4,
                            "goals_for": 184,
                            "goals_against": 201,
                            "goal_diff": -17,
                            "powerplays": 194,
                            "powerplay_goals": 41,
                            "powerplay_pct": 21.1,
                            "powerplays_against": 193,
                            "powerplay_goals_against": 40,
                            "penalty_killing_pct": 79.3,
                            "points_pct": 59.4,
                            "points_per_game": 1.0,
                            "regulation_wins": 25,
                            "streak": {
                                "kind": "loss",
                                "length": 7
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 7,
                                    "losses": 5,
                                    "overtime_losses": 3,
                                    "win_pct": 0.467,
                                    "goals_for": 46,
                                    "goals_against": 44,
                                    "points": 17
                                },
                                {
                                    "record_type": "central",
                                    "wins": 5,
                                    "losses": 8,
                                    "overtime_losses": 5,
                                    "win_pct": 0.278,
                                    "goals_for": 44,
                                    "goals_against": 58,
                                    "points": 15
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 13,
                                    "losses": 16,
                                    "overtime_losses": 9,
                                    "win_pct": 0.342,
                                    "goals_for": 96,
                                    "goals_against": 116,
                                    "points": 35
                                },
                                {
                                    "record_type": "division",
                                    "wins": 8,
                                    "losses": 8,
                                    "overtime_losses": 4,
                                    "win_pct": 0.4,
                                    "goals_for": 52,
                                    "goals_against": 58,
                                    "points": 20
                                },
                                {
                                    "record_type": "east",
                                    "wins": 15,
                                    "losses": 12,
                                    "overtime_losses": 4,
                                    "win_pct": 0.484,
                                    "goals_for": 88,
                                    "goals_against": 85,
                                    "points": 34
                                },
                                {
                                    "record_type": "home",
                                    "wins": 14,
                                    "losses": 15,
                                    "overtime_losses": 6,
                                    "win_pct": 0.4,
                                    "goals_for": 98,
                                    "goals_against": 104,
                                    "points": 34
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 2,
                                    "losses": 6,
                                    "overtime_losses": 2,
                                    "win_pct": 0.2,
                                    "goals_for": 19,
                                    "goals_against": 32,
                                    "points": 6
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 3,
                                    "losses": 6,
                                    "overtime_losses": 1,
                                    "win_pct": 0.3,
                                    "goals_for": 22,
                                    "goals_against": 32,
                                    "points": 7
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 4,
                                    "losses": 5,
                                    "overtime_losses": 1,
                                    "win_pct": 0.4,
                                    "goals_for": 21,
                                    "goals_against": 24,
                                    "points": 9
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 8,
                                    "losses": 7,
                                    "overtime_losses": 1,
                                    "win_pct": 0.5,
                                    "goals_for": 42,
                                    "goals_against": 41,
                                    "points": 17
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 8,
                                    "losses": 8,
                                    "overtime_losses": 4,
                                    "win_pct": 0.4,
                                    "goals_for": 52,
                                    "goals_against": 58,
                                    "points": 20
                                },
                                {
                                    "record_type": "road",
                                    "wins": 14,
                                    "losses": 13,
                                    "overtime_losses": 7,
                                    "win_pct": 0.412,
                                    "goals_for": 86,
                                    "goals_against": 97,
                                    "points": 35
                                },
                                {
                                    "record_type": "west",
                                    "wins": 13,
                                    "losses": 16,
                                    "overtime_losses": 9,
                                    "win_pct": 0.342,
                                    "goals_for": 96,
                                    "goals_against": 116,
                                    "points": 35
                                }
                            ],
                            "rank": {
                                "division": 6,
                                "conference": 12,
                                "wildcard": 6
                            },
                            "calc_rank": {
                                "div_rank": 6,
                                "conf_rank": 12
                            }
                        },
                        {
                            "id": "441862de-0f24-11e2-8525-18a905767e44",
                            "name": "Ducks",
                            "market": "Anaheim",
                            "sr_id": "sr:team:3675",
                            "games_played": 70,
                            "wins": 24,
                            "losses": 43,
                            "overtime_losses": 3,
                            "win_pct": 0.343,
                            "points": 51,
                            "shootout_wins": 1,
                            "shootout_losses": 1,
                            "goals_for": 175,
                            "goals_against": 252,
                            "goal_diff": -77,
                            "powerplays": 209,
                            "powerplay_goals": 38,
                            "powerplay_pct": 18.2,
                            "powerplays_against": 281,
                            "powerplay_goals_against": 75,
                            "penalty_killing_pct": 73.3,
                            "points_pct": 38.6,
                            "points_per_game": 0.7,
                            "regulation_wins": 23,
                            "streak": {
                                "kind": "win",
                                "length": 1
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 7,
                                    "losses": 7,
                                    "overtime_losses": 1,
                                    "win_pct": 0.467,
                                    "goals_for": 39,
                                    "goals_against": 50,
                                    "points": 15
                                },
                                {
                                    "record_type": "central",
                                    "wins": 6,
                                    "losses": 16,
                                    "overtime_losses": 1,
                                    "win_pct": 0.261,
                                    "goals_for": 45,
                                    "goals_against": 79,
                                    "points": 13
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 11,
                                    "losses": 26,
                                    "overtime_losses": 2,
                                    "win_pct": 0.282,
                                    "goals_for": 86,
                                    "goals_against": 138,
                                    "points": 24
                                },
                                {
                                    "record_type": "division",
                                    "wins": 5,
                                    "losses": 10,
                                    "overtime_losses": 1,
                                    "win_pct": 0.313,
                                    "goals_for": 41,
                                    "goals_against": 59,
                                    "points": 11
                                },
                                {
                                    "record_type": "east",
                                    "wins": 13,
                                    "losses": 17,
                                    "overtime_losses": 1,
                                    "win_pct": 0.419,
                                    "goals_for": 89,
                                    "goals_against": 114,
                                    "points": 27
                                },
                                {
                                    "record_type": "home",
                                    "wins": 11,
                                    "losses": 24,
                                    "overtime_losses": 1,
                                    "win_pct": 0.306,
                                    "goals_for": 86,
                                    "goals_against": 121,
                                    "points": 23
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 2,
                                    "losses": 8,
                                    "overtime_losses": 0,
                                    "win_pct": 0.2,
                                    "goals_for": 14,
                                    "goals_against": 38,
                                    "points": 4
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 3,
                                    "losses": 7,
                                    "overtime_losses": 0,
                                    "win_pct": 0.3,
                                    "goals_for": 23,
                                    "goals_against": 38,
                                    "points": 6
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 3,
                                    "losses": 6,
                                    "overtime_losses": 1,
                                    "win_pct": 0.3,
                                    "goals_for": 23,
                                    "goals_against": 44,
                                    "points": 7
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 6,
                                    "losses": 10,
                                    "overtime_losses": 0,
                                    "win_pct": 0.375,
                                    "goals_for": 50,
                                    "goals_against": 64,
                                    "points": 12
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 5,
                                    "losses": 10,
                                    "overtime_losses": 1,
                                    "win_pct": 0.313,
                                    "goals_for": 41,
                                    "goals_against": 59,
                                    "points": 11
                                },
                                {
                                    "record_type": "road",
                                    "wins": 13,
                                    "losses": 19,
                                    "overtime_losses": 2,
                                    "win_pct": 0.382,
                                    "goals_for": 89,
                                    "goals_against": 131,
                                    "points": 28
                                },
                                {
                                    "record_type": "west",
                                    "wins": 11,
                                    "losses": 26,
                                    "overtime_losses": 2,
                                    "win_pct": 0.282,
                                    "goals_for": 86,
                                    "goals_against": 138,
                                    "points": 24
                                }
                            ],
                            "rank": {
                                "division": 7,
                                "conference": 14,
                                "clinched": "eliminated",
                                "wildcard": 8
                            },
                            "calc_rank": {
                                "div_rank": 7,
                                "conf_rank": 14
                            }
                        },
                        {
                            "id": "44155909-0f24-11e2-8525-18a905767e44",
                            "name": "Sharks",
                            "market": "San Jose",
                            "sr_id": "sr:team:3696",
                            "games_played": 70,
                            "wins": 16,
                            "losses": 46,
                            "overtime_losses": 8,
                            "win_pct": 0.229,
                            "points": 40,
                            "shootout_wins": 1,
                            "shootout_losses": 5,
                            "goals_for": 155,
                            "goals_against": 285,
                            "goal_diff": -130,
                            "powerplays": 173,
                            "powerplay_goals": 36,
                            "powerplay_pct": 20.8,
                            "powerplays_against": 220,
                            "powerplay_goals_against": 56,
                            "penalty_killing_pct": 74.5,
                            "points_pct": 34.3,
                            "points_per_game": 0.6,
                            "regulation_wins": 15,
                            "streak": {
                                "kind": "loss",
                                "length": 7
                            },
                            "records": [
                                {
                                    "record_type": "atlantic",
                                    "wins": 3,
                                    "losses": 12,
                                    "overtime_losses": 1,
                                    "win_pct": 0.188,
                                    "goals_for": 30,
                                    "goals_against": 64,
                                    "points": 7
                                },
                                {
                                    "record_type": "central",
                                    "wins": 2,
                                    "losses": 11,
                                    "overtime_losses": 5,
                                    "win_pct": 0.111,
                                    "goals_for": 37,
                                    "goals_against": 65,
                                    "points": 9
                                },
                                {
                                    "record_type": "conference",
                                    "wins": 8,
                                    "losses": 23,
                                    "overtime_losses": 7,
                                    "win_pct": 0.211,
                                    "goals_for": 82,
                                    "goals_against": 151,
                                    "points": 23
                                },
                                {
                                    "record_type": "division",
                                    "wins": 6,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.3,
                                    "goals_for": 45,
                                    "goals_against": 86,
                                    "points": 14
                                },
                                {
                                    "record_type": "east",
                                    "wins": 8,
                                    "losses": 23,
                                    "overtime_losses": 1,
                                    "win_pct": 0.25,
                                    "goals_for": 73,
                                    "goals_against": 134,
                                    "points": 17
                                },
                                {
                                    "record_type": "home",
                                    "wins": 10,
                                    "losses": 20,
                                    "overtime_losses": 4,
                                    "win_pct": 0.294,
                                    "goals_for": 78,
                                    "goals_against": 136,
                                    "points": 24
                                },
                                {
                                    "record_type": "last_10",
                                    "wins": 1,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.1,
                                    "goals_for": 26,
                                    "goals_against": 50,
                                    "points": 4
                                },
                                {
                                    "record_type": "last_10_home",
                                    "wins": 1,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.1,
                                    "goals_for": 26,
                                    "goals_against": 49,
                                    "points": 4
                                },
                                {
                                    "record_type": "last_10_road",
                                    "wins": 1,
                                    "losses": 7,
                                    "overtime_losses": 2,
                                    "win_pct": 0.1,
                                    "goals_for": 24,
                                    "goals_against": 40,
                                    "points": 4
                                },
                                {
                                    "record_type": "metropolitan",
                                    "wins": 5,
                                    "losses": 11,
                                    "overtime_losses": 0,
                                    "win_pct": 0.313,
                                    "goals_for": 43,
                                    "goals_against": 70,
                                    "points": 10
                                },
                                {
                                    "record_type": "pacific",
                                    "wins": 6,
                                    "losses": 12,
                                    "overtime_losses": 2,
                                    "win_pct": 0.3,
                                    "goals_for": 45,
                                    "goals_against": 86,
                                    "points": 14
                                },
                                {
                                    "record_type": "road",
                                    "wins": 6,
                                    "losses": 26,
                                    "overtime_losses": 4,
                                    "win_pct": 0.167,
                                    "goals_for": 77,
                                    "goals_against": 149,
                                    "points": 16
                                },
                                {
                                    "record_type": "west",
                                    "wins": 8,
                                    "losses": 23,
                                    "overtime_losses": 7,
                                    "win_pct": 0.211,
                                    "goals_for": 82,
                                    "goals_against": 151,
                                    "points": 23
                                }
                            ],
                            "rank": {
                                "division": 8,
                                "conference": 16,
                                "clinched": "eliminated",
                                "wildcard": 10
                            },
                            "calc_rank": {
                                "div_rank": 8,
                                "conf_rank": 16
                            }
                        }
                    ]
                }
            
        }
    ]
}"#;
