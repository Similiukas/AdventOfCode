import 'dart:io';
import 'dart:convert';

Future<int> findPasswordsPart1() async {
  var file = new File('input/day2/input.txt');
  Stream<List<int>> inputStream = file.openRead();

  var correctPasswords = 0;
  // Reading line by line
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    List<String> contents = line.split(new RegExp('[-: ]'));  // 3rd item is just a whitespace
    int min = int.parse(contents[0]);
    int max = int.parse(contents[1]);
    // How the hell do you do regex? :/
    // RegExp reg = new RegExp('${contents[2]}{$min,$max}');
    // if (reg.hasMatch(contents[4])){
    //   print('$min, $max, char: [${contents[2]}], [${contents[4]}]');
    //   correctPasswords++;
    // }
    // Yay, found some regex finally
    int correctCharSize = contents[4].replaceAll(new RegExp('[^${contents[2]}]'), '').length;
    // contents[4].runes.forEach((rune) => {
    //   if (String.fromCharCode(rune) == contents[2]){
    //     correctCharSize++
    //   }
    // });
    if (correctCharSize >= min && correctCharSize <= max) {
      correctPasswords++;
    }
  }
  return correctPasswords;
}

Future<int> findPasswordsPart2() async {
  var file = new File('input/day2/input.txt');
  Stream<List<int>> inputStream = file.openRead();

  var correctPasswords = 0;

  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    List<String> contents = line.split(new RegExp('[-: ]'));
    int min = int.parse(contents[0]);
    int max = int.parse(contents[1]);
    // Sad, no XOR operator in dart but hey, that works
    if ((contents[4][min - 1] == contents[2]) != (contents[4][max - 1] == contents[2])) {
      correctPasswords++;
    }
  }
  return correctPasswords;
}

void day2() async {
  print('Well I\'m a bit late');
  int found = await findPasswordsPart2();
  print('Found $found passwords which match the corporate policy');
}