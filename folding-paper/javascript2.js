const order = readline();
const side = readline();

const patterns = {
  R: ['R', 'L', 'U', 'D'],
  L: ['L', 'R', 'U', 'D'],
  U: ['U', 'D', 'R', 'L'],
  D: ['D', 'U', 'R', 'L'],
}

const result = {
  R: 1,
  L: 1,
  U: 1,
  D: 1,
}

for (const char of order) {
  const [a, b, c, d] = patterns[char]
  result[b] += result[a]
  result[a] = 1
  result[c] *= 2
  result[d] *= 2
}
print(result[side]);
