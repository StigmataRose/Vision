const invoke = window.__TAURI__.invoke
function createForwardTable(forwards) {
    // Assume there's a global variable or function 'getForwardsData' that retrieves the forwards data for a given team index
    //var forwardsData = getForwardsData(teamIndex);

   // console.log(String(forwards.length) +'forwards found after');

    var forwardsData = [];

    for (var i = 0; i < forwards.length; i++) {
        let fullName = forwards[i].firstName.default + ' ' + forwards[i].lastName.default;
        let calc_age = calculateAge(forwards[i].birthDate);

        let dataset = {
            picture: forwards[i].headshot,
            name: fullName,
            age: calc_age,
            number: forwards[i].sweaterNumber,
            position: forwards[i].positionCode,
            shoots: forwards[i].shootsCatches
        };
    
        // Output the values to the console
        // console.log("Picture:", dataset.picture);
        //console.log("Name:", fullName);
        // console.log("Number:", dataset.number);
        // console.log("Position:", dataset.position);
        // console.log("Shoots:", dataset.shoots);
    
        // Push the dataset object into the forwardsData array
        forwardsData.push(dataset);
    }

  
    // Create table element
    var table = document.createElement('table');
    table.setAttribute('border', '1');

    // Create table header row
    var headerRow = document.createElement('tr');
    headerRow.style.maxHeight = '50px'; // Set max height of the image
    // Define column headers
    var headers = ['Player', 'Name', 'Age','Number', 'Position', 'Shoots'];

    // Create header cells
    headers.forEach(function(headerText) {
        var headerCell = document.createElement('th');
        headerCell.textContent = headerText;
        headerCell.style.color = 'black';
        headerRow.appendChild(headerCell);
    });

    // Append header row to table
    table.appendChild(headerRow);

    // Create data row for each forward
    forwardsData.forEach(function(forward) {
        var dataRow = document.createElement('tr');
        dataRow.style.maxHeight = '50px';
        // Create cells for each attribute
        var pictureCell = document.createElement('td');
        var nameCell = document.createElement('td');
        var ageCell = document.createElement('td');
        var numberCell = document.createElement('td');
        var positionCell = document.createElement('td');
        var shootsCell = document.createElement('td');
    
        // Create an image element
        var img = document.createElement('img');
        img.src = forward.picture; // Set the source of the image
        img.style.maxHeight = '50px'; // Set max height of the image
        pictureCell.appendChild(img); // Append the image to the pictureCell
    
        // Assign values to other cells
        nameCell.textContent = forward.name;
        ageCell.textContent = forward.age;
        numberCell.textContent = forward.number;
        positionCell.textContent = forward.position;
        shootsCell.textContent = forward.shoots;
    
        // Apply black text color
        nameCell.style.color = 'black';
        ageCell.style.color = 'black';
        numberCell.style.color = 'black';
        positionCell.style.color = 'black';
        shootsCell.style.color = 'black';

        // Set height for each cell
        pictureCell.style.height = '50px';
        nameCell.style.height = '50px';
        ageCell.style.height = '50px';
        numberCell.style.height = '50px';
        positionCell.style.height = '50px';
        shootsCell.style.height = '50px';
    
        // Append cells to data row
        dataRow.appendChild(pictureCell);
        dataRow.appendChild(nameCell);
        dataRow.appendChild(ageCell);
        dataRow.appendChild(numberCell);
        dataRow.appendChild(positionCell);
        dataRow.appendChild(shootsCell);
    
        // Append data row to table
        table.appendChild(dataRow);
    });
    

    // Get the container div
    var containerDiv = document.getElementById('forwardTableContainer');

    // Clear the container
    containerDiv.innerHTML = '';

    let titleF = createUnderlinedTitle("Forwards");
    titleF.style.fontSize = "20px";
    titleF.style.fontWeight = "bold";
    titleF.style.marginTop = "10px";
    titleF.style.color = "black"; // Set font color to black
    // Append title and table to container
    containerDiv.appendChild(titleF);
  
    containerDiv.appendChild(table);
}


