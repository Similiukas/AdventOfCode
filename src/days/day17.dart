import 'dart:io';
import 'dart:convert';

// Return of the day 11
int checkAdjacentConway(var grid, int z, int y, int x){
  int found = 0;
  for (int Z in [-1, 0, 1]){      // Going up and down
    for (int Y in [-1, 0, 1]){    // Going deep and near, idk, just y axis
      for (int X in [-1, 0, 1]){  // Going left and right
        if (X == 0 && Y == 0 && Z == 0) continue; // Skipping the given one
        if (grid[z + Z][y + Y][x + X] == '#') found++;
      }
    }
  }
  return found;
}

// Part one
List<dynamic> conwaySimulation(var grid, int simulation, int inputSize){
  var tempGrid = [];      // Deep copy of grid. Ugly way and must be a simplier way of doing it
  for (int z = 0; z < grid.length; ++z){
    tempGrid.add([]);
    for (int y = 0; y < grid[0].length; ++y){
      tempGrid[z].add([]);
      for (int x = 0; x < grid[0].length; ++x){
        tempGrid[z][y].add(grid[z][y][x]);
      }
    }
  }
  // Ugly iteration but pretty much not going through every single one symbol. Only going by simulation number
  for (int Z = (grid.length ~/ 2) - simulation; Z <= (grid.length ~/ 2) + simulation; ++Z){
    for (int Y = ((grid[0].length - inputSize) ~/ 2) - simulation; Y <= (grid[0].length ~/ 2 + inputSize ~/ 2) + simulation; ++Y){
      for (int X = ((grid[0].length - inputSize) ~/ 2) - simulation; X <= (grid[0].length ~/ 2 + inputSize ~/ 2) + simulation; ++X){
        int activeAdjacent = checkAdjacentConway(grid, Z, Y, X);
        if (grid[Z][Y][X] == '#' && (activeAdjacent != 3 && activeAdjacent != 2)){
          tempGrid[Z][Y][X] = '.';
        }
        else if (grid[Z][Y][X] == '.' && activeAdjacent == 3){
          tempGrid[Z][Y][X] = '#';
        }
      }
    }
  }
  return tempGrid;
}

// Part one
Future<int> activeCubesIn3D(int inputSize) async{
  var grid = [];    // ZYX
  for (int z = 0; z <= 14; ++z){  // Creating a grid of inactive robots
    grid.add(List.generate(2 * 6 + inputSize + 2, (x) => List.generate(2 * 6 + inputSize + 2, (index) => '.')));
  }
  var file = File('input/day17/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  int y = grid[0].length ~/ 2 - inputSize ~/2;  // Filling the input in the middle of the grid
  await for (var line in inputStream.transform(utf8.decoder).transform(LineSplitter())){
    int x = grid[0].length ~/ 2 - inputSize ~/2;
    for (var symbol in line.split('')){
      grid[7][y][x] = symbol;
      x++;
    }
    y++;
  }
  // Simulating
  for (int sim = 1; sim <= 6; ++sim){
    grid = conwaySimulation(grid, sim, inputSize);
  }
  // Printing the cubes after simulations (looks quite nice)
  for (int z = 1; z <= 13; ++z){
    print('z = ${z - 7}');
    grid[z].forEach(print);
  }
  // Counting active ones
  int active = 0;
  for (int z = 1; z <= 13; ++z){
    grid[z].forEach((element) {element.forEach( (symbol) {active += symbol == '#' ? 1 : 0;});});
  }
  return active;
}

void day17() async{
  print('Oof, F for that quote there');
  int active3D = await activeCubesIn3D(9);     // Input size is size of given input grid (+1 if even)
  print('There are $active3D active cubes in 3D');
  // Part two actually might be simple, just adding another dimension to array before Z and iterating over it
  // Don't know if I'll do it, the puzzles are taking a lot of time already so might have to leave it for good
  // Or I might come back and finish it
}