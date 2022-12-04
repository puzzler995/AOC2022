#!/usr/bin/ksh

inputFile="./input"
typeset -i total
total=0
elves=()
while read -r line
do
  if [ "$line" = "" ]
  then
    elves+=("$total")
    total=0
  else
    total+=$line
  fi
done <"$inputFile"

sorted=($(echo ${elves[*]}| tr " " "\n" | sort -nr))
typeset -i first
typeset -i second
typeset -i third
first=${sorted[0]}
second=${sorted[1]}
third=${sorted[2]}
total=$first+$second+$third
echo $total