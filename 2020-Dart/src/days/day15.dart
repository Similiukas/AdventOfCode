import 'dart:io';

// Part one and two
Future<int> memoryGameNumber(int range) async{
  /* Not the fastest and takes a couple seconds but hey, I managed to do it and that's fun */
  var file = File('input/day15/input.txt');
  List<int> input = (await file.readAsString()).split(',').map(int.parse).toList();
  var mostRecent = Map.fromIterable(input,
                      key: (item) => item,
                      value: (item) => input.indexOf(item));
  // Would be prettier but need to invoke it for some reason (print var f or f.toList()) because otherwise mostRecent is empty
  // var f = input.map((item) => mostRecent[item] = input.indexOf(item));
  // var k = input.asMap();           // Simple and elegant but puts keys as values and make a map unmodifiable
  int last = input[input.length - 1]; // Last number said
  for (int i = input.length - 1; i < range - 1; ++i){
    if (!mostRecent.containsKey(last)){ // If number hasn't been said
      mostRecent[last] = i;
      last = 0;
    }
    else{
      int temp = i - mostRecent[last];  // New number said
      mostRecent[last] = i;             // Storing when the last number was said
      last = temp;
    }
  }
  return last;
}

void day15() async{
  print('Oh boy, the server works are starting');
  // Part one
  int simpleOne = await memoryGameNumber(2020);
  print('Simple here: $simpleOne');
  // Part two
  int harderOne = await memoryGameNumber(30000000);
  print('Okay, now it\'s hard: $harderOne');
}