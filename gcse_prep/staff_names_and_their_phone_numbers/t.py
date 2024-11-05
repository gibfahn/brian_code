#!/usr/bin/env python3

StaffName = ["Janice West", "Frank Knight", "Elsa Alsace-Lorraine", "Robert Bobert"]

StaffPhone = ["01923489154", "01942865292", "07752032491", "07654321098"]

StaffOffice = ["London", "Tokyo", "Peru", "New York"]

def bubble_sort(arr: list):
  swaps = True
  while swaps:
    swaps = False
    for i in range(0, len(arr) - 1):
      if arr[i][0] > arr[i+1][0]:
        arr[i], arr[i+1] = arr[i+1], arr[i]
        swaps = True

def display_staff():
  name_and_phone = []
  for i in range(0, len(StaffName)):
      name_and_phone += [[StaffName[i], StaffPhone[i]]]
  bubble_sort(name_and_phone)
  print(name_and_phone)

display_staff()

"""
arr = [[]]
for i in range(0, len(StaffName)):
    arr.append([StaffName[i]])
print(arr)
for r in range(0, len(StaffPhone)):
    a =0
"""