guide = open("/home/kat/AOC2022/day2/input", "r")
myScore = 0
badScore = 0

for line in guide.readlines():
  moves = line.split()
  badMove = moves[0]
  myMove = moves[1]
  match badMove:
    case "A":
      match myMove:
        case "X":
          myScore = myScore + 3
          badScore = badScore + 7 
        case "Y":
          myScore = myScore + 4
          badScore = badScore + 4
        case "Z":
          myScore = myScore + 8
          badScore = badScore + 1
    case "B":
      match myMove:
        case "X":
          myScore = myScore + 1
          badScore = badScore + 8
        case "Y":
          myScore = myScore + 5
          badScore = badScore + 5
        case "Z":
          myScore = myScore + 9
          badScore = badScore + 2
    case "C":
      match myMove:
        case "X":
          myScore = myScore + 2
          badScore = badScore + 9
        case "Y":
          myScore = myScore + 6
          badScore = badScore + 6
        case "Z":
          myScore = myScore + 7
          badScore = badScore + 3

print("My Score: ",myScore)
print("Their Score: ",badScore)