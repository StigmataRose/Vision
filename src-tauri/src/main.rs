// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
//mod League_Hierarchy;
//use League_Hierarchy::{get_nhl_hierarchy, get_nhl_conference_standings};

mod standings;
use std::string;

use standings::{get_nhl,get_nhl_teams,get_nhl_data,NHLData};

mod schedule;
use schedule::{get_nhl_schedule, get_games, GameData};

mod teams;
use teams::{NHL_TEAMS,get_logo_by_id};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


// #[tauri::command]
// pub fn get_logo_by_index(index: usize) -> Result<String, String> {
//     match NHL_TEAMS.get(index) {
//         Some(team) => Ok(team.logo.to_string()), // Convert &str to String
//         None => Err(format!("No team found with index {}", index)),
//     }
// }

#[tauri::command]
fn get_team_logo_by_index(index: usize) -> Result<String, String> {
    Ok("assets/logos/anaheim-ducks-logo.png".to_string())
}



fn main() {
    //let api_url = "https://api.sportradar.com/nhl/trial/v7/en/league/hierarchy.json?api_key=VNiIRM9bxpey8jVjjndD8kIAjgUutIx2gxn9L4J5";
    // match get_nhl_conference_standings(api_url) {
    //     Ok((conference1_teams, conference2_teams)) => {
    //         println!("Conference 1:");
    //         for team in conference1_teams {
    //             println!("  {}", team);
    //         }

    //         println!("Conference 2:");
    //         for team in conference2_teams {
    //             println!("  {}", team);
    //         }
    //     }
    //     Err(err) => eprintln!("Error: {}", err),
    // }

    // match get_nhl(){
    //     Ok(r) => {
    //         println!("");
    //     }
    //     Err(err) => eprintln!("Error: {}", err),
    // }

    // match get_nhl_teams(){
    //     Ok(r) => {
    //         println!("");
    //     }
    //     Err(err) => eprintln!("Error: {}", err),
    // }

    // let id = "441862de-0f24-11e2-8525-18a905767e44";
    // match get_logo_by_id(id) {
    //     Some(logo) => println!("Logo for team with id {}: {}", id, logo),
    //     None => println!("No team found with id {}", id),
    // }
    //build_schedule();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![build_schedule_struct])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
 .invoke_handler(tauri::generate_handler![get_nhl_teams])
*/


#[derive(Serialize)]
pub struct ScheduleTeam {
    pub id: String,
    pub name: String,
    pub division: String,
    pub division_rank: i32,
    pub logo: String,
    pub conference: String,
    pub conference_rank: i32,
    pub wins: i32,
    pub losses: i32,
    pub points: i32,
}

impl ScheduleTeam {
    fn new(id: &str, name: &str, division: &str, division_rank: i32, logo: &str, conference: &str, conference_rank: i32, wins: i32, losses: i32, points: i32) -> Self {
        ScheduleTeam {
            id: id.to_string(),
            name: name.to_string(),
            division: division.to_string(),
            division_rank,
            logo: logo.to_string(),
            conference: conference.to_string(),
            conference_rank,
            wins,
            losses,
            points,
        }
    }

    pub fn copy(&self) -> Self {
        ScheduleTeam {
            id: self.id.clone(),
            name: self.name.clone(),
            division: self.division.clone(),
            division_rank: self.division_rank,
            logo: self.logo.clone(),
            conference: self.conference.clone(),
            conference_rank: self.conference_rank,
            wins: self.wins,
            losses: self.losses,
            points: self.points,
        }
    }


}

#[derive(Serialize)]
pub struct ScheduleGame {
    pub time: String,
    pub network: String,
    pub home_team: ScheduleTeam,
    pub away_team: ScheduleTeam,
}

impl ScheduleGame {
     fn new(
        time: &str,
        network: &str,
        home_team: ScheduleTeam,
        away_team: ScheduleTeam,
    ) -> Self {
        ScheduleGame {
            time: time.to_string(),
            network: network.to_string(),
            home_team: home_team,
            away_team: away_team

        }
    }
}

#[derive(Serialize)]
pub struct Schedule {
    pub games: Vec<ScheduleGame>,
    pub played_games: Vec<ScheduleGame>,
}



 pub fn build_nhl_teams_schedule() -> Vec<ScheduleTeam> {

    let mut schedule_teams: Vec<ScheduleTeam> = Vec::new();

    match get_nhl_data() {
        Ok(Some(data)) => {

         
            // Create a new ScheduleTeam instance
           

            for conference in data.iter_conferences() {
         
                for division in conference.iter_divisions() {
             
                       for team in division.iter_teams() {
                        let logo_path = get_logo_by_id(team.id.clone());
                        
                        let new_team = ScheduleTeam::new(
                            &team.id,
                            &team.name,
                            &division.name,
                            team.rank.division,
                           &logo_path,
                            &conference.name,
                            team.rank.conference,
                            team.wins,
                            team.losses,
                            team.points,
                        ); 
                        
                        schedule_teams.push(new_team);
                    }
                 }
   
            }
           
            // NHL data is available
            //println!("NHL data: {:?}", data);
        }
        Ok(None) => {
            // NHL data is not available
           // println!("NHL data is not available.");
        }
        Err(err) => {
            // Handle error
           // eprintln!("Error: {:?}", err);
        }
    }
    return schedule_teams;
}


#[tauri::command]
fn build_schedule_struct() -> Schedule{
    build_nhl_teams_schedule();
    let nhl_data: GameData = match get_games() {
        Ok(games_result) => {
            match games_result {
                Some(nhl_data) => {
                    // Use nhl_data here
                  //  println!("Successfully obtained game data: {:?}", nhl_data);
                    nhl_data // Return the value from the inner match
                }
                None => {
                    // Handle the case where get_games() returned None
                   // println!("No game data available.");
                    // You might want to return early or handle this case differently
                    return Schedule{
                        games: Vec::new(),
                        played_games: Vec::new()
                    };
                }
            }
        }
        Err(err) => {
            // Handle the case where get_games() returned an error
          //  eprintln!("Error getting game data: {:?}", err);
            // You might want to return early or handle this case differently
            return Schedule{
                games: Vec::new(),
                played_games: Vec::new()
            };
        }
    };
    let schedule_teams = build_nhl_teams_schedule();
    let mut nhl_schedule: Schedule = Schedule{
        games: Vec::new(),
        played_games: Vec::new()
    };
    for game in &nhl_data.games {
        if game.status == "scheduled" {

            let mut networks = String::new();

            if let Some(broadcasts) = &game.broadcasts {
                for broadcast in broadcasts {
                    networks.push_str(&broadcast.network);
                    networks.push_str(", ");
                }
            }

            let mut home_team = ScheduleTeam {
                id: "team001".to_string(),
                name: "Team A".to_string(),
                division: "Division X".to_string(),
                division_rank: 1,
                logo: "logo_url".to_string(),
                conference: "Conference Y".to_string(),
                conference_rank: 2,
                wins: 10,
                losses: 5,
                points: 25,
            };
            let mut away_team = ScheduleTeam {
                id: "team001".to_string(),
                name: "Team A".to_string(),
                division: "Division X".to_string(),
                division_rank: 1,
                logo: "logo_url".to_string(),
                conference: "Conference Y".to_string(),
                conference_rank: 2,
                wins: 10,
                losses: 5,
                points: 25,
            };
            for teams in &schedule_teams {
                if teams.id == game.home.id {
                    let home_team1 = teams.copy();
                    home_team = home_team1;
                    break;
                }
                if teams.id == game.away.id {
                    let away_team1 = teams.copy();
                    away_team = away_team1;
                    break;
                }
            }
            
           let sched = game.scheduled.to_string();
           let gamer = ScheduleGame::new(&sched, &networks, home_team, away_team);

           nhl_schedule.games.push(gamer);
      
            } else {
                let mut networks = String::new();

                if let Some(broadcasts) = &game.broadcasts {
                    for broadcast in broadcasts {
                        networks.push_str(&broadcast.network);
                        networks.push_str(", ");
                    }
                }
    
                let mut home_team = ScheduleTeam {
                    id: "team001".to_string(),
                    name: "Team A".to_string(),
                    division: "Division X".to_string(),
                    division_rank: 1,
                    logo: "logo_url".to_string(),
                    conference: "Conference Y".to_string(),
                    conference_rank: 2,
                    wins: 10,
                    losses: 5,
                    points: 25,
                };
                let mut away_team = ScheduleTeam {
                    id: "team001".to_string(),
                    name: "Team A".to_string(),
                    division: "Division X".to_string(),
                    division_rank: 1,
                    logo: "logo_url".to_string(),
                    conference: "Conference Y".to_string(),
                    conference_rank: 2,
                    wins: 10,
                    losses: 5,
                    points: 25,
                };
                for teams in &schedule_teams {
                    if teams.id == game.home.id {
                        let home_team1 = teams.copy();
                        home_team = home_team1;
                        break;
                    }
                    if teams.id == game.away.id {
                        let away_team1 = teams.copy();
                        away_team = away_team1;
                        break;
                    }
                }
                
               let sched = game.scheduled.to_string();
               let gamer = ScheduleGame::new(&sched, &networks, home_team, away_team);
    
               nhl_schedule.played_games.push(gamer);
            }
        }

        for game in &nhl_data.games {
            println!("{} vs {} on {}", game.home.name, game.away.name, game.scheduled);

        }

        return nhl_schedule;
    }


