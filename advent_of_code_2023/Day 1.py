f = open("./day-01-input.txt", "r")
input = f.read()
total = 0
for line in input.splitlines():
    for char in line:
        if char.isnumeric():
            num1 = char
            break

    for char in line[::-1]:
        if char.isnumeric():
            num_last = char
            break
    digit = num1 + num_last
    total += int(digit)
print(total)
