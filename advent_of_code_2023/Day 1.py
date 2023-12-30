f = open("./day-01-input", "r")
input = f.read()
total = 0
for line in input.splitlines():
    for position in range(len(line)):
        if line[position].isnumeric():
            num1 = line[position]
            break
        if line[position:].startswith("one"):
            num1 = "1"
            break
        if line[position:].startswith("two"):
            num1 = "2"
            break
        if line[position:].startswith("three"):
            num1 = "3"
            break
        if line[position:].startswith("four"):
            num1 = "4"
            break
        if line[position:].startswith("five"):
            num1 = "5"
            break
        if line[position:].startswith("six"):
            num1 = "6"
            break
        if line[position:].startswith("seven"):
            num1 = "7"
            break
        if line[position:].startswith("eight"):
            num1 = "8"
            break
        if line[position:].startswith("nine"):
            num1 = "9"
            break
        if line[position:].startswith("zero"):
            num1 = "0"
            break

    for position in range(len(line) - 1, -1, -1):
        if line[position].isnumeric():
            num_last = line[position]
            break
        if line[position:0:-1].startswith("eno"):
            num_last = "1"
            break
        if line[position:0:-1].startswith("owt"):
            num_last = "2"
            break
        if line[position:0:-1].startswith("eerht"):
            num_last = "3"
            break
        if line[position:0:-1].startswith("ruof"):
            num_last = "4"
            break
        if line[position:0:-1].startswith("evif"):
            num_last = "5"
            break
        if line[position:0:-1].startswith("xis"):
            num_last = "6"
            break
        if line[position:0:-1].startswith("neves"):
            num_last = "7"
            break
        if line[position:0:-1].startswith("thgie"):
            num_last = "8"
            break
        if line[position:0:-1].startswith("enin"):
            num_last = "9"
            break
        if line[position:0:-1].startswith("orez"):
            num_last = "0"
            break
    digit = num1 + num_last
    print(digit)
    total += int(digit)
print(total)

