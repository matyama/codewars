.PHONY: check fmt lint tests

setup:
	poetry env use python3.8
	poetry install

fmt:
	isort .
	black .

lint:
	flake8 .
	pylint codewars tests
	mypy .

tests:
	pytest

check: lint tests
