
const invoke = window.__TAURI__.invoke
window.onload = () => {
  fetchSchedule(); // Call fetchStruct function when window loads
};

// Assuming this code is inside an async function
function fetchSchedule() {
  invoke('build_schedule_struct')
  .then((message) => console.log(message.games[0].time))
  .catch((error) => console.error(error))
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
