const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}


window.onload = () => {
  // Call the Rust function to get the string array
  window.__TAURI__.invoke('get_nhl_teams').then((stringArray) => {
      // Assume you have a div with id "stringArrayContainer" in your HTML
      const container = document.getElementById('stringArrayContainer');
      
      // Clear previous content
      container.innerHTML = '';

      // Loop through the array and create HTML elements to display each string
      stringArray.forEach((str) => {
          const p = document.createElement('p');
          p.textContent = str;
          container.appendChild(p);
      });
  });
};

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

function getFileForIframe(condition) {
  // Define file paths based on condition
  switch (condition) {
      case 'standings':
          return 'standings.html';
      case 'leaders':
          return 'leaders.html';
      case 'playoffs':
          return 'playoffs.html';
      case 'matchup':
          return 'matchup.html';
      default:
          return ''; // Default to empty string or handle error condition
  }
}


function getNHLTeamColors(teamName) {
  const teamColors = {
      "Anaheim Ducks": ["#F47A38", "#000000"],
      "Arizona Coyotes": ["#8C2633", "#E2D6B5"],
      "Boston Bruins": ["#FFB81C", "#000000"],
      "Buffalo Sabres": ["#002654", "#FCB514"],
      "Calgary Flames": ["#C8102E", "#F1BE48"],
      "Carolina Hurricanes": ["#CC0000", "#000000"],
      "Chicago Blackhawks": ["#CF0A2C", "#000000"],
      "Colorado Avalanche": ["#6F263D", "#236192"],
      "Columbus Blue Jackets": ["#002654", "#CE1126"],
      "Dallas Stars": ["#006847", "#8F8F8C"],
      "Detroit Red Wings": ["#CE1126", "#FFFFFF"],
      "Edmonton Oilers": ["#FF4C00", "#041E42"],
      "Florida Panthers": ["#C8102E", "#041E42"],
      "Los Angeles Kings": ["#111111", "#A2AAAD"],
      "Minnesota Wild": ["#154734", "#DDCBA4"],
      "Montreal Canadiens": ["#AF1E2D", "#192168"],
      "Nashville Predators": ["#FFB81C", "#041E42"],
      "New Jersey Devils": ["#CE1126", "#000000"],
      "New York Islanders": ["#00539B", "#F47D30"],
      "New York Rangers": ["#0038A8", "#CE1126"],
      "Ottawa Senators": ["#E31837", "#000000"],
      "Philadelphia Flyers": ["#F74902", "#000000"],
      "Pittsburgh Penguins": ["#FCB514", "#000000"],
      "San Jose Sharks": ["#006D75", "#EA7200"],
      "Seattle Kraken": ["#005A9C", "#FFA300"],
      "St. Louis Blues": ["#002F87", "#FCB514"],
      "Tampa Bay Lightning": ["#002868", "#FFFFFF"],
      "Toronto Maple Leafs": ["#003E7E", "#FFFFFF"],
      "Vancouver Canucks": ["#00205B", "#00843D"],
      "Vegas Golden Knights": ["#B4975A", "#333F42"],
      "Washington Capitals": ["#041E42", "#C8102E"],
      "Winnipeg Jets": ["#041E42", "#004C97"]
  };

  return teamColors[teamName] || [];
}



  // Get all the team buttons
  var teamButtons = document.querySelectorAll(".team-button");
  var index = 0;

  // Set an event listener for each button
  teamButtons.forEach(function(button) {
    var buttonIndex = index + 1;
      // Remove shadow, border, and outline from the button image when clicked
      button.addEventListener("click", function() {
          var img = button.querySelector("img");
          teamButtons.forEach(function(btn) {
                var img = btn.querySelector("img");
                img.style.boxShadow = "none";
                img.style.border = "none";
                img.style.outline = "none";
            });
          // Set initial styles for the image
          img.style.boxShadow = "0 0 10px rgba(0, 0, 0, 0.5)";
          //img.style.border = "2px solid transparent";
          img.style.outline = "2px solid rgba(0, 0, 0, 1.0)";
          img.style.outlineOffset = "-4px";
          index = buttonIndex;
         // document.getElementById("indexDisplay").innerText = "Index: " + index;
      });
  });


    function toggleMenu() {
        var menuToggle = document.getElementById('menu-toggle');

        if (menuToggle.classList.contains('visible')) {
            // Hide menu
            menuToggle.classList.remove('visible');
            // Move menu-toggle back
            menuToggle.style.transform = 'translateX(0)';
            // Change text
            menuToggle.innerHTML = '&#9776; <strong>Menu</strong>'; // "&#9776;" is the hamburger symbol
        } else {
            // Show menu
            menuToggle.classList.add('visible');
            // Move menu-toggle
            menuToggle.style.transform = 'translateX(250px)';
            // Change text
            menuToggle.innerHTML = '&#10006; <strong>Close</strong>'; // "&#10006;" is the close symbol
        }
    }

    async function getTeamLogoByIndex(index) {
      try {
        const logoPath = await invoke('get_team_logo_by_index',{index: 0});
        var image = document.getElementById("myImage");
        // Change the src attribute
        image.src = logoPath;
      } catch (error) {
        console.error('Error while getting team logo:', error);
      }
    }
    
    
    //getTeamLogoByIndex({index: 0});