import '../helpers.dart';

// Part one
Future<int> searchEarliest() async{
  var file = await readFileString('input/day13/input.txt');
  int arrival = int.parse(file[0]);
  int smallest = arrival, id;
  for (var symbol in file[1].split(',')){
    if (symbol == 'x')  continue;
    int time = int.parse(symbol);
    if (time - arrival % time < smallest){
      smallest = time - arrival % time;
      id = time;
    }
  }
  return id * smallest;
}

// Part two (again, defeated :/)
Future<int> searchSynchronize() async{
  var input = ((await readFileString('input/day13/input.txt'))[1]).split(',');
  /* Brute force:
  int multiple = 0, firstOne = int.parse(input[0]);
  bool found = false;
  while (!found){
    multiple += firstOne;
    found = true;
    for (int i = 1; i < input.length; ++i){
      if (input[i] != 'x' && (multiple + i) % int.parse(input[i]) != 0){
        found = false; 
        break;}}} 

  Slightly faster brute force (by adding largest number):
  int largest = 0, m = 0;
  input.forEach((element) { largest = max((int.tryParse(element) ?? 0), largest);});
  while (!found){
    multiple = (largest * (m + 1)) - input.indexOf(largest.toString());   // do while loop here would be better
    while ((multiple + input.indexOf(largest.toString())) % largest != 0 || multiple % int.parse(input[0]) != 0){   // Can be a do while loop maybe?
      m++;
      multiple = (largest * m) - input.indexOf(largest.toString());
    }
    found = true;
    for (int i = 1; i < input.length; ++i){
      if (input[i] != 'x' && (multiple + i) % int.parse(input[i]) != 0){
        found = false;
        break;
  }}}*/

  var inputInt = input.map((e) => int.tryParse(e) ?? 1).toList(); // Replacing strings as integers and 'x' as 1
  // Going through bus times and need to find a such time that (time + index) % input[index] == 0

  // need (time + index) to be divisible by input[index]  ->  (time + index) == 0 mod input[index]
  // Iteration 0: (time) == 0 mod input[0]  ->  time = input[0]
  // Iteration 1: (time + index) == 0 mod input[1]  ->  (input[0] + 1) == 0 mod input[1]
  // So if it's not divisible by input[1] need to add multiples of time until it is  ->
  // time = time + time  ->  time = time + input[0]  until (time + 1) == 0 mod input[1]
  // Then product = timeBefore * input[1]  -> product = product * input[1]

  // Iteration 2: (time + 2) == 0 mod input[2]
  // Again need to find such time that (time + 2) is divisible by input[2]
  // So need to something to time such that  (time + 2) % input[2] == 0  and also previous equations hold:  (time + 1) % input[1]== 0  and time % input[0] == 0
  // Thus time = time + product, where product = input[0] * input[1]
  // This way adding to time multiples of input[0] and multiples of input[1]
  // This way previous equations hold and we find such time that (time + 2) % input[2] == 0
  // After finding such time, saving the new product as product = input[0] * input[1] * input[2]  ->  product = product * input[2]
  int product = inputInt[0], time = 0;
  for (int busIndex = 1; busIndex < inputInt.length; ++busIndex){
    int busTime = inputInt[busIndex];
    while ((time + busIndex) % busTime != 0){ // Looking through times until it holds the equation
      time += product;                      // Incrementing time by multiples of previous times so previous equations hold
    }
    product *= busTime;                     // Saving input[index]
  }
  return time;
}

void day13() async{
  print('Input seems suspiciously small');
  // Part one
  int earliest = await searchEarliest();
  print('Earles bus code: $earliest');
  // Part two
  int veryLong = await searchSynchronize();
  print('Here, take it $veryLong');
}