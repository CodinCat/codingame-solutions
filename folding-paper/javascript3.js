const order = readline();
const side = readline();

const opps = {
  R: 'L',
  L: 'R',
  U: 'D',
  D: 'U',
}
const result = {
  R: 1,
  L: 1,
  U: 1,
  D: 1,
}
for (const c of order) {
  const opp = opps[c]
  const match = c === side || opp === side
  if (match) {
    result[opp] += result[c]
    result[c] = 1
  } else {
    result[side] *= 2
    result[opps[side]] *= 2
  }
}
print(result[side])
