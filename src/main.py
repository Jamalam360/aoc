from datetime import date
from importlib import import_module
from sys import argv
from time import perf_counter

if __name__ == "__main__":
    days = argv[1:]

    if len(days) == 0:
        days.append(str(date.today().day))

    for day in days:
        module = import_module(f"day{day}")
        start = perf_counter()
        module.part_one()
        print(f"Day {day}, Part 1: {perf_counter() - start:.4f}ms")
        start = perf_counter()
        module.part_two()
        print(f"Day {day}, Part 2: {perf_counter() - start:.4f}ms")
