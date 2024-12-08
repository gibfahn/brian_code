#!/usr/bin/env python3

n = input("What number are you going to start with: ")
i = int(input("How long do you want the string to run: "))
y = int(n)
current_length = i - len(n)
while len(n) < i:
  n += str(y + 1)
  y += 1
  current_length -= len(n)
print(n)
print(len(n))
number = n[i - 1]
print(number)
