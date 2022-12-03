elfFile = open("/home/kat/AOC/day1/input", "r")
elves = []
total = 0

for line in elfFile.readlines():
  if line.strip() == "":
    elves.append(total)
    total=0
  else:
    total=total+int(line)

elves.sort(reverse=True)
max = elves[0] + elves[1] + elves[2]
print(max)