function getTeamInfo(input) {
    var datasets = [];

    let name = getTeamName(input);
    let team = getTeamColors(name);
    let primary = hexToRgba(team.primary_color,1.0);
    let secondary = hexToRgba(team.secondary_color,1.0);
    let colors = {primary: primary, secondary: secondary};

    invoke('get_team_roster_by_index', { index: input }).then((roster) => {
        // Assuming roster is an object containing forwards array

      //
      changeTextAndColor(name,primary, secondary);
        let forewards = roster.forwards;
        createForwardTable(forewards);

        let forwards = [];
        let defense = [];
        let goalies = [];
        
        invoke('get_season_stats_by_index', { index: input }).then((season) => {
            // Assuming season contains player stats with a 'position' property
           
            season.skaters.forEach(player => {
                // Check the player's position
             
                switch(player.positionCode) {
                    case 'L':
                        forwards.push(player);
                        break;
                    case 'C':
                        forwards.push(player);
                        break;   
                    case 'R':
                        forwards.push(player);
                        break; 
                    case 'D':
                        defense.push(player);
                        break;
                    default:
                        // Handle unknown position or other cases
                        break;
                }
            });

            season.goalies.forEach(player => {

                goalies.push(player);
                
            });
        
            // Now you have forwards, defense, and goalies arrays populated
            // console.log('Forwards:', forwards);
            // console.log('Defense:', defense);
            // console.log('Goalies:', goalies);
            createTitleAndSubtitle(0, colors, "totalChart_Parent", "Games Played", "Games Played", "Time On Ice", "Plus/Minus");
            create_forwards_charts(forwards, colors);

            createTitleAndSubtitle(1, colors, "forwardPointsChart_Parent", "Points", "Points", "Points Per Game", "Points Per Hour");

            create_forwards_points_charts(forwards, colors);

            createTitleAndSubtitle(2, colors, "forwardGoalChart_Parent", "Goals", "Goals", "Goals Per Game", "Goals Per Hour");

            create_forwards_goals_charts(forwards, colors);

            createTitleAndSubtitle(3, colors, "forwardAssistsChart_Parent", "Assists", "Assists","Assists Per Game", "Assists Per Hour");

            create_assist_points_charts(forwards, colors);

            createTitleAndSubtitle(4, colors, "forwardShotsChart_Parent", "Shots", "Shots","Shots Percentage", "Shots Per Hour");

            create_shot_points_charts(forwards, colors);

        }).catch(error => {
            // Handle error here
            console.error(error);
        });


       // console.log(String(forewards.length) +'forwards found first');
        // Loop through the forwards array
        // roster.forwards.forEach((forward) => {
        //     // Log the information of the first forward
        //     console.log("First Forward Name:", forward.firstName.default);
        //     console.log("First Forward Last Name:", forward.lastName.default);
        //     console.log("First Forward Position:", forward.position);
        //     // You can access other properties of the forward object here and perform necessary operations
        // });

    }).catch(error => {
        // Handle error here
        console.error(error);
    });
}

// Call createForwardTable function when the DOM content is loaded
document.addEventListener('DOMContentLoaded', function() {
    // Example usage:
    //console.log('DOM loaded');
    getTeamInfo(16); // Pass the team index to get the corresponding data
});

function calculateAge(birthDateString) {
    // Split the birthdate string into year, month, and day
    const [year, month, day] = birthDateString.split('-').map(Number);

    // Create a Date object for the birthdate
    const birthdate = new Date(year, month - 1, day);

    // Get the current date
    const currentDate = new Date();

    // Calculate the difference in years
    let age = currentDate.getFullYear() - birthdate.getFullYear();

    // Adjust age if the birthday hasn't occurred yet this year
    if (currentDate.getMonth() < birthdate.getMonth() ||
        (currentDate.getMonth() === birthdate.getMonth() && currentDate.getDate() < birthdate.getDate())) {
        age--;
    }

    return age;
}

