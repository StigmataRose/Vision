// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod League_Hierarchy;
//use League_Hierarchy::{get_nhl_hierarchy, get_nhl_conference_standings};

mod standings;
use standings::{get_nhl,get_nhl_teams};

mod schedule;
use schedule::get_nhl_schedule;

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

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_team_logo_by_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
 .invoke_handler(tauri::generate_handler![get_nhl_teams])
*/