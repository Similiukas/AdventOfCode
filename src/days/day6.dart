import 'dart:io';
import 'dart:convert';

// Part one
Future<int> countGroupsSum() async{
  var file = new File('input/day6/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  int sum = 0;
  Set<String> letters = {};
  await for(var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    if (line.isNotEmpty) {
      line.runes.forEach((rune) {
        letters.add(String.fromCharCode(rune));
      });
    }
    else{
      sum += letters.length;
      letters.clear();
    }
  }
  sum += letters.length;
  return sum;
}

Future<int> countGroupsTogether() async{
  var file = new File('input/day6/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  int sum = 0, person = 0;
  Map<String, int> letters = {};
  await for(var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    if (line.isNotEmpty) {
      person += 1;
      line.runes.forEach((rune) {
        var char = String.fromCharCode(rune);
        letters[char] = (letters[char] == null) ? 1 : letters[char] + 1;  // If that letter doesn't exist, increment by 1
      });
    }
    else{
      letters.values.forEach((element) {
        if (element == person) sum += 1;
      });
      letters.clear();
      person = 0;
    }
  }
  letters.values.forEach((element) { if (element == person) sum += 1; });
  return sum;
}

void day6() async{
  print('Can I get up even earlier?');
  // Part one
  int groupSum = await countGroupsSum();
  print('Groups answer sum: $groupSum');
  // Part two
  int groupTogether = await countGroupsTogether();
  print('Groups together: $groupTogether');
}