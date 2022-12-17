class Monkey:
  def __init__(self, id:int, items: list, operation: list, test: int, trueID:int, falseID:int):
    self.id = id
    self.items = items
    self.operation = operation
    self.test = test
    self.trueID = trueID
    self.falseID = falseID
    self.inspectCount=0
  def turn(self, monkeylist:list, partOne:bool, bigTest:int):
    for item in self.items:
      item = self.inspect(item)
      if partOne:
        item= item//3
      else:
        item=item%bigTest
      if (item%self.test)==0:
        monkeyList[self.trueID].items.append(item)
      else:
        monkeyList[self.falseID].items.append(item)
    self.items = []
  def inspect(self, item):
    numop = True
    if self.operation[1] == "old":
      numop = False
    match self.operation[0]:
      case "+":
        if numop:
          item+=int(self.operation[1])
        else:
          item+=item
      case "-":
        if numop:
          item-=int(self.operation[1])
        else:
          item-=item
      case "*":
        if numop:
          item*=int(self.operation[1])
        else:
          item*=item
      case "/":
        if numop:
          item/=int(self.operation[1])
        else:
          item/=item
    self.inspectCount+=1
    return item

def monkeyInput():
  inputfile = open("/home/kat/AOC2022/day11/input", "r")
  monkeyList = []
  bigTest = 1
  while True:
    line = inputfile.readline()
    if line == '':
      break
    elif line == '\n':
      continue
    line = line.split()
    monkeyNumber = line[1][0]
    line = inputfile.readline().strip().replace(',', ' ').split()
    items = []
    for item in line:
      if item.isdigit():
        items.append(int(item))
    line = inputfile.readline().strip().split()
    operation = [line[4], line[5]]
    test = int(inputfile.readline().strip().split()[3])
    bigTest = test * bigTest
    trueID = int(inputfile.readline().strip().split()[5])
    falseID = int(inputfile.readline().strip().split()[5])
    monkey = Monkey(monkeyNumber, items, operation, test, trueID, falseID)
    monkeyList.append(monkey)
  return monkeyList
monkeyList = monkeyInput()
bigTest = 1
for monkey in monkeyList:
  bigTest = bigTest*monkey.test
for round in range(0,20):
  for monkey in monkeyList:
    monkey.turn(monkeyList, True, 0)
monkey1=0
monkey2=0
for monkey in monkeyList:
  if monkey.inspectCount >= monkey1:
    monkey2=monkey1
    monkey1=monkey.inspectCount
  elif monkey.inspectCount >= monkey2:
    monkey2=monkey.inspectCount

print("Monkey Business: "+str(monkey1*monkey2))

monkeyList=monkeyInput()
bigTest = 1
for monkey in monkeyList:
  bigTest = bigTest*monkey.test
for round in range(0,10000):
  for monkey in monkeyList:
    monkey.turn(monkeyList, False, bigTest)
monkey1=0
monkey2=0
for monkey in monkeyList:
  if monkey.inspectCount >= monkey1:
    monkey2=monkey1
    monkey1=monkey.inspectCount
  elif monkey.inspectCount >= monkey2:
    monkey2=monkey.inspectCount

print("Monkey Business BAD: "+str(monkey1*monkey2))