numbers = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]

def find_first_number(line: str) -> str:
    """Find the first digit in a line and return it.
    
    A digit is a number like '2' or a string of characters like 'two'
    """
    for position in range(len(line)):
        if line[position].isnumeric():
            return line[position]

        for index, x in enumerate(numbers):
            if line[position:].startswith(x):
                return str(index)

def find_last_number(line: str) -> str:
    """Find the last digit in a line and return it.
    
    A digit is a number like '2' or a string of characters like 'two'
    """
    for position in range(len(line) - 1, -1, -1):
        if line[position].isnumeric():
            return line[position]
        for index, x in enumerate(numbers):
            reversed_x = x[::-1]
            if line[position:0:-1].startswith(reversed_x):
                return str(index)
        


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