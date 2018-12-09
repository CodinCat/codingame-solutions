const r1 = parseInt(readline());
const r2 = parseInt(readline());

const r = [r1, r2];

while (r[0] !== r[1]) {
  r.sort((a, b) => a - b);
  r[0] = `${r[0]}`.split('').reduce((sum, n) => sum + parseInt(n, 10), r[0]);
}

print(r[0]);
