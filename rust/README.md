# Rust Kata

## 2 kyu
* [Algebraic Isomorphism](https://www.codewars.com/kata/5917f22dd2563a36a200009c)
  * Implements bunch of algebraic rules which give the name to
    [*Algebraic Data Types*](https://en.wikipedia.org/wiki/Algebraic_data_type)
  * Algebraic rules are satisfied by ADTs under *isomorphism*
  * This is a follow-up to the *Isomorphism* kata and is implemented in
    module [`algebraic_isomorphism`](src/algebraic_isomorphism.rs)
* [Assembler interpreter (Part II)](https://www.codewars.com/kata/58e61f3d8ff24f774400002c)
  * An interpreter of assembler which will support the following
    instructions: `mov x, y`, `inc x`, `dec x`, `add x, y`, `sub x, y`,
    `sub x, y`, `mul x, y`, `div x, y`, `label:`, `jmp lbl`, `jne lbl`,
    `je lbl`, `jge lbl`, `jg lbl`, `jle lbl`, `jl lbl`, `call lbl`,
    `ret`, `msg 'Register: ', x`, `end`, `; comment`
  * Implemented in module [`assembler_interpreter`](src/assembler_interpreter.rs)
* [Evaluate mathematical expression](https://www.codewars.com/kata/52a78825cdfc2cfc87000005)
  * Parses and evaluates algebraic expressions in infix form containing
		binary operators and unary negation
  * Implements the [*Shunting-yard algorithm*](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
  * Implemented in module [`eval_expression`](src/eval_expression.rs)
* [Symbolic differentiation of prefix expressions](https://www.codewars.com/kata/584daf7215ac503d5a0001ae)
  * Parsing prefix expressions into an algebraic tree representation
  * Implements symbolic differentiation in single variable with basic
		unary functions and binary operators
  * Implemented in module [`differentiation`](src/differentiation.rs)

## 3 kyu
* [Binomial Expansion](https://www.codewars.com/kata/540d0fdd3b6532e5c3000b5b)
  * Expands an expression of the form `(ax+b)^n` using the
    [*Binomial formula*](https://en.wikipedia.org/wiki/Binomial_theorem)
  * Implemented in module [`binomial_expansion`](src/binomial_expansion.rs)
* [Closest pair of points in linearithmic time](https://www.codewars.com/kata/5376b901424ed4f8c20002b7)
  * Finds the [closest pair of points](https://en.wikipedia.org/wiki/Closest_pair_of_points_problem)
    in a list of `n` 2D points in `O(n*log(n))` time
  * [*Divide and conquer*](https://en.wikipedia.org/wiki/Divide-and-conquer_algorithm) algorithm
  * Implemented in module [`closest_points`](src/closest_points.rs)
* [Isomorphism](https://www.codewars.com/kata/5922543bf9c15705d0000020)
  * Definition of an [*isomorphism*](https://en.wikipedia.org/wiki/Isomorphism)
  * Validation of properties of isomorphisms: equivalence relation,
    lifting to a `Functor` and `Bifunctor`, lowering isomorphisms over
    `Maybe` and evidence of the impossibility to lower fromISO an `Either`
  * Implemented in module [`isomorphism`](src/isomorphism.rs)
* [Make a spiral](https://www.codewars.com/kata/534e01fbbb17187c7e0000c6)
  * Creates a square matrix of given size with a spiral-like pattern
  * Implemented in module [`spiralize`](src/spiralize.rs)
* [Prime Streaming (PG-13)](https://www.codewars.com/kata/5519a584a73e70fa570005f5)
  * An endless iterator that returns prime numbers
  * The iterator is able to produce 50 million primes in a few seconds
  * Implements the
    [_The Genuine Sieve of Eratosthenes_](https://doi.org/10.1017/S0956796808007004)
    paper
  * Implemented in module [`primes`](src/primes.rs)
* [Rail Fence Cipher: Encoding and Decoding](https://www.codewars.com/kata/58c5577d61aefcf3ff000081)
  * Implements an encoder and decoder for the
    [Rail Fence Cipher](https://en.wikipedia.org/wiki/Rail_fence_cipher)
  * Implemented in module [`rail_fence_cipher`](src/rail_fence_cipher.rs)
* [Screen Locking Patterns](https://www.codewars.com/kata/585894545a8a07255e0002f1)
  * Counts the number of possible patterns on an Android lock screen
		starting from given position
  * Implemented as an exhaustive backtracking search
  * Implemented in module [`screen_patterns`](src/screen_patterns.rs)
* [Sudoku Solver](https://www.codewars.com/kata/5296bc77afba8baa690002d7)
  * Implements a [*CSP-based Sudoku solver*](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem)
    with [AC3](https://en.wikipedia.org/wiki/AC-3_algorithm) checking
    and *MRV* heuristic
  * Implemented in module [`sudoku_solver`](src/sudoku_solver.rs)

## 4 kyu
* [Algebraic Lists](https://www.codewars.com/kata/529a92d9aba78c356b000353)
  * Persistent list implemented via clone with `map` and `filter` API
  * Implemented in module [`algebraic_lists`](src/algebraic_lists.rs)
* [Getting along with Integer Partitions](https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b)
  * Implements *Integer Partitioning* and collects statistics about
		unique partition products
  * Implemented in module [`integer_partitions`](src/integer_partitions.rs)
* [Magnet particles in boxes](https://www.codewars.com/kata/56c04261c3fcf33f2d000534)
  * Simple transformation and aggregation over an `Iterator`
  * Implemented in module [`magnet_particles`](src/magnet_particles.rs)
* [Organize a Round-robin tournament](https://www.codewars.com/kata/561c20edc71c01139000017c)
  * Build a matches table for a round-robin tournament given the number
    of players
  * Implements the [_Circle method_](https://en.wikipedia.org/wiki/Round-robin_tournament)
  * Implemented in module [`round_robin_tourney`](src/round_robin_tourney.rs)
* [Path Finder #1: can you reach the exit?](https://www.codewars.com/kata/5765870e190b1472ec0022a2)
  * Checks if there is a path between two locations in a 2D grid maze
  * Implements simplified version of [(Weighted) A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
    with a Manhattan distance heuristic
  * Implemented in module [`path_finder`](src/path_finder.rs)
* [Recover a secret string from random triplets](https://www.codewars.com/kata/53f40dff5f9d31b813000774)
  * Decodes a text from triplets of letters that define a precedence
    relation on their unique occurrence in the secret message
  * Implements [*topological sort*](https://en.wikipedia.org/wiki/Topological_sorting)
    on the graph of symbol precedences
  * Implemented in module [`recover_secret`](src/recover_secret.rs)
* [Snail](https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1)
  * Produces spiral trail visiting all elements of given matrix
  * Implemented in module [`snail`](src/snail.rs)
* [Social Golfer Problem Validator](https://www.codewars.com/kata/556c04c72ee1147ff20000c9)
  * Write a function that validates a proposed solution for a match
    schedule
  * Implemented in module [`social_golfer`](src/social_golfer.rs)
* [Sort binary tree by levels](https://www.codewars.com/kata/52bef5e3588c56132c0003bc)
  * Traverses given tree in BFS order yielding its values
  * Implements [*Breadth-first search*](https://en.wikipedia.org/wiki/Breadth-first_search)
  * Implemented in module [`tree_levels`](src/tree_levels.rs)
* [Sum by Factors](https://www.codewars.com/kata/54d496788776e49e6b00052f)
  * Collects all *prime factors* of numbers in a given list and
		aggregates sum of respective numbers for each prime factor
  * Implemented in module [`prime_factors`](src/prime_factors.rs)
* [Sum of Intervals](https://www.codewars.com/kata/52b7ed099cdc285c300001cd)
  * Compute the total length of `n` overlapping intervals in
    `O(n*log(n))` time
  * Implemented in module [`sum_intervals`](src/sum_intervals.rs)
* [Twice linear](https://www.codewars.com/kata/5672682212c8ecf83e000050)
  * Finds the nth item of an ordered set that is closed on two linear
    transformations of its elements
  * This implementation runs in `O(n*log(n))` time but could be improved
    to `O(n)` (see other solutions)
  * Implemented in module [`double_linear`](src/double_linear.rs)
* [Validate Sudoku with size `NxN`](https://www.codewars.com/kata/540afbe2dc9f615d5e000425)
  * Sudoku board validator
  * Implemented in module [`validate_sudoku`](src/validate_sudoku.rs)

## 5 kyu
* [Best travel](https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa)
  * Iterator transformations `map`, `filter` and aggregation
  * Implemented in module [`best_travel`](src/best_travel.rs)
* [Graph-like Sequence](https://www.codewars.com/kata/60815326bbb0150009f55f7e)
  * Solves the [Graph Realization Problem](https://en.wikipedia.org/wiki/Graph_realization_problem)
  * Implements the *Erdős-Gallai approach* in `O(n*log(n))` time
  * Implemented in module [`simple_graph`](src/simple_graph.rs)
* [Molecule to atoms](https://www.codewars.com/kata/52f831fa9d332c6591000511)
  * Parsing textual representation of a molecule
  * Counting number of atoms in compound molecules
  * Implemented in module [`molecule`](src/molecule.rs)
* [Moving Zeros To The End](https://www.codewars.com/kata/52597aa56021e91c93000cb0)
  * Write an algorithm that takes an array and moves all of the zeros to
    the end, preserving the order of the other elements
  * Implemented in module [`move_zeros`](src/move_zeros.rs)
* [Perimeter of squares in a rectangle](https://www.codewars.com/kata/559a28007caad2ac4e000083)
  * Computing sum of first *n* Fibonacci numbers
  * Implemented in module [`perimeter`](src/perimeter.rs)
* [Product of consecutive Fib numbers](https://www.codewars.com/kata/5541f58a944b85ce6d00006a)
  * Producing consecutive Fibonacci numbers until a condition on their
		product
  * Implemented in module [`product_fib`](src/product_fib.rs)
* [Simple assembler interpreter](https://www.codewars.com/kata/58e24788e24ddee28e000053)
  * A simple interpreter of assembler which supports:
    `mov x y`, `inc x`, `dec x`, and `jnz x y`
  * Implemented in module [`simple_assembler`](codewars/simple_assembler.py)
* [String incrementer](https://www.codewars.com/kata/54a91a4883a7de5d7800009c)
  * Write a function which parses and increments a trailing counter from
    a string input
  * Implemented in module [`string_incrementer`](src/string_incrementer.rs)
* [Sum of Pairs](https://www.codewars.com/kata/54d81488b981293527000c8f)
  * Find pair of integers which sums up to given value in sub-quadratic
    time
  * Implemented in module [`sum_pairs`](src/sum_pairs.rs)

## 6 kyu
* [Bouncing Ball](https://www.codewars.com/kata/5544c7a5cb454edb3c000047)
  * Calculating number of falling ball bounces.
  * Implemented in module [`bouncing_balls`](src/bouncing_balls.rs)
* [Counting Duplicates](https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1)
  * Filtering characters based on occurrences via `HashMap`
  * Implemented in module [`count_duplicates`](src/count_duplicates.rs)
* [Delete nth](https://www.codewars.com/kata/554ca54ffa7d91b236000023)
  * Slice filtering with state
  * Implemented in module [`delete_nth`](src/delete_nth.rs)
* [Dubstep](https://www.codewars.com/kata/551dc350bf4e526099000ae5)
  * `&str` and `Iterator` APIs: `split`, `filter`, `collect` and `join`
  * Implemented in module [`dubstep`](src/dubstep.rs)
* [Persistent Bugger](https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec)
  * Nested recursion
  * Implemented in module [`persistent_bugger`](src/persistent_bugger.rs)
* [Statistics for an Athletic Association](https://www.codewars.com/kata/55b3425df71c1201a800009c)
  * `Duration` parsing, computing *range*, *mean* and *median*
  * Implemented in module [`caa_stats`](src/caa_stats.rs)
* [Sums of Parts](https://www.codewars.com/kata/5ce399e0047a45001c853c2b)
  * Basic looping and aggregation
  * Implemented in module [`parts_sums`](src/parts_sums.rs)
* [Which are in?](https://www.codewars.com/kata/550554fd08b86f84fe000a58)
  * `Vec` and `Iterator` APIs: `map`, `filter`, `collect`, `sort` and
		`dedup`
  * Implemented in module [`in_array`](src/in_array.rs)

## 7 kyu
* [Growth of a Population](https://www.codewars.com/kata/563b662a59afc2b5120000c6)
  * Basic looping and aggregation
  * Implemented in module [`population_growth`](src/population_growth.rs)
