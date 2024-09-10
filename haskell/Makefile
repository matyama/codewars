.PHONY: build clean fmt lint setup setup-dev test

all: setup build test lint

build:
	stack build --coverage --fast --pedantic --test --no-run-tests

clean:
	stack clean

fmt: FILES := $(shell git ls-files '*.hs')
fmt:
	stack exec -- fourmolu --mode inplace $(FILES)

lint: FILES := $(shell git ls-files '*.hs')
lint:
	stack exec -- fourmolu --mode check $(FILES)
	stack exec -- hlint .

setup:
	stack setup
	stack build --dependencies-only --test --no-run-tests

setup-dev:
	stack install --copy-compiler-tool $$(xargs -a dependencies.dev)

test:
	stack build --coverage --fast --pedantic --test
