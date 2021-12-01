import 'dart:io';
import 'dart:convert';

Future<List<String>> readMap() async{
  var file = new File('input/day3/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  List<String> map = [];
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    map.add(line);
  }
  return map;
}

int treesBam(List<String> map, int slopeX, int slopeY){
  int trees = 0;
  int pointerX = 0;
  for (var pointerY = 0; pointerY < map.length; pointerY += slopeY) {
    if (map[pointerY][pointerX] == "#") {
      trees++;
    }
    pointerX = (pointerX + slopeX) % map[pointerY].length;
  }
  return trees;
}

void day3() async {
  print('Boy is this taking me so long');
  var map = await readMap();
  print(treesBam(map, 3, 1) * treesBam(map, 1, 1) * treesBam(map, 5, 1) * 
  treesBam(map, 7, 1) * treesBam(map, 1, 2));
}