function createUnderlinedTitle(titleText) {
    // Create a div element
    var titleDiv = document.createElement('title_standings');
    
    titleDiv.style.display = 'flex';
  titleDiv.style.justifyContent = 'center';
  titleDiv.style.alignItems = 'center';
  titleDiv.style.textAlign = 'center';
  titleDiv.style.padding = '10px';
  
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


function createSubTitle(titleText) {
    // Create a div element
    var titleDiv = document.createElement('sub_title_standings');
    
    titleDiv.style.display = 'flex';
  titleDiv.style.justifyContent = 'center';
  titleDiv.style.alignItems = 'center';
  titleDiv.style.textAlign = 'center';
  
  titleDiv.style.textDecorationColor = "black";
  titleDiv.style.textDecorationStyle = "solid";
  titleDiv.style.textDecorationThickness = "0.1em";
  titleDiv.style.textDecorationSkip = "ink";
  
  titleDiv.textContent = titleText;
    // Return the created title div
    return titleDiv;
  }
  function changeTitle(index) {
    var titleBox = document.getElementById('title_box');
    let name = getTeamName(index);
    let team = getTeamColors(name);

  // Change border image
titleBox.style.backgroundColor = team.primary_color;
titleBox.style.color = team.secondary_color;
//titleBox.style.borderImage = 'linear-gradient(to left, '+team.primary_color+', '+team.primary_color+', '+team.primary_color+')';
titleBox.style.textShadow = '2px 2px 4px rgba(0, 0, 0, 0.5)'; // Add shadow to text
titleBox.style.borderRadius = '10px'; // Round corners
titleBox.style.boxShadow = '2px 2px 4px rgba(0, 0, 0, 0.5)'; 
    // Change text
    titleBox.innerText = name;
  }


  function createTitleAndSubtitle(chartID,colors,parentID, title, button1, button2, button3) {

    let parentElem = document.getElementById(parentID);
    while (parentElem.firstChild) {
        parentElem.removeChild(parentElem.firstChild);
    }

    function createUnderlinedTitle1(titleText) {
        let titleElem = document.createElement("div");
        titleElem.innerText = titleText;
        titleElem.id = titleText + "ID";
        titleElem.style.textDecoration = "underline";
        return titleElem;
    }

   
    function createSubtitleButton(subTitleText, bold, localIndex) {
        let subTitleElem = document.createElement("button"); // Create a button element
        subTitleElem.innerText = subTitleText;
        subTitleElem.style.fontSize = "14px";
        subTitleElem.style.color = "black"; // Set font color to black
        subTitleElem.style.border = "none"; // Remove border
        subTitleElem.style.background = "none"; // Remove background
        subTitleElem.style.cursor = "pointer"; // Change cursor to pointer
        subTitleElem.id = subTitleText;
        if (bold) {
            subTitleElem.style.fontWeight = "bold";
        }else{
            subTitleElem.style.fontWeight = "normal";
        }
        subTitleElem.addEventListener("click", function() { // Add click event listener

            document.getElementById(button1).style.fontWeight = "normal";
            document.getElementById(button2).style.fontWeight = "normal";
            document.getElementById(button3).style.fontWeight = "normal";

            document.getElementById(subTitleText).style.fontWeight = "bold";
            document.getElementById(title + "ID").innerText = subTitleText;
            update_forwardsChart(chartID, colors, localIndex);

        });
       
        return subTitleElem;
    }


    
    let titleF = createUnderlinedTitle1(title);
    titleF.style.fontSize = "16px";
    titleF.style.fontWeight = "bold";
    titleF.style.color = "black"; // Set font color to black
    document.getElementById(parentID).appendChild(titleF);
  
    let subtitle1 = createSubtitleButton(button1, true, 0); // Pass true to indicate bold
    document.getElementById(parentID).appendChild(subtitle1);
    
    let subtitle2 = createSubtitleButton(button2, false, 1); // Pass false for regular
    document.getElementById(parentID).appendChild(subtitle2);
    
    let subtitle3 = createSubtitleButton(button3, false, 2); // Pass false for regular
    document.getElementById(parentID).appendChild(subtitle3);
         
}


function update_forwardsChart(chartID, colors, localIndex) {
    switch(chartID) {
        case 0: {
            switch(localIndex) {
                case 0: {
                    const sortedData = forwardGames.sort((a, b) => b.amount - a.amount);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.amount);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.TOI);
                    const aphArray = sortedData.map(obj => obj.PM);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'GP',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nTOI: ' + extraData;
                                label += '\n+/-: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('totalChart_Parent');
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('games_played_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                      var canvas = document.createElement('canvas');
                      canvas.id = 'games_played_chart';
                      document.getElementById('totalChart_Parent').appendChild(canvas);
                      var ctx = canvas.getContext('2d');
                    var myChart = new Chart(
                        ctx,
                        config
                    ); 
                    break;
                }
                case 1: {
                    const sortedData = forwardGames.sort((a, b) => b.TOI - a.TOI);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.TOI);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.amount);
                    const aphArray = sortedData.map(obj => obj.PM);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'TOI',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nGP: ' + extraData;
                                label += '\n+/-: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('totalChart_Parent');
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('games_played_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                      var canvas = document.createElement('canvas');
                      canvas.id = 'games_played_chart';
                      document.getElementById('totalChart_Parent').appendChild(canvas);
                      var ctx = canvas.getContext('2d');
                    var myChart = new Chart(
                        ctx,
                        config
                    ); 
                    break;
                }
                case 2: {
                    const sortedData = forwardGames.sort((a, b) => b.PM - a.PM);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.PM);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.amount);
                    const aphArray = sortedData.map(obj => obj.TOI);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: '+/-',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nGP: ' + extraData;
                                label += '\nTOI: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('totalChart_Parent');
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('games_played_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                      var canvas = document.createElement('canvas');
                      canvas.id = 'games_played_chart';
                      document.getElementById('totalChart_Parent').appendChild(canvas);
                      var ctx = canvas.getContext('2d');
                    var myChart = new Chart(
                        ctx,
                        config
                    ); 
                    break;
                }
                default: {
                    console.log("Invalid local index for Chart 0");
                    break;
                }
            }
            break;
        }
        case 1: {
            switch(localIndex) {
                case 0: {
                    console.log(forwardPoints);
                    const sortedData = forwardPoints.sort((a, b) => b.amount - a.amount);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.amount);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.PPG);
                    const aphArray = sortedData.map(obj => obj.PPH);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'Points',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                   
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nPPG: ' + extraData;
                                label += '\nPPH: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                
                
                
                
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardPointsChart_Parent');
                
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_point_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_point_chart';
                      //canvas.width = 400;
                      //canvas.height = 200;
                      document.getElementById('forwardPointsChart_Parent').appendChild(canvas);
                    
                      // Get the canvas context
                      var ctx = canvas.getContext('2d');
                
                      
                
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                case 1: {
                    const sortedData = forwardPoints.sort((a, b) => b.PPG - a.PPG);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.PPG);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.PPH);
                    const aphArray = sortedData.map(obj => obj.amount);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'PPG',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                   
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nPPH: ' + extraData;
                                label += '\nPoints: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                
                
                
                
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardPointsChart_Parent');
                
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_point_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_point_chart';
                      //canvas.width = 400;
                      //canvas.height = 200;
                      document.getElementById('forwardPointsChart_Parent').appendChild(canvas);
                    
                      // Get the canvas context
                      var ctx = canvas.getContext('2d');
                
                      
                
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                case 2: {
                    const sortedData = forwardPoints.sort((a, b) => b.PPH - a.PPH);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.PPH);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.PPG);
                    const aphArray = sortedData.map(obj => obj.amount);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'PPH',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                   
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nPPG: ' + extraData;
                                label += '\nPoints: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                
                
                
                
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardPointsChart_Parent');
                
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_point_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_point_chart';
                      //canvas.width = 400;
                      //canvas.height = 200;
                      document.getElementById('forwardPointsChart_Parent').appendChild(canvas);
                    
                      // Get the canvas context
                      var ctx = canvas.getContext('2d');
                
                      
                
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                default: {
                    console.log("Invalid local index for Chart 1");
                    break;
                }
            }
            break;
        }
        case 2: {
            switch(localIndex) {
                case 0: {
                    const sortedData = forwardGoals.sort((a, b) => b.amount - a.amount);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.amount);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.goalsPerGame);
                    const aphArray = sortedData.map(obj => obj.goalsPerHour);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'Goals',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nGPG: ' + extraData;
                                label += '\nGPH: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardGoalChart_Parent');
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_goal_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_goal_chart';
                      document.getElementById('forwardGoalChart_Parent').appendChild(canvas);
                      var ctx = canvas.getContext('2d');
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                case 1: {
                    const sortedData = forwardGoals.sort((a, b) => b.goalsPerGame - a.goalsPerGame);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.goalsPerGame);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.goalsPerHour);
                    const aphArray = sortedData.map(obj => obj.amount);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'GPG',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nGPH: ' + extraData;
                                label += '\nGoals: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardGoalChart_Parent');
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_goal_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_goal_chart';
                      document.getElementById('forwardGoalChart_Parent').appendChild(canvas);
                      var ctx = canvas.getContext('2d');
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                case 2: {
                    const sortedData = forwardGoals.sort((a, b) => b.goalsPerHour - a.goalsPerHour);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.goalsPerHour);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.goalsPerGame);
                    const aphArray = sortedData.map(obj => obj.amount);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'GPH',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nGPG: ' + extraData;
                                label += '\nGoals: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardGoalChart_Parent');
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_goal_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_goal_chart';
                      document.getElementById('forwardGoalChart_Parent').appendChild(canvas);
                      var ctx = canvas.getContext('2d');
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                default: {
                    console.log("Invalid local index for Chart 2");
                    break;
                }
            }
            break;
        }
        case 3: {
            switch(localIndex) {
                case 0: {
                    const sortedData = forwardAssists.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.APG);
    const aphArray = sortedData.map(obj => obj.APH);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Assists',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nAPG: ' + extraData;
                label += '\nAPH: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};





  // Get the reference to the parent element
var parentElement = document.getElementById('forwardAssistsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_Assists_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_Assists_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardAssistsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );
                    break;
                }
                case 1: {
                    const sortedData = forwardAssists.sort((a, b) => b.APG - a.APG);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.APG);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.APH);
                    const aphArray = sortedData.map(obj => obj.amount);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'APG',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                   
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nAPH: ' + extraData;
                                label += '\nAssists: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                
                
                
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardAssistsChart_Parent');
                
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_Assists_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_Assists_chart';
                      //canvas.width = 400;
                      //canvas.height = 200;
                      document.getElementById('forwardAssistsChart_Parent').appendChild(canvas);
                    
                      // Get the canvas context
                      var ctx = canvas.getContext('2d');
                
                      
                
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                case 2: {
                    const sortedData = forwardAssists.sort((a, b) => b.APH - a.APH);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.APH);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.APG);
    const aphArray = sortedData.map(obj => obj.amount);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'APH',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nAPG: ' + extraData;
                label += '\nAssists: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};





  // Get the reference to the parent element
