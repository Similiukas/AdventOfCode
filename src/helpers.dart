import 'dart:io';
import 'dart:convert';
import 'dart:collection';

/* Reads file and returns List of integers from it */
Future<List<int>> readFileInt(String path) async {
  var file = new File(path);
  return (await file.readAsLines()).map(int.parse).toList();
}
Future<List<int>> readFileIntLines(String path, int numberOfLines) async {
  var file = new File(path);
  int line = 0;
  List<int> input = new List(numberOfLines);
  await for (var l in file.openRead().transform(utf8.decoder).transform(new LineSplitter()).asyncMap(int.parse)){
    if (line == numberOfLines) break;
    input[line] = l;
    line++;
  }
  return input;
}
/* Reads file and returns List of Strings from it */
Future<List<String>> readFileString(String path) async{
  var file = new File(path);
  return await file.readAsLines();
}

/* Simpliest stack you've ever */
class Stack<T>{
  final _stack = Queue<T>();

  bool get isNotEmpty => _stack.isNotEmpty;

  void push(T element) {
    _stack.addLast(element);
  }

  T pop() {
    T lastElement = _stack.last;
    _stack.removeLast();
    return lastElement;
  }

  T top() {
    return _stack.last;
  }
}