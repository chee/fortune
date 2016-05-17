import random
print random.choice(open("fortunes").read().strip().split("\n%\n"))
