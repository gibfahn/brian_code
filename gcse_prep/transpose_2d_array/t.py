#!/usr/bin/env python3

a = [
    [1,2,3,4,5],
    [6,7,8,9,10],
    [11,12,13,14,15],
]

b = []

for row in range(0, len(a)):
  for col in range(0, len(a[row])):
    if len(b) <= col:
      # b += [[]]
      b.append([])
    if len(b[col]) <= row:
      #b[col] += [[]]
      b[col].append([])

    el = a[row][col]
    b[col][row] = el
   

print(b)