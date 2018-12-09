const r1 = parseInt(readline());
const cache = new Map();

function check(i) {
  let head = i;
  while (head < r1) {
    if (cache.has(head)) {
      return false;
    }
    const next = `${head}`
      .split('')
      .reduce((sum, n) => sum + parseInt(n, 10), head);
    cache.set(head, next);
    head = next;
  }
  return head === r1;
}

function run() {
  for (let i = 1; i < r1; i++) {
    if (check(i)) {
      return true;
    }
  }
  return false;
}

print(run() ? 'YES' : 'NO');
