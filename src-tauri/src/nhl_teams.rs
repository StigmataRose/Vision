use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
fn get_team_abb_by_index(index: usize) -> String {
    match index {
        0 => String::from("ANA"),
        1 => String::from("ARI"),
        2 => String::from("BOS"),
        3 => String::from("BUF"),
        4 => String::from("CGY"),
        5 => String::from("CAR"),
        6 => String::from("CHI"),
        7 => String::from("COL"),
        8 => String::from("CBJ"),
        9 => String::from("DAL"),
        10 => String::from("DET"),
        11 => String::from("EDM"),
        12 => String::from("FLA"),
        13 => String::from("LAK"),
        14 => String::from("MIN"),
        15 => String::from("MTL"),
        16 => String::from("NSH"),
        17 => String::from("NJD"),
        18 => String::from("NYI"),
        19 => String::from("NYR"),
        20 => String::from("OTT"),
        21 => String::from("PHI"),
        22 => String::from("PIT"),
        23 => String::from("SJS"),
        24 => String::from("SEA"),
        25 => String::from("STL"),
        26 => String::from("TBL"),
        27 => String::from("TOR"),
        28 => String::from("VAN"),
        29 => String::from("VGK"),
        30 => String::from("WSH"),
        31 => String::from("WPG"),
        _ => String::from(""),
    }
}


