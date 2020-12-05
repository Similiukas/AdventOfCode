import 'dart:io';
import 'dart:convert';
import 'dart:math';

// Just part 1. Would be better to do fillSeats() here as well but leaving it like that
Future<int> findMaxID() async{
  var file = new File('input/day5/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  int maxID = 0;
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    int row = binaryFind(0, 127, line.substring(0, 7), 'F', 'B');
    int column = binaryFind(0, 7, line.substring(7), 'L', 'R');
    maxID = max(maxID, (row * 8 + column));
  }
  return maxID;
}

Future<int> mySeat() async {
  List<List<int>> seats = await fillSeats();
  bool startingSeats = false;   // Ignoring first seats which are null
  for (int row = 0; row < 127; ++row){
    for (int column = 0; column < 7; ++column){
      if (startingSeats && seats[row][column] == null){
        return row * 8 + column;
      }
      else if (!startingSeats && seats[row][column] != null) {
        startingSeats = true;
      }
    }
  }
  return -1;
}

Future<List<List<int>>> fillSeats() async{
  var file = new File('input/day5/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  List<List<int>> seats = List.generate(128, (index) => List(8));   // List of 128 rows and 8 columns (null values)
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    int row = binaryFind(0, 127, line.substring(0, 7), 'F', 'B');
    int column = binaryFind(0, 7, line.substring(7), 'L', 'R');
    seats[row][column] = 1;
  }
  return seats;
}

int binaryFind(int lowerBound, int upperBound, String letters, String lowerLetter, String upperLetter){
  letters.split('').forEach((letter) {
    int mid = (upperBound - lowerBound) ~/ 2 + lowerBound;
    if (letter == lowerLetter) {
      upperBound = mid;
    }
    else if(letter == upperLetter){
      lowerBound = mid + 1;
    }
  });
  return lowerBound;
}

void day5() async{
  print('Starting to get into habit maybe?');
  // Part one
  int maxID = await findMaxID();
  print('Max ID: $maxID');
  // Part two
  int myID = await mySeat();
  print('My ID: $myID');
}