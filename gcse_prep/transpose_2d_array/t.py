#!/usr/bin/env python3

a = [
    [1, 2, 3, 4, 5],
    [6, 7, 8, 9, 10],
    [11, 12, 13, 14, 15],
]

b = []

# Transpose b so the assert below passes
# for row in range(len(a)):
#     for col in range(len(a[row])):
#         if len(b) <= col:
#             b.append([])
#         b[col].append(a[row][col])

b = [list(i) for i in zip(*a)]

print(b)

assert b == [
    [1, 6, 11],
    [2, 7, 12],
    [3, 8, 13],
    [4, 9, 14],
    [5, 10, 15],
]
