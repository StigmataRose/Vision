use serde::{Deserialize, Serialize};
use std::cmp::Reverse;

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamName {
    pub default: String,
    pub fr: Option<String>,
}
impl TeamName {
    pub fn copy(&self) -> TeamName {
        TeamName {
            default: self.default.to_string(),
            fr: self.fr.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaceName {
    pub default: String,
}

impl PlaceName {
    pub fn copy(&self) -> PlaceName {
        PlaceName {
            default: self.default.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Standings {
    pub conferenceAbbrev: String,
    pub conferenceHomeSequence: i32,
    pub conferenceL10Sequence: i32,
    pub conferenceName: String,
    pub conferenceRoadSequence: i32,
    pub conferenceSequence: i32,
    pub date: String,
    pub divisionAbbrev: String,
    pub divisionHomeSequence: i32,
    pub divisionL10Sequence: i32,
    pub divisionName: String,
    pub divisionRoadSequence: i32,
    pub divisionSequence: i32,
    pub gameTypeId: i32,
    pub gamesPlayed: i32,
    pub goalDifferential: i32,
    pub goalDifferentialPctg: f64,
    pub goalAgainst: i32,
    pub goalFor: i32,
    pub goalsForPctg: f64,
    pub homeGamesPlayed: i32,
    pub homeGoalDifferential: i32,
    pub homeGoalsAgainst: i32,
    pub homeGoalsFor: i32,
    pub homeLosses: i32,
    pub homeOtLosses: i32,
    pub homePoints: i32,
    pub homeRegulationPlusOtWins: i32,
    pub homeRegulationWins: i32,
    pub homeTies: i32,
    pub homeWins: i32,
    pub l10GamesPlayed: i32,
    pub l10GoalDifferential: i32,
    pub l10GoalsAgainst: i32,
    pub l10GoalsFor: i32,
    pub l10Losses: i32,
    pub l10OtLosses: i32,
    pub l10Points: i32,
    pub l10RegulationPlusOtWins: i32,
    pub l10RegulationWins: i32,
    pub l10Ties: i32,
    pub l10Wins: i32,
    pub leagueHomeSequence: i32,
    pub leagueL10Sequence: i32,
    pub leagueRoadSequence: i32,
    pub leagueSequence: i32,
    pub losses: i32,
    pub otLosses: i32,
    pub placeName: PlaceName,
    pub pointPctg: f64,
    pub points: i32,
    pub regulationPlusOtWinPctg: f64,
    pub regulationPlusOtWins: i32,
    pub regulationWinPctg: f64,
    pub regulationWins: i32,
    pub roadGamesPlayed: i32,
    pub roadGoalDifferential: i32,
    pub roadGoalsAgainst: i32,
    pub roadGoalsFor: i32,
    pub roadLosses: i32,
    pub roadOtLosses: i32,
    pub roadPoints: i32,
    pub roadRegulationPlusOtWins: i32,
    pub roadRegulationWins: i32,
    pub roadTies: i32,
    pub roadWins: i32,
    pub seasonId: i32,
    pub shootoutLosses: i32,
    pub shootoutWins: i32,
    pub streakCode: String,
    pub streakCount: i32,
    pub teamName: TeamName,
    pub teamCommonName: PlaceName,
    pub teamAbbrev: PlaceName,
    pub teamLogo: String,
    pub ties: i32,
    pub waiversSequence: i32,
    pub wildcardSequence: i32,
    pub winPctg: f64,
    pub wins: i32,
}

impl Standings {
    pub fn copy(&self) -> Standings {
        Standings {
            conferenceAbbrev: self.conferenceAbbrev.to_string(),
            conferenceHomeSequence: self.conferenceHomeSequence,
            conferenceL10Sequence: self.conferenceL10Sequence,
            conferenceName: self.conferenceName.to_string(),
            conferenceRoadSequence: self.conferenceRoadSequence,
            conferenceSequence: self.conferenceSequence,
            date: self.date.to_string(),
            divisionAbbrev: self.divisionAbbrev.to_string(),
            divisionHomeSequence: self.divisionHomeSequence,
            divisionL10Sequence: self.divisionL10Sequence,
            divisionName: self.divisionName.to_string(),
            divisionRoadSequence: self.divisionRoadSequence,
            divisionSequence: self.divisionSequence,
            gameTypeId: self.gameTypeId,
            gamesPlayed: self.gamesPlayed,
            goalDifferential: self.goalDifferential,
            goalDifferentialPctg: self.goalDifferentialPctg,
            goalAgainst: self.goalAgainst,
            goalFor: self.goalFor,
            goalsForPctg: self.goalsForPctg,
            homeGamesPlayed: self.homeGamesPlayed,
            homeGoalDifferential: self.homeGoalDifferential,
            homeGoalsAgainst: self.homeGoalsAgainst,
            homeGoalsFor: self.homeGoalsFor,
            homeLosses: self.homeLosses,
            homeOtLosses: self.homeOtLosses,
            homePoints: self.homePoints,
            homeRegulationPlusOtWins: self.homeRegulationPlusOtWins,
            homeRegulationWins: self.homeRegulationWins,
            homeTies: self.homeTies,
            homeWins: self.homeWins,
            l10GamesPlayed: self.l10GamesPlayed,
            l10GoalDifferential: self.l10GoalDifferential,
            l10GoalsAgainst: self.l10GoalsAgainst,
            l10GoalsFor: self.l10GoalsFor,
            l10Losses: self.l10Losses,
            l10OtLosses: self.l10OtLosses,
            l10Points: self.l10Points,
            l10RegulationPlusOtWins: self.l10RegulationPlusOtWins,
            l10RegulationWins: self.l10RegulationWins,
            l10Ties: self.l10Ties,
            l10Wins: self.l10Wins,
            leagueHomeSequence: self.leagueHomeSequence,
            leagueL10Sequence: self.leagueL10Sequence,
            leagueRoadSequence: self.leagueRoadSequence,
            leagueSequence: self.leagueSequence,
            losses: self.losses,
            otLosses: self.otLosses,
            placeName: self.placeName.copy(),
            pointPctg: self.pointPctg,
            points: self.points,
            regulationPlusOtWinPctg: self.regulationPlusOtWinPctg,
            regulationPlusOtWins: self.regulationPlusOtWins,
            regulationWinPctg: self.regulationWinPctg,
            regulationWins: self.regulationWins,
            roadGamesPlayed: self.roadGamesPlayed,
            roadGoalDifferential: self.roadGoalDifferential,
            roadGoalsAgainst: self.roadGoalsAgainst,
            roadGoalsFor: self.roadGoalsFor,
            roadLosses: self.roadLosses,
            roadOtLosses: self.roadOtLosses,
            roadPoints: self.roadPoints,
            roadRegulationPlusOtWins: self.roadRegulationPlusOtWins,
            roadRegulationWins: self.roadRegulationWins,
            roadTies: self.roadTies,
            roadWins: self.roadWins,
            seasonId: self.seasonId,
            shootoutLosses: self.shootoutLosses,
            shootoutWins: self.shootoutWins,
            streakCode: self.streakCode.to_string(),
            streakCount: self.streakCount,
            teamName: self.teamName.copy(),
            teamCommonName: self.teamCommonName.copy(),
            teamAbbrev: self.teamAbbrev.copy(),
            teamLogo: self.teamLogo.to_string(),
            ties: self.ties,
            waiversSequence: self.waiversSequence,
            wildcardSequence: self.wildcardSequence,
            winPctg: self.winPctg,
            wins: self.wins,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandingsData {
    wildCardIndicator: bool,
    standings: Vec<Standings>,
}

impl StandingsData {
    pub fn new(wild_card_indicator: bool, standings: Vec<Standings>) -> Self {
        StandingsData {
            wildCardIndicator: wild_card_indicator,
            standings: standings,
        }
    }
}


pub fn get_nhl_schedule() -> Result<StandingsData, reqwest::Error> {

    let response = reqwest::blocking::get("https://api-web.nhle.com/v1/standings/now")?;
 
 
    if response.status().is_success() {
    // Get the body of the response as text
    let body = response.text()?;
    let nhl_standings_data: StandingsData = serde_json::from_str(&body).unwrap();
        Ok((nhl_standings_data))
}else{
    // grab local copy
    let nhl_standings_data_empty: StandingsData = StandingsData::new(false, Vec::new());

    println!("Standings Data: Error: 112 not found");
    Ok((nhl_standings_data_empty))
}

}

pub fn get_divisions() -> (Vec<Standings>, Vec<Standings>, Vec<Standings>, Vec<Standings>) {

    let mut vecC: Vec<Standings> = Vec::new();
    let mut vecP: Vec<Standings> = Vec::new();
    let mut vecA: Vec<Standings> = Vec::new();
    let mut vecM: Vec<Standings> = Vec::new();

    match get_nhl_schedule() {
        Ok(data) => {
            // Successfully retrieved NHL schedule data
            // You can access the data here
           // println!("NHL standings data:");
            for standings in &data.standings {
                if standings.divisionAbbrev == "C"{
                    vecC.push(standings.copy());
                }
                if standings.divisionAbbrev == "P"{
                    vecP.push(standings.copy());
                }
                if standings.divisionAbbrev == "A"{
                    vecA.push(standings.copy());
                }
                if standings.divisionAbbrev == "M"{
                    vecM.push(standings.copy());
                }
            }
        }
        Err(e) => {
            // Handle the error
            println!("Error retrieving NHL schedule: {:?}", e);
        }
    }

    vecC.sort_by_key(|s| Reverse(s.points));
    vecP.sort_by_key(|s| Reverse(s.points));
    vecA.sort_by_key(|s| Reverse(s.points));
    vecM.sort_by_key(|s| Reverse(s.points));

    (vecC, vecP, vecA, vecM)
}

pub fn get_league_standings() -> (Vec<Standings>) {

    let mut league: Vec<Standings> = Vec::new();
  

    match get_nhl_schedule() {
        Ok(data) => {
            // Successfully retrieved NHL schedule data
            // You can access the data here
           // println!("NHL standings data:");
            for standings in &data.standings {
                league.push(standings.copy());
            }
        }
        Err(e) => {
            // Handle the error
            println!("Error retrieving NHL schedule: {:?}", e);
        }
    }

    league.sort_by_key(|s| Reverse(s.points));
    
    (league)
}

pub fn get_home_standings() -> (Vec<Standings>) {

    let mut league: Vec<Standings> = Vec::new();
  

    match get_nhl_schedule() {
        Ok(data) => {
            // Successfully retrieved NHL schedule data
            // You can access the data here
           // println!("NHL standings data:");
            for standings in &data.standings {
                league.push(standings.copy());
            }
        }
        Err(e) => {
            // Handle the error
            println!("Error retrieving NHL schedule: {:?}", e);
        }
    }

    league.sort_by_key(|s| Reverse(s.homeWins));
    
    (league)
}

pub fn get_away_standings() -> (Vec<Standings>) {

    let mut league: Vec<Standings> = Vec::new();
  

    match get_nhl_schedule() {
        Ok(data) => {
            // Successfully retrieved NHL schedule data
            // You can access the data here
           // println!("NHL standings data:");
            for standings in &data.standings {
                league.push(standings.copy());
            }
        }
        Err(e) => {
            // Handle the error
            println!("Error retrieving NHL schedule: {:?}", e);
        }
    }

    league.sort_by_key(|s| Reverse(s.roadWins));
    
    (league)
}