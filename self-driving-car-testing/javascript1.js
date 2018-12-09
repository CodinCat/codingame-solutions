const N = parseInt(readline());
const xthenCOMMANDS = readline();
const [carPosition, ...rawCommands] = xthenCOMMANDS.split(';');
let carIndex = parseInt(carPosition, 10) - 1;
const commands = rawCommands.reduce((acc, rawCmd) => {
  const repeat = parseInt(rawCmd.slice(0, -1), 10);
  const cmd = rawCmd.slice(-1);
  for (let i = 0; i < repeat; i++) {
    acc.push(cmd);
  }
  return acc;
}, []);

let road = [];
for (let i = 0; i < N; i++) {
  const [repeat, roadStr] = readline().split(';');
  for (let j = 0; j < repeat; j++) {
    road.push(roadStr);
  }
}
const ans = road.map((row, i) => {
  if (commands[i] === 'R') carIndex++;
  else if (commands[i] === 'L') carIndex--;
  const part1 = row.slice(0, carIndex);
  const part2 = row.slice(carIndex + 1);
  return part1 + '#' + part2;
});
print(ans.join('\n'));
