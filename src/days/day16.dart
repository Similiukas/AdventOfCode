import 'dart:io';
import 'dart:convert';

import '../helpers.dart';

List<List<int>> ticketFields = [];  // Part two
List<int> validTickets;             // Part two

// Part one
Future<List<List<int>>> readTicketFields() async{
  var file = File('input/day16/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  List<List<int>> ranges = [];
  await for (var line in inputStream.transform(utf8.decoder).transform(LineSplitter())){
    if (line.isEmpty) break;
    var numbers = line.split(RegExp(r'\D+')).map(int.tryParse).toList();
    for (int i = 1; i < 4; i+=2){
      ranges.add([]);
      ranges[ranges.length - 1].add(numbers[i]);
      ranges[ranges.length - 1].add(numbers[i+1]);
    }
    ticketFields.add([]);   // Part two
  }
  // Part two, generating one and then copying
  ticketFields[0] = List<int>.generate(ticketFields.length, (index) => index);
  for (int i = 1; i < ticketFields.length; ++i) ticketFields[i] = [...ticketFields[0]];
  return ranges;
}

bool checkBounds(int checker, int lowerBound, int upperBound){
  return checker >= lowerBound && checker <= upperBound;
}

// Part one
Future<int> checkInvalidTrainTickets(List<List<int>> ranges) async{
  List<String> input = await readFileString('input/day16/input.txt');
  int sum = 0;
  validTickets = List<int>.generate((input.length - (ranges.length ~/ 2) - 5), (index) => index + ((ranges.length ~/ 2) + 5));
  for (var line in input.sublist((ranges.length ~/ 2) + 5)){  // Skipping first lines
    for (int check in line.split(',').map(int.parse)){
      int boundIndex = 0;
      while (boundIndex < ranges.length - 1 && !checkBounds(check, ranges[boundIndex][0], ranges[boundIndex][1])) boundIndex++;
      // If ticket is invalid
      if (!checkBounds(check, ranges[boundIndex][0], ranges[boundIndex][1])){
        validTickets.remove(input.indexOf(line));
        sum += check;
        break;
      }
    }
  }
  return sum;
}

// Part two
Future<int> translateTicketInfo(List<List<int>> ranges) async{
  List<String> input = await readFileString('input/day16/input.txt');
  var myTicket = input[ranges.length~/2 + 2].split(',').map(int.tryParse).toList();
  // Most probably could simplify these 3 loops and just use 1 type of loop instead of simple for and for ... in but have stuff to do. Kind of
  // Checking if ticket field can be in nth column, where 0 <= n < number of ticket values
  for (var ticketIndex in validTickets){  // Going through valid tickets
    var ticket = input[ticketIndex].split(',').map(int.parse).toList();
    for (var fieldIndex = 0; fieldIndex < ranges.length; fieldIndex+=2){  // Going through fields ranges
      for (int ticketValue in ticket){    // Going through ticket values
        // If ticket value is not in range of the field, then that field can't be in nth column
        if (!checkBounds(ticketValue, ranges[fieldIndex][0], ranges[fieldIndex][1]) && !checkBounds(ticketValue, ranges[fieldIndex + 1][0], ranges[fieldIndex + 1][1])){
          ticketFields[fieldIndex~/2].remove(ticket.indexOf(ticketValue));
        }
      }
    }
  }

  // Tried to do with set differences but just wanted to do this quickly and move on for the day. And listen to goofy goober <3
  var valuesRemoved = [];
  for (int i = 0; i < ticketFields.length; ++i){
    int valueToRemove;
    for (var arr in ticketFields){    // A value to remove is in array of length 1 and hasn't been already removed
      if (arr.length == 1 && !valuesRemoved.contains(arr[0])){
        valuesRemoved.add(arr[0]);
        valueToRemove = arr[0];
      }
    }
    for (var arr in ticketFields){    // Removing the value from all arrays
      if (arr.length != 1) arr.remove(valueToRemove);
    }
  }
  // Now ticketFields arrays contain 1 value each which are indexes of which column the field represents
  num ans = 1;
  for (var i in ticketFields.sublist(0, 6)){
    ans *= myTicket[i.first];
  }
  return ans;
}

void day16() async{
  print('She kinda nice and the ticket is a bit bigger');
  // Part one
  var ranges = await readTicketFields();  // Ranges of fields
  int invalidSum = await checkInvalidTrainTickets(ranges);
  print('Invalid ticket sum: $invalidSum');
  // Part two
  int firstSixMult = await translateTicketInfo(ranges);
  print('Departures multiplication: $firstSixMult');
}