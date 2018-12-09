import 'dart:io';
import 'dart:math';

void main() {
  String order = stdin.readLineSync();
  String side = stdin.readLineSync();
  var paper = {
    'R': 1,
    'L': 1,
    'U': 1,
    'D': 1,
  };

  order.runes.forEach((int rune) {
    var c = new String.fromCharCode(rune);
    if (c == 'R') {
      paper['L'] += paper['R'];
      paper['R'] = 1;
      paper['U'] *= 2;
      paper['D'] *= 2;
    } else if (c == 'L') {
      paper['R'] += paper['L'];
      paper['L'] = 1;
      paper['U'] *= 2;
      paper['D'] *= 2;
    } else if (c == 'U') {
      paper['D'] += paper['U'];
      paper['U'] = 1;
      paper['R'] *= 2;
      paper['L'] *= 2;
    } else {
      paper['U'] += paper['D'];
      paper['D'] = 1;
      paper['R'] *= 2;
      paper['L'] *= 2;
    }
  });

  print(paper[side]);
}