var parentElement = document.getElementById('forwardAssistsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_Assists_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_Assists_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardAssistsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );
                    break;
                }
                default: {
                    console.log("Invalid local index for Chart 3");
                    break;
                }
            }
            break;
        }
        case 4: {
            switch(localIndex) {
                case 0: {
                    const sortedData = forwardShots.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.SP);
    const aphArray = sortedData.map(obj => obj.SPH);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Shots',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nS: ' + extraData + "%";
                label += '\nSPH: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};





  // Get the reference to the parent element
var parentElement = document.getElementById('forwardShotsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_Shots_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_Shots_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardShotsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );
                    break;
                }
                case 1: {
                    const sortedData = forwardShots.sort((a, b) => b.SP - a.SP);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.SP);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.SPH);
    const aphArray = sortedData.map(obj => obj.amount);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'S',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y + "%";
                // Append additional text with line break
                label += '\nSPH: ' + extraData;
                label += '\nShots: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};





  // Get the reference to the parent element
var parentElement = document.getElementById('forwardShotsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_Shots_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_Shots_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardShotsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );
                    break;
                }
                case 2: {
                    const sortedData = forwardShots.sort((a, b) => b.SPH - a.SPH);
    
                    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
                    const amountsArray = sortedData.map(obj => obj.SPH);
                    const posArray = sortedData.map(obj => obj.position);
                    const apgArray = sortedData.map(obj => obj.SP);
                    const aphArray = sortedData.map(obj => obj.amount);
                
                    const data = {
                        labels: namesArray,
                        datasets: [
                            {
                                label: 'SPH',
                                backgroundColor: colors.primary,
                                borderColor: colors.secondary,
                                borderWidth: 1.5,
                                data: amountsArray,
                                tooltipExtraData: posArray, 
                                tooltipExtraData1: apgArray, 
                                tooltipExtraData2: aphArray, 
                            }
                        ]
                    };
                
                   
                
                // Configuration
                const config = {
                    type: 'bar',
                    data: data,
                    options: {
                        maintainAspectRatio: false, // Set to false to allow custom width and height
                        responsive: true,
                        scales: {
                            x: {
                                stacked: true,
                            },
                            y: {
                                stacked: true,
                               
                            }
                        },
                        plugins: {
                            datalabels: {
                                anchor: 'end',
                                align: 'end',
                                formatter: function(value, context) {
                                    return value; // Display the value as label
                                }
                            },
                            legend: {
                                display: false
                            },
                            tooltip: {
                                callbacks: {
                                    title: function(tooltipItems) {
                                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                                        return tooltipItems[0].label + ' - ' + extraData;
                                      },
                                    label: function(context) {
                                       // Accessing extra tooltip data
                                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                                let label = context.dataset.label + ': ' + context.parsed.y;
                                // Append additional text with line break
                                label += '\nS: ' + extraData + "%";
                                label += '\nShots: ' + extraData2;
                                return label;
                                    }
                                }
                            }
                        }
                    }
                };
                
                
                
                
                
                  // Get the reference to the parent element
                var parentElement = document.getElementById('forwardShotsChart_Parent');
                
                // Check if there is an old canvas
                var oldCanvas = document.getElementById('forwards_Shots_chart');
                if (oldCanvas) {
                    // If old canvas exists, remove it
                    parentElement.removeChild(oldCanvas);
                }
                
                      // Get the canvas element
                      var canvas = document.createElement('canvas');
                      canvas.id = 'forwards_Shots_chart';
                      //canvas.width = 400;
                      //canvas.height = 200;
                      document.getElementById('forwardShotsChart_Parent').appendChild(canvas);
                    
                      // Get the canvas context
                      var ctx = canvas.getContext('2d');
                
                      
                
                    // Create Chart
                    var myChart = new Chart(
                        ctx,
                        config
                    );
                    break;
                }
                default: {
                    console.log("Invalid local index for Chart 3");
                    break;
                }
            }
            break;
        }
        default: {
            console.log("Invalid chart ID");
            break;
        }
    }
}


  function getTeamName(index) {
    const teams = [
        "Anaheim Ducks", "Arizona Coyotes", "Boston Bruins", "Buffalo Sabres", "Calgary Flames",
        "Carolina Hurricanes", "Chicago Blackhawks", "Colorado Avalanche", "Columbus Blue Jackets",
        "Dallas Stars", "Detroit Red Wings", "Edmonton Oilers", "Florida Panthers", "Los Angeles Kings",
        "Minnesota Wild", "Montréal Canadiens", "Nashville Predators", "New Jersey Devils", "New York Islanders",
        "New York Rangers", "Ottawa Senators", "Philadelphia Flyers", "Pittsburgh Penguins", "San Jose Sharks",
        "Seattle Kraken", "St. Louis Blues", "Tampa Bay Lightning", "Toronto Maple Leafs", "Vancouver Canucks",
        "Vegas Golden Knights", "Washington Capitals", "Winnipeg Jets"
    ];

    if (index >= 0 && index < teams.length) {
        return teams[index];
    } else {
        return "Index out of range";
    }
}


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

