// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

mod nhl_standings;
use nhl_standings::{get_nhl_schedule, get_divisions, get_league_standings, get_home_standings, get_away_standings, Standings, TeamName, PlaceName};

mod nhl_teams;
use nhl_teams::{return_roster,return_season_stats,SeasonStats,TeamRoster};



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


pub fn window_invoke_responder<R: tauri::Runtime>(
    window: tauri::Window<R>,
    response: tauri::InvokeResponse,
    success_callback: tauri::api::ipc::CallbackFn,
    error_callback: tauri::api::ipc::CallbackFn,
) {
    let callback_string = match tauri::api::ipc::format_callback_result(
        response.into_result(),
        success_callback,
        error_callback,
    ) {
        Ok(callback_string) => callback_string,
        Err(e) => tauri::api::ipc::format_callback(error_callback, &e.to_string())
            .expect("unable to serialize response string to json"),
    };

    let _ = window.eval(&callback_string);
}

/// Custom iframe-aware initialization script
#[rustfmt::skip]
fn invoke_initialization_script() -> String {
    const STRINGIFY_IPC_MESSAGE_FN: &str = "JSON.stringify";

    // Custom iframe transformCallback proxy based on `core/tauri/scripts/core.js`
    let add_iframe_proxy = format!(r#"
        // Check if invoked within iframe
        if (window.frameElement) {{
          const {{callback, error}} = message;
          
          // Add proxies from parent window to child window
          // as-if transformCallback() was called on parent
          [callback, error].map(id => `_${{id}}`).map(prop =>
            Object.defineProperty(window.parent, prop, {{
              value: (result) => {{
                // Remove proxies from parent
                Reflect.deleteProperty(window.parent, `_${{error}}`)
                Reflect.deleteProperty(window.parent, `_${{callback}}`)
                // Proxy result to child
                return window[prop](result)
              }},
              writable: false,
              configurable: true
            }})
          )
        }}
    "#);

    // Vanilla invoke initialization script from `core/tauri/src/app.rs`
    format!(r#"
    Object.defineProperty(window, '__TAURI_POST_MESSAGE__', {{
      value: (message) => {{
        {add_iframe_proxy}
        window.ipc.postMessage({STRINGIFY_IPC_MESSAGE_FN}(message))
      }}
    }})
    "#)
    .to_string()
}
#[derive(Debug, Deserialize, Serialize)]
struct Divisions{
    central: Vec<Standings>,
    pacific: Vec<Standings>,
    atlantic: Vec<Standings>,
    metropolitan: Vec<Standings>,

}

use tauri::Error;
#[tauri::command]
fn return_divisions() -> Result<Divisions, Error> {
    // Assuming get_divisions() returns a tuple of four vectors
    let (central, pacific, atlantic, metropolitan) = get_divisions();

    Ok(Divisions {
        central,
        pacific,
        atlantic,
        metropolitan,
    })
}

#[tauri::command]
fn return_league_standings() -> Result<Vec<Standings>, Error> {
    // Assuming get_divisions() returns a tuple of four vectors
    let league = get_league_standings();

    Ok(league)
}

#[tauri::command]
fn return_home_standings() -> Result<Vec<Standings>, Error> {
    // Assuming get_divisions() returns a tuple of four vectors
    let league = get_home_standings();
    Ok(league)
}

#[tauri::command]
fn return_away_standings() -> Result<Vec<Standings>, Error> {
    // Assuming get_divisions() returns a tuple of four vectors
    let league = get_away_standings();
    Ok(league)
}

#[tauri::command]
fn get_team_roster_by_index(index: usize) -> Result<TeamRoster, Error> {
    let team_roster = return_roster(index);
    Ok(team_roster)
}

#[tauri::command]
fn get_season_stats_by_index(index: usize) -> Result<SeasonStats, Error> {
    let season = return_season_stats(index);
    Ok(season)
}


fn main() {

    tauri::Builder::default()
    .invoke_handler(
        tauri::generate_handler![
            return_divisions,
            return_league_standings,
            return_home_standings,
            return_away_standings,
            get_team_roster_by_index,
            get_season_stats_by_index
        ]
    )
    .invoke_system(invoke_initialization_script(), window_invoke_responder)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
