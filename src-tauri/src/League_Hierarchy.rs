use reqwest::{Error, Client};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Venue {
    id: String,
    name: String,
    capacity: u32,
    address: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    time_zone: String,
    sr_id: String,
}

#[derive(Debug, Deserialize)]
struct Team {
    id: String,
    name: String,
    market: String,
    alias: String,
    sr_id: String,
    reference: String,
    venue: Venue,
}

#[derive(Debug, Deserialize)]
struct Division {
    id: String,
    name: String,
    alias: String,
    teams: Vec<Team>,
}

#[derive(Debug, Deserialize)]
struct Conference {
    id: String,
    name: String,
    alias: String,
    divisions: Vec<Division>,
}

#[derive(Debug, Deserialize)]
struct LeagueHierarchy {
    league: League,
    conferences: Vec<Conference>,
}

#[derive(Debug, Deserialize)]
struct League {
    id: String,
    name: String,
    alias: String,
}


pub fn get_nhl_hierarchy(api_url: &str) -> Result<(), reqwest::Error> {
    // Make the GET request
    let response = reqwest::blocking::get(api_url)?;

    // Parse JSON response
    let hierarchy: LeagueHierarchy = response.json()?;


   
    // Access the data
    println!("League Name: {}", hierarchy.league.name);
    println!("Conferences:");

    for conference in hierarchy.conferences {
        println!("  {}", conference.name);
        for division in conference.divisions {
            println!("    {}", division.name);
            for team in division.teams {
                println!("       {} {}", team.name, team.market);
                println!("       {}", team.id.as_str());
            }
        }
    }

    Ok(())
}

pub fn get_nhl_conference_standings(api_url: &str) -> Result<(Vec<String>, Vec<String>), reqwest::Error> {
    // Make the GET request
    let response = reqwest::blocking::get(api_url)?;

    // Parse JSON response
    let hierarchy: LeagueHierarchy = response.json()?;

    // Create vectors to store teams for each conference
    let mut conference1_teams: Vec<String> = Vec::new();
    let mut conference2_teams: Vec<String> = Vec::new();


    // Iterate over conferences and collect team names into vectors based on conference
    for (i, conference) in hierarchy.conferences.into_iter().enumerate() {
        let mut teams_for_conference: Vec<String> = Vec::new();
        for division in conference.divisions {
            for team in division.teams {
                teams_for_conference.push(format!("{} {}", team.name, team.market));
            }
        }
        // Push the teams into respective conference vectors
        match i {
            0 => conference1_teams.extend(teams_for_conference),
            1 => conference2_teams.extend(teams_for_conference),
            _ => {}
        }
    }

    // Return the tuple of vectors
    Ok((conference1_teams, conference2_teams))
}