let forwardGames = []; 
let forwardPoints = [];
let forwardGoals = [];
let forwardAssists = [];
let forwardShots= [];
function create_forwards_charts(forwards, colors){

    forwardGames = [];
    for (const skater of forwards) {
        let name = skater.firstName.default + " " + skater.lastName.default;
        let points = skater.gamesPlayed;
        let position = skater.positionCode;
        let m_toi = skater.avgTimeOnIcePerGame / 60;
        let m_pm = skater.plusMinus;
        forwardGames.push({
            label: name,
            amount: points,
            position: position,
            TOI: m_toi.toFixed(2),
            PM: m_pm.toFixed(0)

        });
    }
    
    const sortedData = forwardGames.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.TOI);
    const aphArray = sortedData.map(obj => obj.PM);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Games Played',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nTOI: ' + extraData;
                label += '\n+/-: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};





  // Get the reference to the parent element
var parentElement = document.getElementById('totalChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('games_played_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'games_played_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('totalChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );


    
}


function create_forwards_goals_charts(forwards, colors){
    forwardGoals = [];

    for (const skater of forwards) {
        let name = skater.firstName.default + " " + skater.lastName.default;
        let goals = skater.goals;
        let position = skater.positionCode;
        let apg = goals / skater.gamesPlayed;
        let aph = goals / ((skater.gamesPlayed * (skater.avgTimeOnIcePerGame / 60)) / 60);
        forwardGoals.push({
            label: name,
            amount: goals,
            position: position,
            goalsPerGame: apg.toFixed(2),
            goalsPerHour: aph.toFixed(2)

        });
    }
    
    const sortedData = forwardGoals.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.goalsPerGame);
    const aphArray = sortedData.map(obj => obj.goalsPerHour);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Goals',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nGPG: ' + extraData;
                label += '\nGPH: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};
  // Get the reference to the parent element
