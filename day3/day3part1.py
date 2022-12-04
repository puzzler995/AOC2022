sacks = open("/home/kat/AOC2022/day3/input", "r")
priorities = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"]
prioritySum = 0
for sack in sacks.readlines():
  sack = sack.replace("\n", "")
  splitp = ((len(sack)-1)//2)+1
  cmpt1 = sack[0:splitp]
  cmpt2 = sack[splitp:len(sack)]
  commonLetter = ''.join(set(cmpt1).intersection(cmpt2))
  prioritySum += priorities.index(commonLetter)+1
print(prioritySum)