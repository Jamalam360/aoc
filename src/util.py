import pathlib

def get_input(day: int) -> list[str]:
	with open(pathlib.Path(__file__).parent.resolve().joinpath(f"./inputs/{day}.txt")) as f:
		return f.readlines()