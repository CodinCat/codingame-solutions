const order = readline();
const side = readline();

const result = {
  R: 1,
  L: 1,
  U: 1,
  D: 1,
};

const handle = {
  R() {
    result.L = result.L + result.R;
    result.R = 1;
    result.U = result.U * 2;
    result.D = result.D * 2;
  },
  L() {
    result.R = result.L + result.R;
    result.L = 1;
    result.U = result.U * 2;
    result.D = result.D * 2;
  },
  U() {
    result.D = result.U + result.D;
    result.U = 1;
    result.L = result.L * 2;
    result.R = result.R * 2;
  },
  D() {
    result.U = result.U + result.D;
    result.D = 1;
    result.L = result.L * 2;
    result.R = result.R * 2;
  },
};

for (let c of order) {
  handle[c]();
}

print(result[side].toString());
