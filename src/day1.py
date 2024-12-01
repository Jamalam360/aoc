from util import get_input
from collections import Counter

def get_lists() -> tuple[list[int], list[int]]:
	input = [int(num) for line in get_input(1) for num in line.split("   ")]
	return (input[0::2], input[1::2])

def part_one():
	left, right = get_lists()
	differences = [abs(b - a) for a, b in zip(sorted(left), sorted(right))]
	print(sum(differences))

def part_two():
	left, right = get_lists()
	right_occurrences = Counter(right)
	print(sum([left_num * right_occurrences[left_num] for left_num in left]))
