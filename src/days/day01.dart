import 'dart:async';
import 'dart:io';

Future<List<int>> readFile() async {
  var file = new File('input/day1/input.txt');
  // Can do it in one line if don't need to line by line
  return (await file.readAsLines()).map(int.parse).toList();
  // Doing line by line (Need to add "import 'dart:convert'")
  /* Stream<List<int>> inputStream = file.openRead();
  var integers = <int>[];
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    integers.add(int.parse(line));
  }
  return integers; */
}

void part1(List data){
  for (var i = 0; i < data.length - 1; i++) {
    var j = i + 1;
    while (data[i] + data[j] < 2020) {
      j++;
    }
    if (data[i] + data[j] == 2020) {
      print('Found it! ${data[i]} + ${data[j]} = ${data[i] + data[j]}\nAnswer: ${data[i] * data[j]}');
      break;
    }
  }
}
// That's just ugly but spent way too much time on this
void part2(List data){
  for (var i = 0; data[i] + data[i + 1] + data[i + 2] < 2020; i++) {
    var j = i + 1;
    var m = j + 1;
    while(data[i] + data[j] + data[j + 1] < 2020) {
      m = j + 1;
      while (data[i] + data[j] + data[m] < 2020) {
        m++;
      }
      if (data[i] + data[j] + data[m] == 2020) { break; }
      j++;
    }
    if (data[i] + data[j] + data[m] == 2020) {
      print('Found it! ${data[i]} + ${data[j]} + ${data[m]} = ${data[i] + data[j] + data[m]}\nAnswer: ${data[i] * data[j] * data[m]}');
      break;
    }
  }
}

void day1() async {
  print('Starting day 1');
  var data = await readFile();
  data.sort();
  print('File: $data');
  part2(data);
}