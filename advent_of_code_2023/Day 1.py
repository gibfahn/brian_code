numbers = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
    "zero": "0",
}

def find_first_number(line: str) -> str:
    for position in range(len(line)):
        if line[position].isnumeric():
            return line[position]

        for x in numbers:
            if line[position:].startswith(x):
                return numbers[x]

def find_last_number(line: str) -> str:
    for position in range(len(line) - 1, -1, -1):
        if line[position].isnumeric():
            return line[position]
        for x in numbers:
            reversed_x = x[::-1]
            if line[position:0:-1].startswith(reversed_x):
                return numbers[x]
        


f = open("./day-01-input", "r")
input = f.read()
total = 0

for line in input.splitlines():
    num1 = find_first_number(line)
    num_last = find_last_number(line)

    digit = num1 + num_last
    print(digit)
    total += int(digit)
print(total)