var parentElement = document.getElementById('forwardGoalChart_Parent');
// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_goal_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}
      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_goal_chart';
      document.getElementById('forwardGoalChart_Parent').appendChild(canvas);
      var ctx = canvas.getContext('2d');
    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );
}

function create_forwards_points_charts(forwards, colors){
    forwardPoints = [];

    for (const skater of forwards) {
        let name = skater.firstName.default + " " + skater.lastName.default;
        let points = skater.points;
        let position = skater.positionCode;
        let apg = points / skater.gamesPlayed;
        let aph = points / ((skater.gamesPlayed * (skater.avgTimeOnIcePerGame / 60)) / 60);
        forwardPoints.push({
            label: name,
            amount: points,
            position: position,
            PPG: apg.toFixed(2),
            PPH: aph.toFixed(2)

        });
    }
    
    const sortedData = forwardPoints.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.PPG);
    const aphArray = sortedData.map(obj => obj.PPH);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Points',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nPPG: ' + extraData;
                label += '\nPPH: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};






  // Get the reference to the parent element
var parentElement = document.getElementById('forwardPointsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_point_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_point_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardPointsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );


    
}



function create_assist_points_charts(forwards, colors){
    forwardAssists = [];

    for (const skater of forwards) {
        let name = skater.firstName.default + " " + skater.lastName.default;
        let assists = skater.assists;
        let position = skater.positionCode;
        let apg = assists / skater.gamesPlayed;
        let aph = assists / ((skater.gamesPlayed * (skater.avgTimeOnIcePerGame / 60)) / 60);
        forwardAssists.push({
            label: name,
            amount: assists,
            position: position,
            APG: apg.toFixed(2),
            APH: aph.toFixed(2)

        });
    }
    
    const sortedData = forwardAssists.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.APG);
    const aphArray = sortedData.map(obj => obj.APH);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Assists',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nAPG: ' + extraData;
                label += '\nAPH: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};







  // Get the reference to the parent element
