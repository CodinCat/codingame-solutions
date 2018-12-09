const expression = readline();

const brackets = '{}[]()';
const isLeft = char => '{[('.includes(char);
const isPair = (l, r) =>
  (l === '{' && r === '}') ||
  (l === '[' && r === ']') ||
  (l === '(' && r === ')');

print(
  (function () {
    const left = [];
    for (let x of expression) {
      if (!brackets.includes(x)) continue;
      if (isLeft(x)) {
        left.push(x);
      } else {
        if (!isPair(left.pop(), x)) {
          return false;
        }
      }
    }
    return left.length === 0;
  })()
);
