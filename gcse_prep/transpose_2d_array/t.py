#!/usr/bin/env python3

a = [
    [1, 2, 3, 4, 5],
    [6, 7, 8, 9, 10],
    [11, 12, 13, 14, 15],
]

b = [[]]
# b[0] -> b[[]]
# b[0][0] = a[0][0]

# location -> `for in range()`
# contents -> `for in list`
# both -> `for i in range()`, and then `list[i]`

for row in range(len(a)):
    for col in range(len(a[row])):
        if len(b) <= col:
            b.append([])
        b[col].append(a[row][col])

print(b)
