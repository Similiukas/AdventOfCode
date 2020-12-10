import 'dart:io';
import 'dart:math';

import '../helpers.dart';

// Part one
Future<int> notXMAS() async {
  var file = File('input/day9/input.txt');
  List<int> inputStream = (await file.readAsLines()).map(int.parse).toList();
  List<int> preamble = await readFileIntLines('input/day9/input.txt', 25);  // First 25 lines are preamble
  int pointer = -1;
  for(int number in inputStream.sublist(25)){                               // Continuing to read file from line 26
    bool followsRules = false;
    for (int tester in preamble){                 // Checking if given number could be the sum of two numbers from preamble
      if (number - tester < 0) continue;          // Does this even make it faster?
      if (preamble.contains((number - tester))){
        followsRules = true;
        break;
      }
    }
    if (!followsRules)    return number;
    preamble[(++pointer) % 25] = number;          // Adding the number to the preamble by not shifting array but just replacing last element
  }
  return -1;
}

// Part two
Future<int> contigousSet(int defect) async {
  var sum = 0, smallest = defect, largest = 0, pointer = 0;
  List<int> input = await readFileInt('input/day9/input.txt');  // Would be better to read files up to defect value
  // Going through file and adding all the numbers until they are bigger than defect. Then restarting from the next number. Very not ideal, pretty much brute forcing
  for (int i = 0; input[i] < defect; ++i){
    int line = input[i];
    if (sum < defect){
      sum += line;
      smallest = min(smallest, line);
      largest = max(largest, line);
    }
    else if(sum > defect){
      i = ++pointer;      // Since the sum is bigger, resetting i to the value it started + 1
      sum = 0; smallest = defect; largest = 0;
    }
    else return smallest + largest;
  }
  return -1;
}

void day9() async {
  print('Definitely starting to feel ://');
  // Part one
  int followsNot = await notXMAS();
  print('The one who does not like the rules: $followsNot');
  // Part two
  int setSum = await contigousSet(followsNot);
  print('Smallest and largest set numbers sum: $setSum');
}