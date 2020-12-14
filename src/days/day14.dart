import 'dart:io';
import 'dart:convert';
import 'dart:math';

// Part one
int addBinaryToMask(List<String> mask, int number){
  List<String> maskTemp = [...mask];
  List<String> binary = number.toRadixString(2).padLeft(36, '0').split(''); // Converting to binary and adding 0
  for (int i = 0; i < 36; ++i){
    if (maskTemp[i] == 'X'){  // If it's not X, then leaving the mask
      maskTemp[i] = binary[i];
    }
  }
  return int.tryParse(maskTemp.join(''), radix:2);  // Returning back decimal
}

Future<int> initializeFerryMask() async{
  var file = File('input/day14/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  var currentMask = <String>[];
  var memory = <int, int>{};
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    if (line.substring(0, 4) == 'mask'){
      currentMask = line.substring(7).split('');
    }
    else{
      var regex = new RegExp(r'mem\[(\d+)\] = (\d+)').firstMatch(line);
      int memoryPosition = int.parse(regex.group(1));
      int numberToStore = int.parse(regex.group(2));
      memory[memoryPosition] = addBinaryToMask(currentMask, numberToStore);
    }
  }
  return memory.values.reduce((value, element) => value + element);
}

// Part two
List<int> bitmaskedMemoryAddress(List<String> mask, int memoryPosition){
  List<String> binary = memoryPosition.toRadixString(2).padLeft(36, '0').split(''); // Creating a binary array
  List<String> result = [];   // Bitmasked result
  var indexOfX = <int>[];
  for (int i = 0; i < 36; ++i){
    if (mask[i] == '0')       result.add(binary[i]);
    else if (mask[i] == '1')  result.add('1');
    else {
      indexOfX.add(i);
      result.add('X');      // Can add whatever here
    }
  }
  var memoryAddresses = <int>[];  // All possible memory addresses
  int binaryAdditionSum = 0;
  // // Going through all possible combinations by adding binary number for 2 X: 00 -> 01 -> 10 -> 11
  for (int i = 0; i < pow(2, indexOfX.length); ++i){
    var binaryPermutation = binaryAdditionSum.toRadixString(2).padLeft(indexOfX.length, '0').split('');
    for (int j = 0; j < binaryPermutation.length; ++j){
      result[indexOfX[j]] = binaryPermutation[j];
    }
    memoryAddresses.add(int.tryParse(result.join(''), radix:2));  // Converting binary to decimal
    binaryAdditionSum++;
  }
  return memoryAddresses;
}

Future<int> initializeFerryMaskVersion2() async{
  var file = File('input/day14/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  var currentMask = <String>[];
  var memory = <int, int>{};
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    if (line.substring(0, 4) == 'mask'){
      currentMask = line.substring(7).split('');
    }
    else{
      var regex = new RegExp(r'mem\[(\d+)\] = (\d+)').firstMatch(line);
      int memoryPosition = int.parse(regex.group(1));
      int numberToStore = int.parse(regex.group(2));
      // Getting all possible addresses changed by bitmask
      for (int address in bitmaskedMemoryAddress(currentMask, memoryPosition)){
        memory[address] = numberToStore;
      };
    }
  }
  return memory.values.reduce((value, element) => value + element);
}


void day14() async{
  print('Boy, I just wanted a simple one at least for today');
  // Part one
  int initializing = await initializeFerryMask();
  print('Initialized sum : $initializing');
  // Part two
  int version2 = await initializeFerryMaskVersion2();
  print('Memory sum for version 2: $version2');
}