fn get_team_name_by_index(index: usize) -> String {
    match index {
        0 => String::from("Anaheim Ducks"),
        1 => String::from("Arizona Coyotes"),
        2 => String::from("Boston Bruins"),
        3 => String::from("Buffalo Sabres"),
        4 => String::from("Calgary Flames"),
        5 => String::from("Carolina Hurricanes"),
        6 => String::from("Chicago Blackhawks"),
        7 => String::from("Colorado Avalanche"),
        8 => String::from("Columbus Blue Jackets"),
        9 => String::from("Dallas Stars"),
        10 => String::from("Detroit Red Wings"),
        11 => String::from("Edmonton Oilers"),
        12 => String::from("Florida Panthers"),
        13 => String::from("Los Angeles Kings"),
        14 => String::from("Minnesota Wild"),
        15 => String::from("Montréal Canadiens"),
        16 => String::from("Nashville Predators"),
        17 => String::from("New Jersey Devils"),
        18 => String::from("New York Islanders"),
        19 => String::from("New York Rangers"),
        20 => String::from("Ottawa Senators"),
        21 => String::from("Philadelphia Flyers"),
        22 => String::from("Pittsburgh Penguins"),
        23 => String::from("San Jose Sharks"),
        24 => String::from("Seattle Kraken"),
        25 => String::from("St. Louis Blues"),
        26 => String::from("Tampa Bay Lightning"),
        27 => String::from("Toronto Maple Leafs"),
        28 => String::from("Vancouver Canucks"),
        29 => String::from("Vegas Golden Knights"),
        30 => String::from("Washington Capitals"),
        31 => String::from("Winnipeg Jets"),
        _ => String::from(""),
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: u32,
    pub headshot: String,
    pub firstName: Name,
    pub lastName: Name,
    pub sweaterNumber: Option<u32>,
    pub positionCode: String,
    pub shootsCatches: String,
    pub heightInInches: u32,
    pub weightInPounds: u32,
    pub heightInCentimeters: u32,
    pub weightInKilograms: u32,
    pub birthDate: String,
    pub birthCity: Option<City>,
    pub birthCountry: String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Name {
    pub default: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    pub default: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateProvince {
    pub default: String,
    #[serde(default)]
    pub fr: Option<String>,
    #[serde(default)]
    pub sk: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamRoster {
    pub forwards: Vec<Player>,
    pub defensemen: Vec<Player>,
    pub goalies: Vec<Player>,
}

pub fn get_roster_by_index(index: usize) -> Result<TeamRoster, reqwest::Error> {

let base_url = "https://api-web.nhle.com/v1/roster/";
let team_code = get_team_abb_by_index(index);
let suffix = "/current";

let full_url = format!("{}{}{}", base_url, team_code, suffix);

let response = reqwest::blocking::get(full_url)?;
 
 
    if response.status().is_success() {
    // Get the body of the response as text
    let body = response.text()?;
    let nhl_roster_data: TeamRoster = serde_json::from_str(&body).unwrap();

        Ok((nhl_roster_data))
}else{
    // grab local copy
    let nhl_roster_data_empty: TeamRoster = TeamRoster {
        forwards: Vec::new(),
        defensemen: Vec::new(),
        goalies: Vec::new(),
    };
    println!("Roster Data: Error: 146 not found");
    Ok((nhl_roster_data_empty))
}

}

pub fn return_roster(index: usize) -> TeamRoster {
    // Call get_roster_by_index to fetch the roster
    match get_roster_by_index(index) {
        Ok(roster) => roster, // Return the roster if it's successfully retrieved
        Err(err) => {
            // Handle the error if roster retrieval fails
            eprintln!("Error retrieving roster: {}", err);
            // Return an empty roster or handle the error in another way
            TeamRoster {
                forwards: vec![],
                defensemen: vec![],
                goalies: vec![],
            }
        }
    }
}

// this is for the club info struct
#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerName {
    pub default: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Skater {
    pub playerId: u32,
    pub headshot: String,
    pub firstName: PlayerName,
    pub lastName: PlayerName,
    pub positionCode: String,
    pub gamesPlayed: u32,
    pub goals: u32,
    pub assists: u32,
    pub points: u32,
    pub plusMinus: i32,
    pub penaltyMinutes: u32,
    pub powerPlayGoals: u32,
    pub shorthandedGoals: u32,
    pub gameWinningGoals: u32,
    pub overtimeGoals: u32,
    pub shots: u32,
    pub shootingPctg: f64,
    pub avgTimeOnIcePerGame: f64,
    pub avgShiftsPerGame: f64,
    pub faceoffWinPctg: f64,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Goalie {
    pub playerId: u32,
    pub headshot: String,
    pub firstName: PlayerName,
    pub lastName: PlayerName,
    pub gamesPlayed: u32,
    pub gamesStarted: u32,
    pub wins: u32,
    pub losses: u32,
    pub ties: u32,
    pub overtimeLosses: u32,
    pub goalsAgainstAverage: f64,
    pub savePercentage: f64,
    pub shotsAgainst: u32,
    pub saves: u32,
    pub goalsAgainst: u32,
    pub shutouts: u32,
    pub goals: u32,
    pub assists: u32,
    pub points: u32,
    pub penaltyMinutes: u32,
    pub timeOnIce: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonStats {
    pub season: String,
    pub gameType: u32,
    pub skaters: Vec<Skater>,
    pub goalies: Vec<Goalie>,
}


pub fn get_season_stats_by_index(index: usize) -> Result<SeasonStats, reqwest::Error> {

    let base_url = "https://api-web.nhle.com/v1/club-stats/";
    let team_code = get_team_abb_by_index(index);
    let suffix = "/now";
    
    let full_url = format!("{}{}{}", base_url, team_code, suffix);
    
    let response = reqwest::blocking::get(full_url)?;
     
     
        if response.status().is_success() {
        // Get the body of the response as text
        let body = response.text()?;
        let nhl_roster_data: SeasonStats = serde_json::from_str(&body).unwrap();
    
        // for player in &nhl_roster_data.forwards {
        //     println!("Forward: {} {}", player.firstName.default, player.lastName.default);
        // }
    
        // for player in &nhl_roster_data.defensemen {
        //     println!("Defenseman: {} {}", player.firstName.default, player.lastName.default);
        // }
    
        // for player in &nhl_roster_data.goalies {
        //     println!("Goalie: {} {}", player.firstName.default, player.lastName.default);
        // }
    
            Ok((nhl_roster_data))
    }else{
        // grab local copy
        let nhl_roster_data_empty: SeasonStats = SeasonStats {
            season: String::new(),
            gameType: 0, // Assuming a default value for game type
            skaters: Vec::new(),
            goalies: Vec::new(),
        };
        println!("Roster Data: Error: 146 not found");
        Ok((nhl_roster_data_empty))
    }
    
    }


    pub fn return_season_stats(index: usize) -> SeasonStats {
        // Call get_roster_by_index to fetch the roster
        match get_season_stats_by_index(index) {
            Ok(season) => season, // Return the roster if it's successfully retrieved
            Err(err) => {
                // Handle the error if roster retrieval fails
                eprintln!("Error retrieving roster: {}", err);
                // Return an empty roster or handle the error in another way
                SeasonStats {
                    season: String::default(),
                    gameType: 0, // Provide default value for gameType
                    skaters: Vec::new(), // Empty Vec for skaters
                    goalies: Vec::new(), // Empty Vec for goalies
                }
            }
        }
    }