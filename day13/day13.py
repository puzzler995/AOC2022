packets = []
def parseArray(currentstr:list):
  packet = []
  while True:
    try:
      character = currentstr.pop(0)
    except:
      return packet;
    if character == "[":
      quop = parseArray(currentstr)
      packet.append(quop[1])
      currentstr = quop[0]
    elif character == "]":
      quop = (currentstr, packet)
      return quop;
    elif character ==",":
      continue
    else:
      packet.append(character)
def compareLists(left:list, right:list, topLevel:bool = False):
  # if left.__len__() > right.__len__():
  #   return False
  correct = None
  while not correct:
    for i, l in enumerate(left):
      try: 
        r = right[i]
      except:
        return False
      if isinstance(l, list):
        if not isinstance(r, list):
          correct = compareLists(l, [r])
        else:
          correct = compareLists(l, r)
        if correct:
          return True
        elif correct == False:
          return False
      elif isinstance(right[i], list):
        correct = compareLists([l], r)
        if correct:
          return True
        elif correct == False:
          return False
      else:
        if l<r:
          return True
        if l>r:
          return False
        # correct = data<=right[i]
    if left.__len__() == right.__len__():
      if topLevel:
        return True
      else:
        return None
    elif left.__len__() < right.__len__():
      return True
    else:
        return False
  if topLevel:
    return True
  return correct
with open("/home/kat/AOC2022/day13/input", "r") as file:
  sum = 0
  index = 0
  while True:
    index+=1
    line = file.readline()
    line2 = file.readline()
    file.readline()
    if line == "":
      break
    l1 = []
    for c in line.strip():
      l1.append(c)
    l2 = []
    for c in line2.strip():
      l2.append(c)
    l1.pop()
    l1.pop(0)
    l2.pop()
    l2.pop(0)
    left = parseArray(l1)
    right = parseArray(l2)
    if compareLists(left, right, True):
      sum+=index

  print(sum)