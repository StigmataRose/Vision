const invoke = window.__TAURI__.invoke


function getTeamColors(teamName) {
    const teams = {
        "Anaheim Ducks": { primary_color: "#F47A38", secondary_color: "#000000" },
        "Arizona Coyotes": { primary_color: "#8C2633", secondary_color: "#E2D6B5" },
        "Boston Bruins": { primary_color: "#FFB81C", secondary_color: "#000000" },
        "Buffalo Sabres": { primary_color: "#002654", secondary_color: "#FCB514" },
        "Calgary Flames": { primary_color: "#C8102E", secondary_color: "#F1BE48" },
        "Carolina Hurricanes": { primary_color: "#CC0000", secondary_color: "#000000" },
        "Chicago Blackhawks": { primary_color: "#CF0A2C", secondary_color: "#000000" },
        "Colorado Avalanche": { primary_color: "#6F263D", secondary_color: "#236192" },
        "Columbus Blue Jackets": { primary_color: "#002654", secondary_color: "#CE1126" },
        "Dallas Stars": { primary_color: "#006847", secondary_color: "#8F8F8C" },
        "Detroit Red Wings": { primary_color: "#CE1126", secondary_color: "#FFFFFF" },
        "Edmonton Oilers": { primary_color: "#FF4C00", secondary_color: "#041E42" },
        "Florida Panthers": { primary_color: "#C8102E", secondary_color: "#041E42" },
        "Los Angeles Kings": { primary_color: "#111111", secondary_color: "#A2AAAD" },
        "Minnesota Wild": { primary_color: "#154734", secondary_color: "#DDCBA4" },
        "Montréal Canadiens": { primary_color: "#AF1E2D", secondary_color: "#192168" },
        "Nashville Predators": { primary_color: "#FFB81C", secondary_color: "#041E42" },
        "New Jersey Devils": { primary_color: "#CE1126", secondary_color: "#000000" },
        "New York Islanders": { primary_color: "#00539B", secondary_color: "#F47D30" },
        "New York Rangers": { primary_color: "#0038A8", secondary_color: "#CE1126" },
        "Ottawa Senators": { primary_color: "#E31837", secondary_color: "#000000" },
        "Philadelphia Flyers": { primary_color: "#F74902", secondary_color: "#000000" },
        "Pittsburgh Penguins": { primary_color: "#FCB514", secondary_color: "#000000" },
        "San Jose Sharks": { primary_color: "#006D75", secondary_color: "#EA7200" },
        "Seattle Kraken": { primary_color: "#005A9C", secondary_color: "#FFA300" },
        "St. Louis Blues": { primary_color: "#002F87", secondary_color: "#FCB514" },
        "Tampa Bay Lightning": { primary_color: "#002868", secondary_color: "#FFFFFF" },
        "Toronto Maple Leafs": { primary_color: "#003E7E", secondary_color: "#FFFFFF" },
        "Vancouver Canucks": { primary_color: "#00205B", secondary_color: "#00843D" },
        "Vegas Golden Knights": { primary_color: "#B4975A", secondary_color: "#333F42" },
        "Washington Capitals": { primary_color: "#041E42", secondary_color: "#C8102E" },
        "Winnipeg Jets": { primary_color: "#041E42", secondary_color: "#004C97" }
    };
  
    return teams[teamName];
  }
  
  
  function hexToRgba(hex, alpha) {
    // Remove the '#' character if present
    hex = hex.replace('#', '');
  
    // Parse the hex string to get individual RGB values
    var r = parseInt(hex.substring(0, 2), 16);
    var g = parseInt(hex.substring(2, 4), 16);
    var b = parseInt(hex.substring(4, 6), 16);
  
    // Construct the rgba string
    var rgba = 'rgba(' + r + ', ' + g + ', ' + b + ', ' + alpha + ')';
  
    return rgba;
  }

