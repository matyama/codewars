.PHONY: build clean fmt lint setup setup-dev test

all: setup build test lint

build:
	stack build --coverage --fast --pedantic --test --no-run-tests

clean:
	stack clean

fmt:
	stack exec -- brittany --write-mode=inplace *.hs

lint:
	stack exec -- brittany -c *.hs
	stack exec -- hlint .

setup:
	stack setup
	stack build --dependencies-only --test --no-run-tests

setup-dev:
	stack install brittany hlint

test:
	stack build --coverage --fast --pedantic --test
