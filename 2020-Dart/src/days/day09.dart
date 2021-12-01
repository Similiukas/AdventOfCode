import 'dart:io';
import 'dart:math';

import '../helpers.dart';

// Part one
Future<int> notXMAS() async {
  var file = File('input/day09/input.txt');
  List<int> inputStream = (await file.readAsLines()).map(int.parse).toList();
  List<int> preamble = await readFileIntLines('input/day09/input.txt', 25);  // First 25 lines are preamble
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
  List<int> input = await readFileInt('input/day09/input.txt');  // Would be better to read files up to defect value
  // Going through file and adding all the numbers until they are bigger than defect. Then restarting from the next number. Very not ideal, pretty much brute forcing
  for (int i = 0; input[i] < defect; ++i){
    int line = input[i];
    if (sum < defect){
      sum += line;
      smallest = min(smallest, line);
      largest = max(largest, line);
    }
    else if(sum > defect){
      do {                      // Since sum is bigger than defect, need to decrease it until it's not
        sum -= input[pointer++];// Decreasing from the first elements added to imaginary set (since the set needs to be contigous)
        smallest = input.sublist(pointer, i).reduce(min); // Need to find min and max again
        largest = input.sublist(pointer, i).reduce(max);
      } while (sum > defect);
      i--;                                                // If sum is not found, need to check if the last element since adding it might get the sum
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