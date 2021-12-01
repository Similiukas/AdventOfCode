import 'dart:io';
import 'dart:convert';

import '../helpers.dart';

// Part two
num presedence(var element) { return (element == '+') ? 3 : 2; }
// Part one and two
num evaluate(num a, num b, var op){
  switch (op) {
    case '+':
      return a + b;
    case '-':
      return a - b;
    case '*':
      return a * b;
    default:
      return a / b;
  }
}
// Part one and two
void calculateStack(Stack operatorStack, Stack outputStack){
  num numberB = num.parse(outputStack.pop());
  num numberA = num.parse(outputStack.pop());
  outputStack.push(evaluate(numberA, numberB, operatorStack.pop()).toString());
}
// Part one and two
num shuntingYard(List<String> expression, {bool partTwo = false}){
  var outputStack = Stack();
  var operatorStack = Stack();
  for (var token in expression){
    if (int.tryParse(token) != null)  outputStack.push(token);
    else if (['+','-','*','/'].contains(token)){
      if (partTwo){ // For part two just adding presedence checking
        while (operatorStack.isNotEmpty && operatorStack.top() != '(' && presedence(operatorStack.top()) >= presedence(token)){
          calculateStack(operatorStack, outputStack);
        }
      }
      else{
        while (operatorStack.isNotEmpty && operatorStack.top() != '('){
          calculateStack(operatorStack, outputStack);
        }
      }
      operatorStack.push(token);
    }
    else if (token == '(')  operatorStack.push('(');
    else if (token == ')'){ // Evaluating the brackets
      while (operatorStack.isNotEmpty && operatorStack.top() != '('){
        calculateStack(operatorStack, outputStack);
      }
      operatorStack.pop();
    }
  }
  while (operatorStack.isNotEmpty){           // After main loop finishing off remaining expression
    calculateStack(operatorStack, outputStack);
  }
  return num.parse(outputStack.pop());
}
// Part one and two
Future<num> mathIsMath(bool partTwo) async{
  var file = File('input/day18/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  num sum = 0;
  await for (var line in inputStream.transform(utf8.decoder).transform(LineSplitter())){
    var expression = line.replaceAll('(', '( ').replaceAll(')', ' )').split(' ');
    sum += shuntingYard(expression, partTwo: partTwo);
  }
  return sum;
}

void day18() async{
  print('The end is finally here, ah, feels good');
  num shuntingGoesBrrr = await mathIsMath(false);
  print('When you don\'t need to check the input it\'s so simple $shuntingGoesBrrr');
  num mathIsCool = await mathIsMath(true);
  print('Thanks assignment 2, that was really fast $mathIsCool');
}