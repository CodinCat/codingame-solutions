const inputs = readline().split(' ');
const W = parseInt(inputs[0]);
const H = parseInt(inputs[1]);

const lines = [];
for (let i = 0; i < H; i++) {
  lines.push(readline());
}

const orderedKeys = [];
const result = {};
for (let i = 0; i < W; i += 3) {
  const key = lines[0][i];
  const destIndex = walk(i);
  result[key] = lines[H - 1][destIndex];
  orderedKeys.push(key);
}

orderedKeys.forEach(key => {
  print(`${key}${result[key]}`);
});

function walk(startIndex) {
  let col = startIndex;
  for (let i = 1; i < H - 1; i++) {
    const line = lines[i];
    if (line[col + 1] === '-') {
      col += 3;
    } else if (line[col - 1] === '-') {
      col -= 3;
    }
  }
  return col;
}