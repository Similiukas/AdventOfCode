import 'dart:io';
import 'dart:convert';

// Part one and two
void goGoShip(String direction, int distance, List<int> coordinates){
  // Don't need to return since List<int> are passed by reference
  // Key point is that coordinates[0] being x axis and coordinates[y] being y axis
  // Then North is y > 0 and South is y < 0. Same for x axis
  if      (direction == 'E')  coordinates[0] += distance;   // Going East
  else if (direction == 'W')  coordinates[0] -= distance;   // Going West
  else if (direction == "N")  coordinates[1] += distance;   // Going North
  else                        coordinates[1] -= distance;   // Going South
}

// Part one
String turnShip(String currentDirection, String turn, int degrees){
  var directions = ['E', 'S', 'W', 'N'];
  if (turn == 'R'){ // Figuring out how many times need to turn (degrees/90) and then turning from current direction
    return directions[(directions.indexOf(currentDirection) +  degrees~/90) % 4];
  }
  return directions[(directions.indexOf(currentDirection) + (4 - degrees~/90)) % 4];
}

// Part one
Future<int> shipInstructions() async{
  var file = File('input/day12/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  String currentDirection = 'E';  // Current ship's direction
  List<int> position = [0, 0];
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    int distance = int.parse(line.substring(1));
    switch (line.substring(0, 1)) {
      case 'F':
        // Move the ship in currenr direction by 'distance'
        goGoShip(currentDirection, distance, position);
        break;
      case 'N':
        // Move the ship in NORTH by 'distance'
        goGoShip('N', distance, position);
        break;
      case 'E':
        goGoShip('E', distance, position);
        break;
      case 'S':
        // Move the ship in SOUTH by 'distance'
        goGoShip('S', distance, position);
        break;
      case 'W':
        goGoShip('W', distance, position);
        break;
      case 'L':
        // Turn the ship left by 'distance'/90
        currentDirection = turnShip(currentDirection, 'L', distance);
        break;
      case 'R':
        // Turn the ship RIGHT by 'distance'/90
        currentDirection = turnShip(currentDirection, 'R', distance);
        break;
    }
  }
  return position[0].abs() + position[1].abs();
}

// Part two
Future<int> waypointInstructions() async{
  var file = File('input/day12/input.txt');
  Stream<List<int>> inputStream = file.openRead();
  List<int> shipPosition = [0, 0], waypointPosition = [10, 1];
  await for (var line in inputStream.transform(utf8.decoder).transform(new LineSplitter())){
    int distance = int.parse(line.substring(1));
    switch (line.substring(0, 1)) {
      case 'F':
        // Move the ship to the waypoint 'distance' times
        goGoShip('E', distance * waypointPosition[0], shipPosition);  // Moving the ship east/north or west/south
        goGoShip('N', distance * waypointPosition[1], shipPosition);  // If waypoint is negative, then moving west/south
        break;
      case 'N':
        goGoShip('N', distance, waypointPosition);
        break;
      case 'E':
        goGoShip('E', distance, waypointPosition);
        break;
      case 'S':
        goGoShip('S', distance, waypointPosition);
        break;
      case 'W':
        goGoShip('W', distance, waypointPosition);
        break;
      case 'L':
        // Turn the waypoint left (counter clockwise) degrees/90 times
        for (int i = 0; i < distance ~/90; ++i){      // Waypoint (x0; y0) becomes (-y0; x0)
          int temp = waypointPosition[0];
          waypointPosition[0] = - waypointPosition[1];
          waypointPosition[1] = temp;
        }
        break;
      case 'R':
        // Turn the waypoint right (clockwise) degrees/90 times
        for (int i = 0; i < distance ~/90; ++i){      // Waypoint (x0; y0) becomes (-y0; x0)
          int temp = waypointPosition[0];
          waypointPosition[0] = waypointPosition[1];
          waypointPosition[1] = - temp;
        }
        break;
    }
  }
  return shipPosition[0].abs() + shipPosition[1].abs();
}

void day12() async{
  print('And here begins the story of being one');
  // Part one
  int shipIsAt = await shipInstructions();
  print('Ship is at: $shipIsAt');
  // Part two
  int withWaypoints = await waypointInstructions();
  print('Ship\'s Manhattan distance with waypoints is: $withWaypoints');
}