function createStandingsTable() {

    invoke('return_divisions').then((divisions) => {
       
        
      // Get the standings div
      var standingsDiv = document.getElementById("standings");
    
      let titleW = createUnderlinedTitle("WESTERN CONFERENCE");
      titleW.style.fontSize = "20px";
      titleW.style.fontWeight = "bold";
      titleW.style.marginTop = "10px";
      //title.width = "400px";
      standingsDiv.appendChild(titleW);
      {
      // Create a table element
      var table = document.createElement("table");
      table.style.width = "100%";
    
      // Create the table rows
     
      var columns = 2;
    
          var row = document.createElement("tr");
    
          // Create the cells
          for (var j = 0; j < columns; j++) {
              // Create a table cell
              var cell = document.createElement("td");
              cell.style.width = "50%";
              cell.style.textAlign = "center";
              // Assign text content based on row and column
              if (j === 0) {
                let title = createUnderlinedTitle("CENTRAL");
                title.style.fontSize = "20px";
                title.style.fontWeight = "bold";
                title.style.marginTop = "5px";
                //title.width = "400px";
                cell.appendChild(title);
                for(var i = 0; i < divisions.central.length; i++){
                  let cellTable = createTableWithRow(divisions.central[i]);
                  cell.appendChild(cellTable);
                }
              } else if (j === 1) {
                let title = createUnderlinedTitle("PACIFIC");
                title.style.fontSize = "20px";
                title.style.fontWeight = "bold";
                title.style.marginTop = "5px";
                //title.width = "400px";
                cell.appendChild(title);
                for(var i = 0; i < divisions.pacific.length; i++){
                  let cellTable = createTableWithRow(divisions.pacific[i]);
                  cell.appendChild(cellTable);
                }
              } 
              // Append the cell to the row
              row.appendChild(cell);
          }
    
          // Append the row to the table
          table.appendChild(row);
      
    
      // Append the table to the standings div
      standingsDiv.appendChild(table);
        }
      let titleE = createUnderlinedTitle("EASTERN CONFERENCE");
      titleE.style.fontSize = "20px";
      titleE.style.fontWeight = "bold";
      titleE.style.marginTop = "10px";
      //title.width = "400px";
      standingsDiv.appendChild(titleE);
    
      {
        // Create a table element
        var table = document.createElement("table");
        table.style.width = "100%";
      
        // Create the table rows
       
        var columns = 2;
      
            var row = document.createElement("tr");
      
            // Create the cells
            for (var j = 0; j < columns; j++) {
                // Create a table cell
                var cell = document.createElement("td");
                cell.style.width = "50%";
                cell.style.textAlign = "center";
                // Assign text content based on row and column
                if (j === 0) {
                  let title = createUnderlinedTitle("ATLANTIC");
                  title.style.marginTop = "5px";
                  title.style.fontSize = "20px";
                  title.style.fontWeight = "bold";
                  //title.width = "400px";
                  cell.appendChild(title);
                  for(var i = 0; i < divisions.atlantic.length; i++){
                    let cellTable = createTableWithRow(divisions.atlantic[i]);
                    cell.appendChild(cellTable);
                  }
                } else if (j === 1) {
                  let title = createUnderlinedTitle("METROPOLITAN");
                  title.style.marginTop = "5px";
                  title.style.fontSize = "20px";
                  title.style.fontWeight = "bold";
                  //title.width = "400px";
                  cell.appendChild(title);
                  for(var i = 0; i < divisions.metropolitan.length; i++){
                    let cellTable = createTableWithRow(divisions.metropolitan[i]);
                    cell.appendChild(cellTable);
                  }
                } 
                // Append the cell to the row
                row.appendChild(cell);
            }
      
            // Append the row to the table
            table.appendChild(row);
        
      
        // Append the table to the standings div
        standingsDiv.appendChild(table);
          }
    
    }).catch(error => {
      // Handle error here
      console.error(error);
    });
    
    }
    
    
    function createTableWithRow(team) {
      // Create a table element
      var table = document.createElement("table");
      table.style.width = "100%";
      table.style.margin = "auto"; 
      table.style.textAlign = "left";
      // Create a table row
      var tableRow = document.createElement("tr");
    
      // Create table cells for logo, name, and points
      var logoCell = document.createElement("td");
      var nameCell = document.createElement("td");
      var pointsCell = document.createElement("td");
    
      // Create img element for logo
      var logoImg = document.createElement("img");
      logoImg.src = String(team.teamLogo);
      logoImg.alt = team.teamName.default;
      logoImg.style.width = "16.3%";
      logoImg.style.display = "block";
      logoImg.style.display = 'block';
      logoImg.style.marginLeft = 'auto';
    
    
      logoCell.appendChild(logoImg);
      
      if(team.teamName.default === "Nashville Predators"){
        nameCell.id = "nashvillePredators";
        nameCell.style.fontWeight = "bold";
      }
     
      // Set text content for name and points
      if(team.wildcardSequence === 1){
        nameCell.textContent = team.teamName.default + "*";
      }else if(team.wildcardSequence === 2){
        nameCell.textContent = team.teamName.default + "**";
      }else{
        nameCell.textContent = team.teamName.default;
      }
      
      pointsCell.textContent = team.points;
      nameCell.style.width = "40%";
      pointsCell.style.width = "33.3%";
      pointsCell.textAlign = "left";
      nameCell.textAlign = "left";
     
      // Append cells to the row
      tableRow.appendChild(logoCell);
      tableRow.appendChild(nameCell);
      tableRow.appendChild(pointsCell);
      
    
      // Append the row to the table
      table.appendChild(tableRow);
    
      return table;
    }
    
    function createUnderlinedTitle(titleText) {
      // Create a div element
      var titleDiv = document.createElement('title_standings');
      
      titleDiv.style.display = 'flex';
    titleDiv.style.justifyContent = 'center';
    titleDiv.style.alignItems = 'center';
    titleDiv.style.textAlign = 'center';
    
      var lineDiv = document.createElement('line_div');
      // Set the text content of the div to the provided title text
      lineDiv.textContent = titleText;
      
      // Apply styles for underlining only the bottom part
      lineDiv.style.width = "33.3%";
     
      //titleDiv.style.textDecoration = "underline";
      lineDiv.style.textDecorationColor = "black";
      lineDiv.style.textDecorationStyle = "solid";
      lineDiv.style.textDecorationThickness = "0.1em";
      lineDiv.style.textDecorationSkip = "ink";
      
      lineDiv.style.borderBottom = "2px solid black";
        
      titleDiv.appendChild(lineDiv);
      // Return the created title div
      return titleDiv;
    }
    

    
    // function load_charts(){
    //   populate_league_win_loss();
    //   populate_home_win_loss();
    //   chartTitles();
    // }
    
    function chartTitles(){
      // Call the function to create the underlined title
    var underlinedTitle = createUnderlinedTitle("Win and Loss");
    underlinedTitle.style.fontSize = "20px";
    underlinedTitle.style.fontWeight = "bold";
    underlinedTitle.style.marginTop = "10px";
    // Get a reference to the div with the id "chart_wins_losses"
    var chartDiv = document.getElementById("league_chart");
    
    // Append the created underlined title to the chart div
    chartDiv.appendChild(underlinedTitle);
    
    var underline_home_title = createUnderlinedTitle("Home Records");
    underline_home_title.style.fontSize = "20px";
    underline_home_title.style.fontWeight = "bold";
    underline_home_title.style.marginTop = "10px";
    // Get a reference to the div with the id "chart_wins_losses"
    var homeDiv = document.getElementById("home_chart");
    
    // Append the created underlined title to the chart div
    homeDiv.appendChild(underline_home_title);

    var underline_away_title = createUnderlinedTitle("Road Records");
    underline_away_title.style.fontSize = "20px";
    underline_away_title.style.fontWeight = "bold";
    underline_away_title.style.marginTop = "10px";
    // Get a reference to the div with the id "chart_wins_losses"
    var awayDiv = document.getElementById("away_chart");
    
    // Append the created underlined title to the chart div
    awayDiv.appendChild(underline_away_title);

    var underline_differ_title = createUnderlinedTitle("Goal Differential");
    underline_differ_title.style.fontSize = "20px";
    underline_differ_title.style.fontWeight = "bold";
    underline_differ_title.style.marginTop = "10px";
    // Get a reference to the div with the id "chart_wins_losses"
    var differDiv = document.getElementById("differ_chart");
    
    // Append the created underlined title to the chart div
    differDiv.appendChild(underline_differ_title);

    var underline_L10_title = createUnderlinedTitle("Last 10");
    underline_L10_title.style.fontSize = "20px";
    underline_L10_title.style.fontWeight = "bold";
    underline_L10_title.style.marginTop = "10px";
    // Get a reference to the div with the id "chart_wins_losses"
    var L10Div = document.getElementById("last10_chart");
    
    // Append the created underlined title to the chart div
    L10Div.appendChild(underline_L10_title);
    }
    
  
    // document.addEventListener("DOMContentLoaded", createStandingsTable);
    // document.addEventListener("DOMContentLoaded", populate_league_win_loss);
    // document.addEventListener("DOMContentLoaded", populate_home_win_loss);
    // document.addEventListener("DOMContentLoaded", populate_away_win_loss);
    // document.addEventListener("DOMContentLoaded", populate_differ_chart);
   
    
    document.addEventListener('DOMContentLoaded', function() {
      // Example usage:
      //console.log('DOM loaded');
      
      chartTitles();
      createStandingsTable();
      display_charts();
      // display_league_info();
      // display_home_info();
      // display_away_info();
      // display_differ_info();
  });

  function display_charts(){

    invoke('return_league_standings').then((league) => {
     
      let league_records = [];
      let home_records = [];
      let away_records = [];
      let differ_array = [];

      let last10_array = [];

      for (var i = 0; i < league.length; i++) {
        let label = league[i].teamAbbrev.default;
        let wins = league[i].wins;
        let losses = league[i].losses;

        let full = league[i].teamName.default;
        let common = league[i].teamCommonName.default;
        let x = league[i].goalDifferential;
        let y = league[i].goalsForPctg;
        let z = league[i].goalsForPctg;

        let home_wins = league[i].homeWins;
        let home_losses = league[i].homeLosses;

        let away_wins = league[i].roadWins;
        let away_losses = league[i].roadLosses;

        let L10wins = league[i].l10Wins;
        let L10losses = league[i].l10Losses;

        league_records.push({
          label: label,
          wins: wins,
          losses: losses
        });

        differ_array.push({
          full: full,
          common: common,
          x: x,
          y: y,
          z: z});

        home_records.push({
          label: label,
          wins: home_wins,
          losses: home_losses
        });

        away_records.push({
          label: label,
          wins: away_wins,
          losses: away_losses
        });

        last10_array.push({
            label: label,
            wins: L10wins,
            losses: L10losses
          });
        
    }

    display_league_info(league_records);
    display_differ_info(differ_array);

    const sortedHome = home_records.sort((a, b) => b.wins - a.wins);
    display_home_info(sortedHome);
    const sortedAway = away_records.sort((a, b) => b.wins - a.wins);
    display_away_info(sortedAway);

    const sortedL10 = last10_array.sort((a, b) => b.wins - a.wins);
    display_L10_info(sortedL10);
   

  }).catch(error => {
    // Handle error here
    console.error(error);
  });
  
  }

  function display_league_info(league){
    var abb_array = [];
    var wins_array = [];
    var losses_array = [];

  
  
      for (var i = 0; i < league.length; i++) {
        abb_array.push(league[i].label);
        wins_array.push(league[i].wins);
        losses_array.push(league[i].losses);
        
    }


    // Create a script element for chart.js
    // var chartScript = document.createElement('script');
    // chartScript.src = '../chart.js';
    // Sample data for the chart
       // Sample data for the chart
       
   
       var win_color_b = hexToRgba("#041E42",1.0);
       var loss_color_b = hexToRgba("#FFB81C",1.0);
  
       var win_color = hexToRgba("#041E42",0.7);
       var loss_color = hexToRgba("#FFB81C",0.7);
       var data = {
        labels: abb_array,
        datasets: [{
            label: 'Wins',
            data: wins_array,
            backgroundColor: win_color,
            borderColor: win_color_b,
            borderWidth: 1
        },
        {
            label: 'Losses',
            data: losses_array,
            backgroundColor: loss_color,
            borderColor: loss_color_b,
            borderWidth: 1
        }]
    };
  

     // Get the reference to the parent element
     var parentElement = document.getElementById('League_Chart_Parent');
     // Check if there is an old canvas
     var oldCanvas = document.getElementById('leagueWinsAndLossesCanvas');
     if (oldCanvas) {
         // If old canvas exists, remove it
         parentElement.removeChild(oldCanvas);
     }

    // Get the canvas element
    var canvas = document.createElement('canvas');
    canvas.id = 'leagueWinsAndLossesCanvas';
  
    
    //canvas.width = 400;
    //canvas.height = 200;
    document.getElementById('League_Chart_Parent').appendChild(canvas);
  
    // Get the canvas context
    var ctx = canvas.getContext('2d');
  
    // Create the chart
    var myChart = new Chart(ctx, {
        type: 'bar',
        data: data,
        options: {
            scales: {
                x: {
                    stacked: true
                },
                y: {
                    stacked: true,
                    ticks: {
                      beginAtZero: true,
                  },
                  min: 0,
                  max: 82
                }
            },
            responsive: true, // Allow chart to resize
            maintainAspectRatio: false
        }
    });


  }

  function display_home_info(league){
    var abb_array = [];
    var wins_array = [];
    var losses_array = [];
 
 
      for (var i = 0; i < league.length; i++) {

        abb_array.push(league[i].label);
        wins_array.push(league[i].wins);
        losses_array.push(league[i].losses);
     
    }

    // var chartScript = document.createElement('script2');
    // chartScript.src = '../chart.js';

    var win_color_b = hexToRgba("#041E42",1.0);
    var loss_color_b = hexToRgba("#FFB81C",1.0);

    var win_color = hexToRgba("#041E42",0.7);
    var loss_color = hexToRgba("#FFB81C",0.7);

     //  var teamColors = generatePrimaryTeamColors(teamName_array);
      // var teamColors2 = generateSecondaryTeamColors(teamName_array);

       var data = {
        labels: abb_array,
        datasets: [{
            label: 'Wins',
            data: wins_array,
            backgroundColor: win_color,
            borderColor: win_color_b,
            borderWidth: 1
        },
        {
            label: 'Losses',
            data: losses_array,
            backgroundColor: loss_color,
            borderColor: loss_color_b,
            borderWidth: 1
        }]
    };

    // Get the reference to the parent element
    var parentElement = document.getElementById('Home_Chart_Parent');
    // Check if there is an old canvas
    var oldCanvas = document.getElementById('homeChartCanvas');
    if (oldCanvas) {
        // If old canvas exists, remove it
        parentElement.removeChild(oldCanvas);
    }

    // Get the canvas element
    var canvas = document.createElement('canvas');
    canvas.id = 'homeChartCanvas';
    //canvas.width = 400;
    //canvas.height = 200;
    document.getElementById('Home_Chart_Parent').appendChild(canvas);
  
    // Get the canvas context
    var ctx = canvas.getContext('2d');
  

    
    // Create the chart
    var myChart = new Chart(ctx, {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                legend: {
                    display: true
                },
                
            },
            scales: {
                x: {
                    stacked: true
                },
                y: {
                    stacked: true,
                    ticks: {
                      beginAtZero: true,
                  },
                  min: 0,
                  max: 41
                }
            },
            responsive: true, // Allow chart to resize
            maintainAspectRatio: false
        }
    });


  
    //canvas.style.margin = '0 auto';
    //canvas.style.width = '100%';
  }

  function display_away_info(league){

    var abb_array = [];
    var wins_array = [];
    var losses_array = [];
   

      for (var i = 0; i < league.length; i++) {
        abb_array.push(league[i].label);
        wins_array.push(league[i].wins);
        losses_array.push(league[i].losses);
    }

    // var chartScript = document.createElement('script2');
    // chartScript.src = '../chart.js';

    var win_color_b = hexToRgba("#041E42",1.0);
    var loss_color_b = hexToRgba("#FFB81C",1.0);

    var win_color = hexToRgba("#041E42",0.7);
    var loss_color = hexToRgba("#FFB81C",0.7);
       var data = {
        labels: abb_array,
        datasets: [{
            label: 'Wins',
            data: wins_array,
            backgroundColor: win_color,
            borderColor: win_color_b,
            borderWidth: 1
        },
        {
            label: 'Losses',
            data: losses_array,
            backgroundColor: loss_color,
            borderColor: loss_color_b,
            borderWidth: 1
        }]
    };
  

      // Get the reference to the parent element
      var parentElement = document.getElementById('Away_Chart_Parent');
      // Check if there is an old canvas
      var oldCanvas = document.getElementById('AwayRecordCanvas');
      if (oldCanvas) {
          // If old canvas exists, remove it
          parentElement.removeChild(oldCanvas);
      }

    // Get the canvas element
    var canvas = document.createElement('canvas');
    canvas.id = 'AwayRecordCanvas';
    //canvas.width = 400;
    //canvas.height = 200;
    document.getElementById('Away_Chart_Parent').appendChild(canvas);
  
    // Get the canvas context
    var ctx = canvas.getContext('2d');
  
    // Create the chart
    var myChart = new Chart(ctx, {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                legend: {
                    display: true
                },
            },
            scales: {
                x: {
                    stacked: true
                },
                y: {
                    stacked: true,
                    ticks: {
                      beginAtZero: true,
                  },
                  min: 0,
                  max: 41
                }
            },
            responsive: true, // Allow chart to resize
            maintainAspectRatio: false
        }
    });



  }


  function display_L10_info(league){

    var abb_array = [];
    var wins_array = [];
    var losses_array = [];
   

      for (var i = 0; i < league.length; i++) {
        abb_array.push(league[i].label);
        wins_array.push(league[i].wins);
        losses_array.push(league[i].losses);
    }

    // var chartScript = document.createElement('script2');
    // chartScript.src = '../chart.js';

    var win_color_b = hexToRgba("#041E42",1.0);
    var loss_color_b = hexToRgba("#FFB81C",1.0);

    var win_color = hexToRgba("#041E42",0.7);
    var loss_color = hexToRgba("#FFB81C",0.7);
       var data = {
        labels: abb_array,
        datasets: [{
            label: 'Wins',
            data: wins_array,
            backgroundColor: win_color,
            borderColor: win_color_b,
            borderWidth: 1
        },
        {
            label: 'Losses',
            data: losses_array,
            backgroundColor: loss_color,
            borderColor: loss_color_b,
            borderWidth: 1
        }]
    };
  

      // Get the reference to the parent element
      var parentElement = document.getElementById('Last10_Chart_Parent');
      // Check if there is an old canvas
      var oldCanvas = document.getElementById('L10RecordCanvas');
      if (oldCanvas) {
          // If old canvas exists, remove it
          parentElement.removeChild(oldCanvas);
      }

    // Get the canvas element
    var canvas = document.createElement('canvas');
    canvas.id = 'L10RecordCanvas';
    //canvas.width = 400;
    //canvas.height = 200;
    document.getElementById('Last10_Chart_Parent').appendChild(canvas);
  
    // Get the canvas context
    var ctx = canvas.getContext('2d');
  
    // Create the chart
    var myChart = new Chart(ctx, {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                legend: {
                    display: true
                },
            },
            scales: {
                x: {
                    stacked: true
                },
                y: {
                    stacked: true,
                    ticks: {
                      beginAtZero: true,
                  },
                  min: 0,
                  max: 10
                }
            },
            responsive: true, // Allow chart to resize
            maintainAspectRatio: false
        }
    });



  }

  function populate_away_win_loss() {

   
  
  }

  function display_differ_info(league) {

    var datasets = [];


       for (var i = 0; i < league.length; i++) {

       // console.log("Full Name:", full);
        let dataset = {
           label: league[i].common,
           data: [{ x: league[i].x, y: league[i].y, r: league[i].z }],
           backgroundColor: hexToRgba(getTeamColors(league[i].full).primary_color,1.0),
           borderColor: hexToRgba(getTeamColors(league[i].full).secondary_color,1.0)
       };
   
       // Output the values to the console
// console.log("Label:", dataset.label);
// console.log("Data:", dataset.data);
// console.log("Background Color:", dataset.backgroundColor);
// console.log("Border Color:", dataset.borderColor);

       // Push the dataset object into the datasets array
       datasets.push(dataset);
        
    }

    //var dataS = [];

// for (let i = 0; i < commonName.length; i++) {
//     // const label = commonName[i];
//     // const data = [{ x: difference[i], y: difference[i], r: percent[i] }];
//     console.log(commonName[i]);
//     // const backgroundColor = hexToRgba(getTeamColors(fullname[i]).secondary_color,1.0);
//     // const borderColor = hexToRgba(getTeamColors(fullname[i]).primary_color,1.0);
//     // dataS.push(new BubbleSet(label, data, backgroundColor, borderColor));
// }

   // Get the reference to the parent element
   var parentElement = document.getElementById('Differ_Chart_Parent');
   // Check if there is an old canvas
   var oldCanvas = document.getElementById('DifferCanvas');
   if (oldCanvas) {
       // If old canvas exists, remove it
       parentElement.removeChild(oldCanvas);
   }

 // Get the canvas element
 var canvas = document.createElement('canvas');
 canvas.id = 'DifferCanvas';
 //canvas.width = 400;
 //canvas.height = 200;
 document.getElementById('Differ_Chart_Parent').appendChild(canvas);

 // Get the canvas context
 var ctx = canvas.getContext('2d');
var data = {
   datasets: datasets
};

var options = {
   plugins: {
       legend: {
           display: false
       },
       tooltip: {
        callbacks: {
          label: function(context) {
            console.log(context);
              var label = context.dataset.label;

              label += " Y GPG: " + String(context.dataset.data[0].y);

              label += " X Goal Diff: " + String(context.dataset.data[0].x);

              return label;
          }
      }
    }
   },
   scales: {
       x: {
            grace: '10%'
       
       },
       y: {
       
           grace: '10%'
           
       }
   },
   responsive: true, // Allow chart to resize
   maintainAspectRatio: false
};

var bubbleChart = new Chart(ctx, {
   type: 'bubble',
   data: data,
   options: options
});

 

  }

  function generatePrimaryTeamColors(teamName) {
    var colors = [];
    for (var i = 0; i < teamName.length; i++) {
        var rgbaColor = hexToRgba(getTeamColors(teamName[i]).primary_color,1.0);
        colors.push(rgbaColor);
    }
    console.log(colors);
    return colors;
}

function generateSecondaryTeamColors(teamName) {
  var colors = [];
  for (var i = 0; i < teamName.length; i++) {
      var rgbaColor = hexToRgba(getTeamColors(teamName[i]).secondary_color,0.5);
      colors.push(rgbaColor);
  }
  console.log(colors);
  return colors;
}