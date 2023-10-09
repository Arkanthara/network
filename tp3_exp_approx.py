import random

def exponentiel(l):
    n = 10**(-100)
    k = 0
    while (random.random(0, 1) < 1 - l/n):
        k = k + 1

    return k/n
