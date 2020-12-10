import 'dart:math';

import '../helpers.dart';

// Part one
Future<int> findChainDifferences() async{
  List<int> input = await readFileInt('input/day10/input.txt');
  input.add(0);
  input.sort();
  int difference1 = 0, difference3 = 0;
  for (int i = 0; i < input.length - 1; ++i){
    if (input[i + 1] - input[i] == 1){
      difference1++;
    }
    else{
      difference3++;
    }
  }
  difference3++;
  return difference1 * difference3;
}

// Part two (inspired by reddit this time, was thinking of something similar myself but coulnd't do it)
Future<int> findChainPaths() async{
  List<int> chain = await readFileInt('input/day10/input.txt');
  chain.add(0);
  chain.sort();
  var numberOfPaths = [1];  // Stores ways of how to get to the nth element in nth place
  for (int i = 1; i < chain.length; ++i){
    int paths = 0;          // How many ways there are to get to the nth element
    for (int j = 0; j < i; ++j){
      if (chain[i] - chain[j] <= 3){  // If it's possible to get from chain[i] to chain[j] 
        // If you can get form chain[i] to chain[j], then to get to chain[i] you can get in chain[j] ways
        // 0, 1, 2, 3, 4...
        // You can get to 1 chain[i] only from 0 (1 way numberOfPaths[j])
        // You can get to 2 chain[i] from 0 chain[j] (1 way numberOfPaths[j]), you can get to 2 chain[i] from 1 chain[j] (1 way numberOfPaths[j]) (2 ways in total)
        // You can get to 3 chain[i] from 0 chain[j] (1 way numberOfPaths[j]), you can get to 3 chain[i] from 1 chain[j] (1 way numberOfPaths[j]), you can get to 3 chain[i] from 2 chain[j] (2 ways numberOfPaths[j]) (4 in total)
        // You can get to 4 from 1 (1 way), you can get to 4 from 2 (2 way), you can get to 4 from 3 (4 ways) (7 in total)
        // ...
        paths += numberOfPaths[j];
      }
    }
    numberOfPaths.add(paths); // Adding how many ways there are to get to the nth element
  }
  return numberOfPaths.last;
}

void day10() async{
  print('Is there more hope?');
  // Part one
  int chainJolts = await findChainDifferences();
  print('Chain answer: $chainJolts');
  // Part two
  int numberOfPaths = await findChainPaths();
  print('Thanks reddit, you are indeed smart: $numberOfPaths');
}