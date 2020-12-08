import '../helpers.dart';

// Part one
int findParents(String bag, int found, List<String> outerBags, List<String> file){
  /* Going through bags parents and checking how many parents it has recursively */
  for (var line in file){
    if (line.contains('$bag bags contain') && bag != 'shiny gold'){ // If the bag is a parent, increment answer (except shiny gold bag itself)
      found++;
    }
    else if (line.contains('$bag bag')) {   // If bag is a children
      String outerBag = line.split(' ')[0] + ' ' + line.split(' ')[1];  // Getting parent
      if (!outerBags.contains(outerBag)){   // If parent hasn't been counted yet, checking all the bags when it's a child
        outerBags.add(outerBag);
        found = findParents(outerBag, found, outerBags, file);
      }
    }
  }
  return found;
}
// Part two
int findChildren(String bag, int found, List<String> file){
  /* Going through bags and if the bag is outer layer (parent) then multiplying the amount of children by a multiple.
     Recursively going through children and doing the same thing */
  for (var line in file){
    if (line.contains('$bag bags contain')){  // If bag is the outer layer
      var splitLine = line.split(' ');
      if (splitLine[4] != 'no'){     // If bag doesn't have children then returns 1 and doesn't look for children
        for (var i = 4; i < splitLine.length; i+=4){  // Going every 4th item since bag has two colour names a word 'bag/s' and a multiple
          found += int.parse(splitLine[i]) * findChildren((splitLine[i + 1] + ' ' + splitLine[i + 2]), 1, file);
        }
      }
    }
  }
  return found;
}

Future<void> findBags() async{
  int found = 0;
  List<String> file = await readFileString('input/day7/input.txt');
  // Part one
  found = findParents('shiny gold', found, <String>[], file);
  print('Found shiny bags parents: $found');
  // Part two
  found = 1;
  found = findChildren('shiny gold', found, file);
  print('Found shiny bags children: ${--found}');   // Decreasing by 1 since the shiny gold bag doesn't count
}


void day7(){
  print('Just by reading the instructions it\'s yikes');
  // Part one and two
  findBags();
}