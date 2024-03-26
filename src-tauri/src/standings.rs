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
 
 pub fn get_nhl_data() -> Result<Option<NHLData>, reqwest::Error> {
    let response = reqwest::blocking::get(api_url)?;
 
    let nhl_data: NHLData;
     // Check if the response was successful (status code 200)
    if response.status().is_success() {
        // Get the body of the response as text
        let body = response.text()?;
        nhl_data = serde_json::from_str(&body).unwrap();
 

  
       // println!("Response body: {}", body);
       Ok(Some(nhl_data))
        // You can parse the body as needed, depending on the content
    } else {
        Ok(None)
    }
 
 
}

/*
let data: NHLData = serde_json::from_str(JSON_DATA).unwrap();
 
  
 if let Some(data) = data.get("conferences") {
     println!("field = {:?}", data);
 } else {
     println!("field is missing");
 }
 
*/
