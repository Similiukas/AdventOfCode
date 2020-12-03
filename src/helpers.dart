import 'dart:io';

/* Reads file and returns List of integers from it */
Future<List<int>> readFileInt(String path) async {
  var file = new File(path);
  return (await file.readAsLines()).map(int.parse).toList();
}