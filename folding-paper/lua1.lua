order = io.read()
side = io.read()

paper = {
  ['R'] = 1,
  ['L'] = 1,
  ['U'] = 1,
  ['D'] = 1,
}

handle = {
  ['R'] = function ()
    paper.L = paper.L + paper.R
    paper.R = 1
    paper.U = paper.U * 2
    paper.D = paper.D * 2
  end,
  ['L'] = function ()
    paper.R = paper.R + paper.L
    paper.L = 1
    paper.U = paper.U * 2
    paper.D = paper.D * 2
  end,
  ['U'] = function ()
    paper.D = paper.D + paper.U
    paper.U = 1
    paper.R = paper.R * 2
    paper.L = paper.L * 2
  end,
  ['D'] = function ()
    paper.U = paper.U + paper.D
    paper.D = 1
    paper.R = paper.R * 2
    paper.L = paper.L * 2
  end,
}

for i = 1, #order do
  local c = string.sub(order, i, i)
  handle[c]()
end

print(paper[side])