var parentElement = document.getElementById('forwardAssistsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_Assists_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_Assists_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardAssistsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );


    
}

function create_shot_points_charts(forwards, colors){
    forwardShots = [];

    for (const skater of forwards) {
        let name = skater.firstName.default + " " + skater.lastName.default;
        let shots = skater.shots;
        let position = skater.positionCode;
        let apg = skater.shootingPctg * 100;
        let aph = shots / ((skater.gamesPlayed * (skater.avgTimeOnIcePerGame / 60)) / 60);
        forwardShots.push({
            label: name,
            amount: shots,
            position: position,
            SP: apg.toFixed(1),
            SPH: aph.toFixed(2)

        });
    }
    
    const sortedData = forwardShots.sort((a, b) => b.amount - a.amount);
    
    const namesArray = sortedData.map(obj => obj.label); // Corrected property name
    const amountsArray = sortedData.map(obj => obj.amount);
    const posArray = sortedData.map(obj => obj.position);
    const apgArray = sortedData.map(obj => obj.SP);
    const aphArray = sortedData.map(obj => obj.SPH);

    const data = {
        labels: namesArray,
        datasets: [
            {
                label: 'Shots',
                backgroundColor: colors.primary,
                borderColor: colors.secondary,
                borderWidth: 1.5,
                data: amountsArray,
                tooltipExtraData: posArray, 
                tooltipExtraData1: apgArray, 
                tooltipExtraData2: aphArray, 
            }
        ]
    };

   

// Configuration
const config = {
    type: 'bar',
    data: data,
    options: {
        maintainAspectRatio: false, // Set to false to allow custom width and height
        responsive: true,
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true,
               
            }
        },
        plugins: {
            datalabels: {
                anchor: 'end',
                align: 'end',
                formatter: function(value, context) {
                    return value; // Display the value as label
                }
            },
            legend: {
                display: false
            },
            tooltip: {
                callbacks: {
                    title: function(tooltipItems) {
                        let extraData = tooltipItems[0].dataset.tooltipExtraData[tooltipItems[0].dataIndex];
                        return tooltipItems[0].label + ' - ' + extraData;
                      },
                    label: function(context) {
                       // Accessing extra tooltip data
                let extraData = context.dataset.tooltipExtraData1[context.dataIndex];
                let extraData2 = context.dataset.tooltipExtraData2[context.dataIndex];
                let label = context.dataset.label + ': ' + context.parsed.y;
                // Append additional text with line break
                label += '\nS: ' + extraData + "%";
                label += '\nSPH: ' + extraData2;
                return label;
                    }
                }
            }
        }
    }
};





  // Get the reference to the parent element
