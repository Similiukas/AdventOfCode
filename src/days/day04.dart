import 'dart:io';
import 'dart:convert';

Future<int> checkPassports() async{
  var file = new File('input/day4/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  // List<String> requiredFields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']; For part one
  var numberOfItems = 0, validPassports = 0;
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    if (line.isNotEmpty){
      var items = line.split(new RegExp(r'[: ]'));
      for (var i = 0; i < items.length; i+=2) {
        switch (items[i]) {
          case 'byr':
            numberOfItems += numberBounds(items[i + 1], 1920, 2002) ? 1 : 0;
            break;
          case 'iyr':
            numberOfItems += numberBounds(items[i + 1], 2010, 2020) ? 1 : 0;
            break;
          case 'eyr':
            numberOfItems += numberBounds(items[i + 1], 2020, 2030) ? 1 : 0;
            break;
          case 'hgt':
            if ((items[i + 1].endsWith('in') && numberBounds(items[i + 1].substring(0, 2), 59, 76)) ||
                (items[i + 1].endsWith('cm') && numberBounds(items[i + 1].substring(0, 3), 150, 193))) {
              numberOfItems++;
            }
            break;
          case 'hcl':
            if (items[i + 1].contains(new RegExp('^#[a-f0-9]{6}\$'))) { numberOfItems++; }
            break;
          case 'ecl':
            if (['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].contains(items[i + 1])) { numberOfItems++; }
            break;
          case 'pid':
            if (items[i + 1].contains(new RegExp(r'^\d{9}$'))) { numberOfItems++; }
            break;
        }
      }
    }
    else{
      if (numberOfItems == 7) {
        validPassports++;
      }
      numberOfItems = 0;
    }
  }
  if (numberOfItems == 7) {
    validPassports++;
  }
  return validPassports;
}

bool numberBounds(String item, int lowerBound, int upperBound){
  int number = int.tryParse(item);
  if (number == null) return false;
  return number >= lowerBound && number <= upperBound;
}

void day4() async {
  print("Starting earlier today");
  int answer = await checkPassports();
  print('Total valid passports: $answer');
}