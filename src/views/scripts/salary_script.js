var app = angular.module('MyApp', ['ngAnimate']);
app.controller('AppController', function ($scope){
  $scope.msg = "Don't give up!"
  
 
  $scope.players = [
    {
    playerName: "Damian Lillard",
    playerStatus: "Guaranteed",
    playerSalary: "24420000",
  },
    {
    playerName: "Al-Farouq Aminu",
    playerStatus: "Guaranteed",
    playerSalary: "7680965",
  },
    {
    playerName: "Ed Davis",
    playerStatus: "Guaranteed",
    playerSalary: "6666667",
  },
    {
    playerName: "CJ McCollum",
    playerStatus: "Guaranteed",
    playerSalary: "3219579",
  },
    {
    playerName: "Noah Vonleh",
    playerStatus: "Guaranteed",
    playerSalary: "2751360",
  },
    {
    playerName: "Mason Plumlee",
    playerStatus: "Guaranteed",
    playerSalary: "2328530",
  },
    {
    playerName: "Pat Connaughton",
    playerStatus: "Guaranteed",
    playerSalary: "874636",
  },
    {
    playerName: "Gerald Henderson",
    playerStatus: "Cap Hold",
    playerSalary: "9000000",
  },
    {
    playerName: "Meyers Leonard",
    playerStatus: "Cap Hold",
    playerSalary: "7689700",
  },
    {
    playerName: "Moe Harkless",
    playerStatus: "Cap Hold",
    playerSalary: "7235148",
  },
    {
    playerName: "Chris Kaman",
    playerStatus: "Cap Hold",
    playerSalary: "6520800",
  },
    {
    playerName: "Brian Roberts",
    playerStatus: "Cap Hold",
    playerSalary: "3711422",
  },
    {
    playerName: "Allen Crabbe",
    playerStatus: "Cap Hold",
    playerSalary: "2725003",
  },
     {
    playerName: "Luis Montero",
    playerStatus: "Unguaranteed",
    playerSalary: "874636",
  },
    {
    playerName: "Cliff Alexander",
    playerStatus: "Unguaranteed",
    playerSalary: "874636",
  },
    
  ]
  $scope.remaining = 0;
  $scope.total = 0;
  $scope.updateTotal = function () {
    $scope.total = 0;
    // Add roster charges if less than 12 players
    if ($scope.players.length < 12){
      $scope.total += (12 - $scope.players.length) * 543471
    }
    
    angular.forEach($scope.players, function (obj){
$scope.total += Number(obj.playerSalary);      
    })
    $scope.total += 1984005;
    $scope.remaining = 94000000 - $scope.total;
  }
  $scope.addNew = function(){
    $scope.players.push({
    playerName: "New Player",
    playerStatus: "New",
    playerSalary: "543471",
  })
  }
  $scope.deletePlayer = function(id){
    $scope.players.splice(id, 1);
  }
})