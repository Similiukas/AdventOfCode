import '../helpers.dart';

bool changed = true;

// Part one (very ugly but just use part two implementation)
bool checkAdjacent(int y, int x, int howMany, List<List<String>> seats){
  int found = 0;
  if ((x + 1) < seats[0].length && seats[y][x+1] == '#')  found++;  // right
  if ((x - 1) >= 0 && seats[y][x-1] == '#')               found++;  // left
  if ((y + 1) < seats.length && seats[y+1][x] == '#')     found++;  // down
  if ((y - 1) >= 0 && seats[y-1][x] == '#')               found++;  // up

  if ((y + 1) < seats.length && (x + 1) < seats[0].length && seats[y+1][x+1] == '#')  found++;  // diagonal down right
  if ((y + 1) < seats.length && (x - 1) >= 0 && seats[y+1][x-1] == '#')               found++;  // diagonal down left
  if ((y - 1) >= 0 && (x + 1) < seats[0].length && seats[y-1][x+1] == '#')            found++;  // diagonal up right
  if ((y - 1) >= 0 && (x - 1) >= 0 && seats[y-1][x-1] == '#')                         found++;  // diagonal up left
  if (found >= howMany) return true;
  return false;
}

// Part two
bool checkAdjacentLoop(int y, int x, int howMany, List<List<String>> seats){
  // Left first implementation as comment (yeah, ugly, I know but does the job)
  // for (int X = x + 1; X < seats[0].length; ++X){  // right
  //   if (seats[y][X] == '#'){
  //     // print('right');
  //     found++;
  //     break;
  //   }
  //   else if (seats[y][X] == 'L'){
  //     break;
  //   }
  // }
  // for (int X = x - 1; X >= 0; --X){               // left
  //   if (seats[y][X] == '#'){
  //     // print('left');
  //     found++;
  //     break;
  //   }
  //   else if (seats[y][X] == 'L'){
  //     break;
  //   }
  // }
  // for (int Y = y + 1; Y < seats.length; ++Y){     // down
  //   if (seats[Y][x] == '#'){
  //     // print('down');
  //     found++;
  //     break;
  //   }
  //   else if (seats[Y][x] == 'L'){
  //     break;
  //   }
  // }
  // for (int Y = y - 1; Y >= 0; --Y){               // up
  //   if (seats[Y][x] == '#'){
  //     // print('up');
  //     found++;
  //     break;
  //   }
  //   else if (seats[Y][x] == 'L'){
  //     break;
  //   }
  // }
  // int X = x + 1, Y = y + 1;
  // while (X < seats[0].length && Y < seats.length && seats[Y][X] != 'L'){  // down right
  //   if (seats[Y][X] == '#'){
  //     // print('down right');
  //     found++;
  //     break;
  //   }
  //   X++; Y++;
  // }
  // X = x - 1; Y = y + 1;
  // while (X >= 0 && Y < seats.length && seats[Y][X] != 'L'){               // down left
  //   if (seats[Y][X] == '#'){
  //     // print('down left');
  //     found++;
  //     break;
  //   }
  //   X--; Y++;
  // }
  // X = x + 1; Y = y - 1;
  // while (X < seats[0].length && Y >= 0 && seats[Y][X] != 'L'){            // up right
  //   if (seats[Y][X] == '#'){
  //     // print('up right');
  //     found++;
  //     break;
  //   }
  //   X++; Y--;
  // }
  // X = x - 1; Y = y - 1;
  // while (X >= 0 && Y >= 0 && seats[Y][X] != 'L'){                         // up left
  //   if (seats[Y][X] == '#'){
  //     // print('up left');
  //     found++;
  //     break;
  //   }
  //   X--; Y--;
  // }
  // Credit to u/sophiebits
  int found = 0;
  for (int Y in [-1, 0, 1]){    // Going up and down
    for (int X in [-1, 0, 1]){  // Going left and right
      if (X == 0 && Y == 0) continue; // Skipping the given one
      int mult = 1;             // Going the same direction multiple times
      // For part one, no need of while loop, just simply check 0 <= y + Y < length and 0 <= x + X < width 
      while ((0 <= y + mult*Y  && y + mult*Y < seats.length )   &&    // 0 <= (y + mult * Y) < row length
             (0 <= x + mult*X  && x + mult*X < seats[0].length) &&    // 0 <= (x + mult * X) < column width
             (seats[y + mult*Y][x + mult*X] != 'L')){                 // Didn't bump into empty seat because people don't see past seats (can see only first seat)
        if (seats[y + mult*Y][x + mult*X] == '#'){
          found++;
          break;
        }
        mult++;
      }
    }
  }
  if (found >= howMany) return true;
  return false;
}

List<List<String>> simulateSeatChange(List<List<String>> seats){
  // var temp = List<List<String>>.from(seats);
  // Dart is kinda dumb, can make a clone of one dimensional array but no way of deepcopy ://
  var temp = <List<String>>[];                // Pretty much making a deepcopy of input
  for (int i = 0; i < seats.length; ++i){
    temp.add([]);
    for (int j = 0; j < seats[0].length; ++j){
      temp[i].add(seats[i][j]);
    }
  }
  changed = false;                            // If after simulation nothing changed, then the equilibrium has been made
  for (int i = 0; i < seats.length; ++i){     // Going through evey seat and looking if need to change it
    for (int j = 0; j < seats[0].length; ++j){
      switch (seats[i][j]){
        case 'L':
          if (!checkAdjacentLoop(i, j, 1, seats)){
            temp[i][j] = '#';
            changed = true;
          }
          break;
        case '#':
          if (checkAdjacentLoop(i, j, 5, seats)) {
            temp[i][j] = 'L';
            changed = true;
          }
          break;
      }
    }
  }
  return temp;
}

Future<int> checkSeats() async{
  List<String> input = await readFileString('input/day11/input.txt');
  List<List<String>> seats = [...input.map((e) => e.split(''))];  // Splitting String into List<String> since dart no change string
  int howManyLs = 0;
  while (changed){
    seats = simulateSeatChange(seats);
  }
  seats.forEach((lst) {lst.forEach((e) {howManyLs += (e == '#') ? 1 : 0;});});  // Counting occupied seats
  return howManyLs;
}

void day11() async{
  print('I shouldn\'t give myself more hope :/');
  int occupied = await checkSeats();
  print('After simulations [$occupied] seats are occupied');
}