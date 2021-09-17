# Rust Kata

## 2 kyu
* [Evaluate mathematical expression](https://www.codewars.com/kata/52a78825cdfc2cfc87000005)
		* Parses and evaluates algebraic expressions in infix form
			containing binary operators and unary negation
		* Implements the [*Shunting-yard algorithm*](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
		* Implemented in package [`eval-expression`](https://github.com/matyama/codewars/tree/main/rust/eval-expression)
* [Symbolic differentiation of prefix expressions](https://www.codewars.com/kata/584daf7215ac503d5a0001ae)
		* Parsing prefix expressions into an algebraic tree representation
		* Implements symbolic differentiation in single variable with basic
			unary functions and binary operators
		* Implemented in package [`differentiation`](https://github.com/matyama/codewars/tree/main/rust/differentiation)

## 3 kyu
* [Make a spiral](https://www.codewars.com/kata/534e01fbbb17187c7e0000c6)
		* Creates a square matrix of given size with a spiral-like pattern
		* Implemented in package [`spiralize`](https://github.com/matyama/codewars/tree/main/rust/spiralize)
* [Screen Locking Patterns](https://www.codewars.com/kata/585894545a8a07255e0002f1)
		* Counts the number of possible patterns on an Android lock screen
			starting from given position
		* Implemented as an exhaustive backtracking search
		* Implemented in package [`screen-patterns`](https://github.com/matyama/codewars/tree/main/rust/screen-patterns)

## 4 kyu
* [Algebraic Lists](https://www.codewars.com/kata/529a92d9aba78c356b000353)
		* Persistent list implemented via clone with `map` and `filter` API
		* Implemented in package [`algebraic-lists`](https://github.com/matyama/codewars/tree/main/rust/algebraic-lists)
* [Getting along with Integer Partitions](https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b)
		* Implements *Integer Partitioning* and collects statistics about
			unique partition products
		* Implemented in package [`integer-partitions`](https://github.com/matyama/codewars/tree/main/rust/integer-partitions)
* [Magnet particles in boxes](https://www.codewars.com/kata/56c04261c3fcf33f2d000534)
		* Simple transformation and aggregation over an `Iterator`
		* Implemented in package [`magnet-particles`](https://github.com/matyama/codewars/tree/main/rust/magnet-particles)
* [Snail](https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1)
		* Produces spiral trail visiting all elements of given matrix
		* Implemented in package [`snail`](https://github.com/matyama/codewars/tree/main/rust/snail)
* [Sum by Factors](https://www.codewars.com/kata/54d496788776e49e6b00052f)
		* Collects all *prime factors* of numbers in a given list and
			aggregates sum of respective numbers for each prime factor
		* Implemented in package [`prime-factors`](https://github.com/matyama/codewars/tree/main/rust/prime-factors)
* [Validate Sudoku with size `NxN`](https://www.codewars.com/kata/540afbe2dc9f615d5e000425)
		* Sudoku board checked
		* Implemented in package [`validate-sudoku`](https://github.com/matyama/codewars/tree/main/rust/validate-sudoku)

## 5 kyu
* [Best travel](https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa)
		* Iterator transformations `map`, `filter` and aggregation
		* Implemented in package [`best-travel`](https://github.com/matyama/codewars/tree/main/rust/best-travel)
* [Graph-like Sequence](https://www.codewars.com/kata/60815326bbb0150009f55f7e/rust)
		* Solves the [Graph Realization Problem](https://en.wikipedia.org/wiki/Graph_realization_problem)
		* Implements the *Erdős-Gallai approach* in `O(n*log(n))` time
		* Implemented in package [`simple-graph`](https://github.com/matyama/codewars/tree/main/rust/simple-graph)
* [Molecule to atoms](https://www.codewars.com/kata/52f831fa9d332c6591000511)
		* Parsing textual representation of a molecule
		* Counting number of atoms in compound molecules
		* Implemented in package [`molecule`](https://github.com/matyama/codewars/tree/main/rust/molecule)
* [Perimeter of squares in a rectangle](https://www.codewars.com/kata/559a28007caad2ac4e000083)
		* Computing sum of first *n* Fibonacci numbers
		* Implemented in package [`perimeter`](https://github.com/matyama/codewars/tree/main/rust/perimeter)
* [Product of consecutive Fib numbers](https://www.codewars.com/kata/5541f58a944b85ce6d00006a)
		* Producing consecutive Fibonacci numbers until a condition on their
			product
		* Implemented in package [`product-fib`](https://github.com/matyama/codewars/tree/main/rust/product-fib)

## 6 kyu
* [Bouncing Ball](https://www.codewars.com/kata/5544c7a5cb454edb3c000047)
		* Calculating number of falling ball bounces.
		* Implemented in package [`bouncing-balls`](https://github.com/matyama/codewars/tree/main/rust/bouncing-balls)
* [Counting Duplicates](https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1)
		* Filtering characters based on occurrences via `HashMap`
		* Implemented in package [`count-duplicates`](https://github.com/matyama/codewars/tree/main/rust/count-duplicates)
* [Delete nth](https://www.codewars.com/kata/554ca54ffa7d91b236000023)
		* Slice filtering with state
		* Implemented in package [`delete-nth`](https://github.com/matyama/codewars/tree/main/rust/delete-nth)
* [Dubstep](https://www.codewars.com/kata/551dc350bf4e526099000ae5)
		* `&str` and `Iterator` APIs: `split`, `filter`, `collect` and `join`
		* Implemented in package [`dubstep`](https://github.com/matyama/codewars/tree/main/rust/dubstep)
* [Persistent Bugger](https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec)
		* Nested recursion
		* Implemented in package [`persistent-bugger`](https://github.com/matyama/codewars/tree/main/rust/persistent-bugger)
* [Statistics for an Athletic Association](https://www.codewars.com/kata/55b3425df71c1201a800009c)
		* `Duration` parsing, computing *range*, *mean* and *median*
		* Implemented in package [`caa-stats`](https://github.com/matyama/codewars/tree/main/rust/caa-stats)
* [Sums of Parts](https://www.codewars.com/kata/5ce399e0047a45001c853c2b)
		* Basic looping and aggregation
		* Implemented in package [`parts-sums`](https://github.com/matyama/codewars/tree/main/rust/parts-sums)
* [Which are in?](https://www.codewars.com/kata/550554fd08b86f84fe000a58)
		* `Vec` and `Iterator` APIs: `map`, `filter`, `collect`, `sort` and
			`dedup`
		* Implemented in package [`in-array`](https://github.com/matyama/codewars/tree/main/rust/in-array)

## 7 kyu
* [Growth of a Population](https://www.codewars.com/kata/563b662a59afc2b5120000c6)
		* Basic looping and aggregation
		* Implemented in package [`population-growth`](population-growth)
