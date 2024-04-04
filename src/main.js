
const invoke = window.__TAURI__.invoke
window.onload = () => {
  //fetchSchedule(); // Call fetchStruct function when window loads
  //alert('Right Click Folders to remove notifications');
};

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
    img.style.outline = "2px solid rgba(0, 0, 0, 1.0)";
    img.style.outlineOffset = "-4px";
    index = buttonIndex;
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
    const logoPath = await invoke('get_team_logo_by_index', { index: 0 });
    var image = document.getElementById("myImage");
    // Change the src attribute
    image.src = logoPath;
  } catch (error) {
    console.error('Error while getting team logo:', error);
  }
}


// Create a 3x3 table
function createTable(index) {
  var table = document.createElement('table');
  table.id = 'gameTable';

  var tbody = document.createElement('tbody');
  table.appendChild(tbody);

  for (var i = 0; i < 3; i++) {
      var row = document.createElement('tr');

      for (var j = 0; j < 3; j++) {
          var cell = document.createElement('td');
          var cellNum = (i * 3) + (j + 1);

          cell.className = 'box' + cellNum;
          row.appendChild(cell);

          if (cellNum === 1) {
              cell.style.textAlign = "left";
              cell.textContent = scheduled.games[index].time;
              //console.log(game.time); // Output game.time to the console
          }
          if (cellNum === 3) {
              cell.style.textAlign = "right";
              cell.textContent = scheduled.games[index].network;
          }
          if (cellNum === 4) {
              cell.style.textAlign = "center";
              cell.appendChild(createTeamInfoDiv('HOME',scheduled.games[index].home_team));
          }
          if (cellNum === 5) {
              cell.style.textAlign = "center";
              cell.textContent = "vs";
          }
          if (cellNum === 6) {
              cell.style.textAlign = "center";
              cell.appendChild(createTeamInfoDiv('AWAY',scheduled.games[index].away_team));
          }
          if (cellNum === 7) {
              cell.style.textAlign = "center";
              cell.textContent = String(scheduled.games[index].home_team.points);
          }
          if (cellNum === 8) {
            cell.style.textAlign = "center";
            cell.textContent = "Season Points" ;
        }
          if (cellNum === 9) {
              cell.style.textAlign = "center";
              cell.textContent = String(scheduled.games[index].away_team.points);
          }
      }

      tbody.appendChild(row);
  }

  return table;
}

function createTeamInfoDiv(team_name, the_team) {
  var teamInfoDiv = document.createElement('div');
  teamInfoDiv.className = 'team-info';

  var teamNameDiv = document.createElement('div');
  teamNameDiv.className = 'team-name';
  teamNameDiv.textContent = the_team.name;

  var innerDiv = createInnerContent(the_team);
  innerDiv.className = 'inner';

  var recordDiv = document.createElement('div');
  recordDiv.className = 'record';
  recordDiv.textContent = String(the_team.wins) + "- " + String(the_team.losses);

  teamInfoDiv.appendChild(teamNameDiv);
  teamInfoDiv.appendChild(innerDiv);
  teamInfoDiv.appendChild(recordDiv);

  return teamInfoDiv;
}

function createInnerContent(the_team) {
  var innerDiv = document.createElement('div');
  innerDiv.className = 'inner';
  innerDiv.style.display = 'inline-block';
  innerDiv.style.margin = 'auto';

  var divisionColumn = document.createElement('div');
  divisionColumn.className = 'division';
  divisionColumn.textContent = the_team.division;

  var logoColumn = document.createElement('div');
  logoColumn.className = 'logo';
  var logoImg = document.createElement('img');
  logoImg.src = "../" + the_team.logo;
  logoImg.style.height = '100px';
  logoColumn.appendChild(logoImg);

  var conferenceColumn = document.createElement('div');
  conferenceColumn.className = 'conference';
  conferenceColumn.textContent = the_team.conference;

  var table = document.createElement('table');
  var row = table.insertRow(0);

  for (var i = 0; i < 3; i++) {
      var cell = row.insertCell(i);
      
      if (i === 0) {
        cell.textContent = String(the_team.division_rank);
          cell.appendChild(divisionColumn);
      }
      if (i === 1) {
        cell.textContent = String(the_team.name);
          cell.appendChild(logoColumn);
      }
      if (i === 2) {
        cell.textContent = String(the_team.conference_rank);
          cell.appendChild(conferenceColumn);
      }
  }

  innerDiv.appendChild(table);

  return innerDiv;
}

