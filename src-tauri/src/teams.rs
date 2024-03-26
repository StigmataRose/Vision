pub struct NHLTeam {
    pub name: &'static str,
    pub id: &'static str,
    pub primary_color: &'static str,
    pub secondary_color: &'static str,
    pub logo: &'static str,
}

pub fn get_logo_by_id(id: String) -> String {
    for team in NHL_TEAMS {
        if team.id == id {
            return team.logo.to_string();
        }
    }
    // If no matching ID is found, return a default or placeholder string
    String::from("none") // Change this to whatever default logo you prefer
}

// #[tauri::command]
// pub fn get_logo_by_index(index: usize) -> Option<String> {
//     if let Some(team) = NHL_TEAMS.get(index) {
//         return Some(team.logo.to_string());
//     }
//     None
// }

pub fn get_id_by_index(index: usize) -> Option<String> {
    if let Some(team) = NHL_TEAMS.get(index) {
        return Some(team.id.to_string());
    }
    None

}

pub const NHL_TEAMS: &'static [NHLTeam] = &[
    NHLTeam {
        name: "Anaheim Ducks",
        id: "441862de-0f24-11e2-8525-18a905767e44",
        primary_color: "#F47A38",
        secondary_color: "#000000",
        logo: "assets/logos/anaheim-ducks-logo.png",
    },
    NHLTeam {
        name: "Arizona Coyotes",
        id: "44153da1-0f24-11e2-8525-18a905767e44",
        primary_color: "#8C2633",
        secondary_color: "#E2D6B5",
        logo: "assets/logos/arizona-coyotes-logo.png",
    },
    NHLTeam {
        name: "Boston Bruins",
        id: "4416ba1a-0f24-11e2-8525-18a905767e44",
        primary_color: "#FFB81C",
        secondary_color: "#000000",
        logo: "assets/logos/boston-bruins-logo.png",
    },
    NHLTeam {
        name: "Buffalo Sabres",
        id: "4416d559-0f24-11e2-8525-18a905767e44",
        primary_color: "#002654",
        secondary_color: "#FCB514",
        logo: "assets/logos/buffalo-sabres-logo.png",
    },
    NHLTeam {
        name: "Calgary Flames",
        id: "44159241-0f24-11e2-8525-18a905767e44",
        primary_color: "#C8102E",
        secondary_color: "#F1BE48",
        logo: "assets/logos/calgary-flames-logo.png",
    },
    NHLTeam {
        name: "Carolina Hurricanes",
        id: "44182a9d-0f24-11e2-8525-18a905767e44",
        primary_color: "#CC0000",
        secondary_color: "#000000",
        logo: "assets/logos/carolina-hurricanes-logo.png",
    },
    NHLTeam {
        name: "Chicago Blackhawks",
        id: "4416272f-0f24-11e2-8525-18a905767e44",
        primary_color: "#CF0A2C",
        secondary_color: "#000000",
        logo: "assets/logos/chicago-blackhawks-logo.png",
    },
    NHLTeam {
        name: "Colorado Avalanche",
        id: "4415ce44-0f24-11e2-8525-18a905767e44",
        primary_color: "#6F263D",
        secondary_color: "#236192",
        logo: "assets/logos/colorado-avalanche-logo.png",
    },
    NHLTeam {
        name: "Columbus Blue Jackets",
        id: "44167db4-0f24-11e2-8525-18a905767e44",
        primary_color: "#002654",
        secondary_color: "#CE1126",
        logo: "assets/logos/columbus-blue-jackets-logo.png",
    },
    NHLTeam {
        name: "Dallas Stars",
        id: "44157522-0f24-11e2-8525-18a905767e44",
        primary_color: "#006847",
        secondary_color: "#8F8F8C",
        logo: "assets/logos/dallas-stars-logo.png",
    },
    NHLTeam {
        name: "Detroit Red Wings",
        id: "44169bb9-0f24-11e2-8525-18a905767e44",
        primary_color: "#CE1126",
        secondary_color: "#FFFFFF",
        logo: "assets/logos/detroit-red-wings-logo.png",
    },
    NHLTeam {
        name: "Edmonton Oilers",
        id: "4415ea6c-0f24-11e2-8525-18a905767e44",
        primary_color: "#FF4C00",
        secondary_color: "#041E42",
        logo: "assets/logos/edmonton-oilers-logo.png",
    },
    NHLTeam {
        name: "Florida Panthers",
        id: "4418464d-0f24-11e2-8525-18a905767e44",
        primary_color: "#C8102E",
        secondary_color: "#041E42",
        logo: "assets/logos/florida-panthers-logo.png",
    },
    NHLTeam {
        name: "Los Angeles Kings",
        id: "44151f7a-0f24-11e2-8525-18a905767e44",
        primary_color: "#111111",
        secondary_color: "#A2AAAD",
        logo: "assets/logos/los-angeles-kings-logo.png",
    },
    NHLTeam {
        name: "Minnesota Wild",
        id: "4416091c-0f24-11e2-8525-18a905767e44",
        primary_color: "#154734",
        secondary_color: "#DDCBA4",
        logo: "assets/logos/minnesota-wild-logo.png",
    },
    NHLTeam {
        name: "Montreal Canadiens",
        id: "441713b7-0f24-11e2-8525-18a905767e44",
        primary_color: "#AF1E2D",
        secondary_color: "#192168",
        logo: "assets/logos/montreal-canadiens-logo.png",
    },
    NHLTeam {
        name: "Nashville Predators",
        id: "441643b7-0f24-11e2-8525-18a905767e44",
        primary_color: "#FFB81C",
        secondary_color: "#041E42",
        logo: "assets/logos/nashville-predators-logo.png",
    },
    NHLTeam {
        name: "New Jersey Devils",
        id: "44174b0c-0f24-11e2-8525-18a905767e44",
        primary_color: "#CE1126",
        secondary_color: "#000000",
        logo: "assets/logos/new-jersey-devils-logo.png",
    },
    NHLTeam {
        name: "New York Islanders",
        id: "441766b9-0f24-11e2-8525-18a905767e44",
        primary_color: "#00539B",
        secondary_color: "#F47D30",
        logo: "assets/logos/new-york-islanders-logo.png",
    },
    NHLTeam {
        name: "New York Rangers",
        id: "441781b9-0f24-11e2-8525-18a905767e44",
        primary_color: "#0038A8",
        secondary_color: "#CE1126",
        logo: "assets/logos/new-york-rangers-logo.png",
    },
    NHLTeam {
        name: "Ottawa Senators",
        id: "4416f5e2-0f24-11e2-8525-18a905767e44",
        primary_color: "#E31837",
        secondary_color: "#000000",
        logo: "assets/logos/ottawa-senators-logo.png",
    },
    NHLTeam {
        name: "Philadelphia Flyers",
        id: "44179d47-0f24-11e2-8525-18a905767e44",
        primary_color: "#F74902",
        secondary_color: "#000000",
        logo: "assets/logos/philadelphia-flyers-logo.png",
    },
    NHLTeam {
        name: "Pittsburgh Penguins",
        id: "4417b7d7-0f24-11e2-8525-18a905767e44",
        primary_color: "#FCB514",
        secondary_color: "#000000",
        logo: "assets/logos/pittsburgh-penguins-logo.png",
    },
    NHLTeam {
        name: "San Jose Sharks",
        id: "44155909-0f24-11e2-8525-18a905767e44",
        primary_color: "#006D75",
        secondary_color: "#EA7200",
        logo: "assets/logos/san-jose-sharks-logo.png",
    },
    NHLTeam {
        name: "Seattle Kraken",
        id: "1fb48e65-9688-4084-8868-02173525c3e1",
        primary_color: "#005A9C",
        secondary_color: "#FFA300",
        logo: "assets/logos/seattle-kraken-logo.png",
    },
    NHLTeam {
        name: "St. Louis Blues",
        id: "441660ea-0f24-11e2-8525-18a905767e44",
        primary_color: "#002F87",
        secondary_color: "#FCB514",
        logo: "assets/logos/st-louis-blues-logo.png",
    },
    NHLTeam {
        name: "Tampa Bay Lightning",
        id: "4417d3cb-0f24-11e2-8525-18a905767e44",
        primary_color: "#002868",
        secondary_color: "#FFFFFF",
        logo: "assets/logos/tampa-bay-lightning-logo.png",
    },
    NHLTeam {
        name: "Toronto Maple Leafs",
        id: "441730a9-0f24-11e2-8525-18a905767e44",
        primary_color: "#003E7E",
        secondary_color: "#FFFFFF",
        logo: "assets/logos/toronto-maple-leafs-logo.png",
    },
    NHLTeam {
        name: "Vancouver Canucks",
        id: "4415b0a7-0f24-11e2-8525-18a905767e44",
        primary_color: "#00205B",
        secondary_color: "#00843D",
        logo: "assets/logos/vancouver-canucks-logo.png",
    },
    NHLTeam {
        name: "Vegas Golden Knights",
        id: "42376e1c-6da8-461e-9443-cfcf0a9fcc4d",
        primary_color: "#B4975A",
        secondary_color: "#333F42",
        logo: "assets/logos/vegas-golden-knights-logo.png",
    },
    NHLTeam {
        name: "Washington Capitals",
        id: "4417eede-0f24-11e2-8525-18a905767e44",
        primary_color: "#041E42",
        secondary_color: "#C8102E",
        logo: "assets/logos/washington-capitals-logo.png",
    },
    NHLTeam {
        name: "Winnipeg Jets",
        id: "44180e55-0f24-11e2-8525-18a905767e44",
        primary_color: "#041E42",
        secondary_color: "#004C97",
        logo: "assets/logos/winnipeg-jets-logo.png",
    }];