var parentElement = document.getElementById('forwardShotsChart_Parent');

// Check if there is an old canvas
var oldCanvas = document.getElementById('forwards_Shots_chart');
if (oldCanvas) {
    // If old canvas exists, remove it
    parentElement.removeChild(oldCanvas);
}

      // Get the canvas element
      var canvas = document.createElement('canvas');
      canvas.id = 'forwards_Shots_chart';
      //canvas.width = 400;
      //canvas.height = 200;
      document.getElementById('forwardShotsChart_Parent').appendChild(canvas);
    
      // Get the canvas context
      var ctx = canvas.getContext('2d');

      

    // Create Chart
    var myChart = new Chart(
        ctx,
        config
    );

}

const getOrCreateTooltip = (chart) => {
    let tooltipEl = chart.canvas.parentNode.querySelector('div');
  
    if (!tooltipEl) {
      tooltipEl = document.createElement('div');
      tooltipEl.style.background = 'rgba(0, 0, 0, 0.7)';
      tooltipEl.style.borderRadius = '3px';
      tooltipEl.style.color = 'white';
      tooltipEl.style.opacity = 1;
      tooltipEl.style.pointerEvents = 'none';
      tooltipEl.style.position = 'absolute';
      tooltipEl.style.transform = 'translate(-50%, 0)';
      tooltipEl.style.transition = 'all .1s ease';
  
      const table = document.createElement('table');
      table.style.margin = '0px';
  
      tooltipEl.appendChild(table);
      chart.canvas.parentNode.appendChild(tooltipEl);
    }
  
    return tooltipEl;
  };
  
  const externalTooltipHandler = (context) => {
    // Tooltip Element
    const {chart, tooltip} = context;
    const tooltipEl = getOrCreateTooltip(chart);
  
    // Hide if no tooltip
    if (tooltip.opacity === 0) {
      tooltipEl.style.opacity = 0;
      return;
    }
  
    // Set Text
    if (tooltip.body) {
      const titleLines = tooltip.title || [];
      const bodyLines = tooltip.body.map(b => b.lines);
  
      const tableHead = document.createElement('thead');
  
      titleLines.forEach(title => {
        const tr = document.createElement('tr');
        tr.style.borderWidth = 0;
  
        const th = document.createElement('th');
        th.style.borderWidth = 0;
        const text = document.createTextNode(title);
  
        th.appendChild(text);
        tr.appendChild(th);
        tableHead.appendChild(tr);
      });
  
      const tableBody = document.createElement('tbody');
      bodyLines.forEach((body, i) => {
        const colors = tooltip.labelColors[i];
  
        const span = document.createElement('span');
        span.style.background = colors.backgroundColor;
        span.style.borderColor = colors.borderColor;
        span.style.borderWidth = '2px';
        span.style.marginRight = '10px';
        span.style.height = '10px';
        span.style.width = '10px';
        span.style.display = 'inline-block';
  
        const tr = document.createElement('tr');
        tr.style.backgroundColor = 'inherit';
        tr.style.borderWidth = 0;
  
        const td = document.createElement('td');
        td.style.borderWidth = 0;
  
        const text = document.createTextNode(body);
  
        td.appendChild(span);
        td.appendChild(text);
        tr.appendChild(td);
        tableBody.appendChild(tr);
      });
  
      const tableRoot = tooltipEl.querySelector('table');
  
      // Remove old children
      while (tableRoot.firstChild) {
        tableRoot.firstChild.remove();
      }
  
      // Add new children
      tableRoot.appendChild(tableHead);
      tableRoot.appendChild(tableBody);
    }
  
    const {offsetLeft: positionX, offsetTop: positionY} = chart.canvas;
  
    // Display, position, and set styles for font
    tooltipEl.style.opacity = 1;
    tooltipEl.style.left = positionX + tooltip.caretX + 'px';
    tooltipEl.style.top = positionY + tooltip.caretY + 'px';
    tooltipEl.style.font = tooltip.options.bodyFont.string;
    tooltipEl.style.padding = tooltip.options.padding + 'px ' + tooltip.options.padding + 'px';
  };

  function changeTextAndColor(title, color, color2) {
    // Change text inside the h1 element
    document.getElementById("title").textContent = title;

    // Change CSS variable value
    document.documentElement.style.setProperty('--main-color', color);
    document.documentElement.style.setProperty('--second-color', color2);
  }