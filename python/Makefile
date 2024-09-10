.PHONY: all check fmt lint tests
.DEFAULT_GOAL := all

install:
	poetry env use python3.11
	poetry install

fmt:
	ruff format

lint:
	ruff format --check
	ruff check
	mypy .

tests:
	pytest

check: lint tests

all: fmt check
