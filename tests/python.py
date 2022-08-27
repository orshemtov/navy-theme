# Python code

from enum import Enum
import random

WATER = "💧"

class Coin(Enum):
    HEADS = 0
    TAILS = 1

def flip_coin() -> Coin:
    return Coin.HEADS if random.random() > 0.5 else Coin.TAILS

class Magician:
    def __init__(self, name: str):
        self.name = name

    def turn_water_into_whiskey(self, water: str):
        print(water)
        print("Enjoy your 🥃")

    def turn_water_into_wine(self, water: str):
        print(water)
        print("Enjoy your 🍷")



def main():
    magician = Magician("Harry Potter")

    if flip_coin() is Coin.HEADS:
        magician.turn_water_into_whiskey(WATER)
    else:
        magician.turn_water_into_wine(WATER)

if __name__ == "__main__":
    main()