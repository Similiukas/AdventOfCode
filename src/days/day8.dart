import '../helpers.dart';

var jmpAndNop = <int>{};    // For part two and one
bool fixedFile = false;     // For part two
// Part one
int accBeforeLoop(List<String> file){
  int acc = 0, pointer = 0;
  jmpAndNop.clear();
  // Looping until that instruction hasn't been done yet
  while (!jmpAndNop.contains(pointer) && pointer < file.length - 1){
    jmpAndNop.add(pointer);   // Adding instruction to set
    List<String> instruction = file[pointer].split(' ');
    switch (instruction[0]){
      case 'acc':
        acc += int.parse(instruction[1]);
        pointer++;
        break;
      case 'jmp':
        pointer += int.parse(instruction[1]);
        break;
      case 'nop':
        pointer++;
        break;
    }
  }
  // If loop ended when pointer reached the end, the file is not corrupted
  if (pointer == file.length - 1){
    fixedFile = true;
    return acc;
  }
  return acc;                                   // If file is corrupted (it loops)
}

// Part two
int findDefected(List<String> file){
  var possiblyDefected = [...jmpAndNop];        // All instructions from Part One
  for (int i in possiblyDefected){              // Looping through every single one and trying to fix either 'nop' or 'jmp'
    if (file[i].contains('acc')) { continue; }  // 'acc' instructions are not corrupted
    List<String> fileTemp = [...file];          // Creates a copy of file
    fileTemp[i] = (fileTemp[i][0] == 'n') ? fileTemp[i].replaceAll('nop', 'jmp') : fileTemp[i].replaceAll('jmp', 'nop');
    int acc = accBeforeLoop(fileTemp);
    if (fixedFile){ return acc; }
  }
  return -1;
}

Future<void> findLoop() async {
  List<String> file = await readFileString('input/day8/input.txt');
  // Part one
  int acc = accBeforeLoop(file);
  print('Accumulator before loop: $acc');
  // Part two
  acc = findDefected(file);
  print('When file is not corrupted acc: $acc');
}

void day8(){
  print('Tutorial meetings are slowing me down :/');  // I swear there's sometimes 2 new lines and sometimes just one. Are there any ghosts helping me?
  findLoop();
}