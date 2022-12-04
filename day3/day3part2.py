#sacks = open("/home/kat/AOC2022/day3/input", "r")
priorities = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"]
prioritySum = 0
with open("/home/kat/AOC2022/day3/input", "r") as sacks:
  line = 0
  allSacks = sacks.readlines()
  print(len(allSacks))
  while line < len(allSacks):
    badge = ''.join(set(allSacks[line]).intersection(allSacks[line+1]))
    badge = ''.join(set(badge).intersection(allSacks[line+2]))
    badge = badge.replace("\n", "")
    prioritySum += priorities.index(badge)+1
    line = line + 3
print(prioritySum)