var scheduled;

async function fetchSchedule() {
  try {
    console.log("called");
  invoke('build_schedule_struct')
    .then((schedule) => {
      console.log("in the structure");
      scheduled = schedule;
      // Call a function or perform logic here that requires big
      performLogic();
    })
    .catch((error) => console.error(error));
  } catch (error) {
    console.error(error);
  }
}

var globalElement = document.createElement('div');

// Optionally, set attributes or properties for the element
globalElement.id = 'myGlobalElement';
globalElement.textContent = 'This is my global element';

// You can also set styles if needed
globalElement.style.color = 'blue';
globalElement.style.fontSize = '18px';

function performLogic() {
  if (scheduled) {
    console.log("outside of scope");

    var parentDiv = document.getElementById('schedule_games'); // Declaring parentDiv variable

    parentDiv.appendChild(createTableWithRows());
  
  }
}

// Call the fetchSchedule function
//fetchSchedule();


document.addEventListener('DOMContentLoaded', function() {
  try {
   // console.log(scheduled.games.length);
    
} catch (error) {
   //console.error('An error occurred:', error);
}
});

// Time -- .time
// networks .
// Home Name .home_team.name
// Home Division .home_team.Division
// Home Rank .home_team.Rank
// Home Conference .home_team.Conference
// Home Conference Rank .home_team.Rank
// Wins .home_team.wins
// Losses .home_team.losses
// Points . .home_team.Points

function createTableWithRows() {
  // Create a table element
  let table = document.createElement('table');

  let size = scheduled.games.length;
  if(scheduled){
    console.log("pass");
  }
console.log("Size of scheduled.games:", size);

  // scheduled.games.forEach(game => {
  //   let row = table.insertRow();
  //   //row.appendChild(createTable(game));
  //   // scheduleGamesDiv.style.width = '100%';
  //   row.textContent = "game.time";
  //   row.style.border = '1px solid black';
  //   row.style.textAlign = 'center';

  // });
  //Create rows and cells
  for (let i = 0; i < size; i++) {
      let row = table.insertRow();
      row.appendChild(createTable(i));
      // scheduleGamesDiv.style.width = '100%';
      row.style.border = '1px solid black';
      row.style.textAlign = 'center';
  }

//   for (let i = 0; i < size; i++) {
//     let row = table.insertRow();
//     let cell = row.insertCell();
//     let div = document.createElement('table');
//     div.textContent = "Your text here"; // Replace "Your text here" with your desired text
//     cell.appendChild(div);
//     row.style.border = '1px solid black';
//     row.style.textAlign = 'center';
// }

  return table;
}




function fillTables() {


  var parentDiv = document.getElementById('schedule_games'); // Declaring parentDiv variable
  // Create a new div element

  
  // Set attributes or properties for the new div if needed
 let table = 
  
  // Append the new div to the parent div
  //parentDiv.appendChild(createTableWithRows());
  parentDiv.appendChild(globalElement);
}

function myFunction() {
console.log('myFunction');

  invoke('return_divisions').then((divisions) => {
   
    
      console.log(String(divisions.central[0].teamName.default));
      // Now you can work with central, pacific, atlantic, and metropolitan vectors
      // Example: console.log(central);
    
   // console.log(String(divisions.central.conferenceAbbrev));
    // Now you can work with central, pacific, atlantic, and metropolitan vectors
    // Example: console.log(central);
}).catch(error => {
    // Handle error here
    console.error(error);
});
 
}


// document.addEventListener("click", function(event) {
//   var customMenu = document.getElementById("menu");
//   if (!customMenu.contains(event.target)) {
//       customMenu.style.display = "none";
//   }
// });