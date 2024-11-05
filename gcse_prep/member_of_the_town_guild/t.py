#!/usr/bin/env python3
import random

requirements_passed = 0
failed_requirements = []

age = int(input("how old are you: "))
if age >= 18 and age <= 65:
  requirements_passed += 1
  print(f"{requirements_passed}/5: Old enough")
  age_failed = False
else:
  failed_requirements += ["Your age doesn't meet the requirement."]
  age_failed = True

life_input = int(input("How long have you lived in the town for? "))
if (life_input >= 20 or 
    life_input >= 5 and input("are your parents still in town? ") == "yes" or
    input("One parent dead, one lives in town?") == "yes" or
    input("Bro or sis and you lived in town for 10 years?") == "yes"
  ):
  requirements_passed += 1
  print(f"{requirements_passed}/5: Life enough")
else:
  failed_requirements += ["Don't meet the time in town / parents requirements"]

if int(input("How much money do you have in your current bank account: ")) >= 500000:
  requirements_passed += 1
  print(f"{requirements_passed}/5: Rich enough")
else:
  failed_requirements += ["Not rich enough"]


if input("Have you gone to prison? ") == "no":
  requirements_passed += 1
  print(f"{requirements_passed}/5: Prison enough")
else:
  failed_requirements += ["went to prison"]


support = int(input("How many members of town guild support you: "))
if support > 2:
  requirements_passed += 1
  print(f"{requirements_passed}/5: Member support enough")
else:
  failed_requirements += ["Not enough members support you"]

if requirements_passed == 5 or (requirements_passed == 4 and support >= 5 and age_failed == False):
  member_num = random.randint(0, 100000000)
  print(f"Welcome, member number: {member_num}")
else:
  print(f"you failed to meet the requirements, you only met {requirements_passed} of 5")
  for failed_requirement in failed_requirements:
    print(failed_requirement)