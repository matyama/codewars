# Python Kata

## 2 kyu
* [Symbolic differentiation of prefix expressions](https://www.codewars.com/kata/584daf7215ac503d5a0001ae)
  * Symbolic differentiation of prefix expressions with a fixed set of
    unary functions and binary operations in single variable
  * Abstract classes, `dataclass`, meta-classes, recursion, parsing
  * Implemented in module [`differentiation`](codewars/differentiation.py)

## 3 kyu
* [Binomial Expansion](https://www.codewars.com/kata/540d0fdd3b6532e5c3000b5b)
  * Expands an expression of the form `(ax+b)^n` using the [*Binomial formula*](https://en.wikipedia.org/wiki/Binomial_theorem)
  * Implemented in module [`binomial_expansion`](codewars/binomial_expansion.py)
* [Closest pair of points in linearithmic time](https://www.codewars.com/kata/5376b901424ed4f8c20002b7)
  * Finds the [closest pair of points](https://en.wikipedia.org/wiki/Closest_pair_of_points_problem)
    in a list of `n` 2D points in `O(n*log(n))` time
  * [*Divide and conquer*](https://en.wikipedia.org/wiki/Divide-and-conquer_algorithm) algorithm
  * Implemented in module [`closest_points`](codewars/closest_points.py)
* [GET TO THE CHOPPA!](https://www.codewars.com/kata/5573f28798d3a46a4900007a)
  * Implements *A** with *Manhattan distance* heuristic to find the
    shortest path in a *gridworld* environment
  * Implemented in module [`grid_path`](codewars/grid_path.py)
* [Huffman Encoding](https://www.codewars.com/kata/54cf7f926b85dcc4e2000d9d)
  * Implements[*Huffman coding*](https://en.wikipedia.org/wiki/Huffman_coding)
  * Implemented in module [`huffman`](codewars/huffman.py)
* [The Millionth Fibonacci](https://www.codewars.com/kata/53d40c1e2f13e331fc000c26)
  * Calculates the n-th *Fibonacci number* in `O(log(n))` time
  * Implemented in module [`fibonacci`](codewars/fibonacci.py)
* [Sudoku Solver](https://www.codewars.com/kata/5296bc77afba8baa690002d7)
  * Implements a [*CSP-based Sudoku solver*](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem)
    with [AC3](https://en.wikipedia.org/wiki/AC-3_algorithm) checking and
    *MRV* heuristic
  * Implemented in module [`sudoku_solver`](codewars/sudoku_solver.py)

## 4 kyu
* [Conway's Game of Life](https://www.codewars.com/kata/52423db9add6f6fc39000354)
  * Simulates given number of iterations of [*Conway's Game of Life*](http://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
    with an infinite universe and cropping to the area containing living
    cells
  * Implemented in module [`game_of_life`](codewars/game_of_life.py)
* [Decode the Morse code, advanced](https://www.codewars.com/kata/54b72c16cd7f5154e9000457)
  * Implements an advanced [*Morse code*](https://en.wikipedia.org/wiki/Morse_code)
    decoder from a binary message with an unknown transmission rate
  * Implemented in module [`decode_morse_adv`](codewars/decode_morse_adv.py)
* [Matrix Determinant](https://www.codewars.com/kata/52a382ee44408cea2500074c)
  * Computes the *determinant* of given square matrix via the
    [*Laplace expansion*](https://en.wikipedia.org/wiki/Laplace_expansion)
  * Implemented in module [`determinant`](codewars/determinant.py)
* [parseInt() reloaded](https://www.codewars.com/kata/525c7c5ab6aecef16e0001a5)
  * Converts given string representing a number in words into an integer
  * Implemented in module [`parse_int`](codewars/parse_int.py)
* [Range Extraction](https://www.codewars.com/kata/51ba717bb08c1cd60f00002f)
  * Format a list of integers and shorten it by replacing consecutive
    sequences by a range
  * Stateful iteration, `yield` and `yield from` generator functions
  * Implemented in module [`range_extraction`](codewars/range_extraction.py)
* [Shortest Knight Path](https://www.codewars.com/kata/549ee8b47111a81214000941)
  * Finds shortest path of a knight piece on a chess board between two
    positions
  * Implements [*Dijkstra's SP algorithm*](https://en.wikipedia.org/wiki/Dijkstra's_algorithm)
    ([*Uniform-cost Search*](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Practical_optimizations_and_infinite_graphs))
  * Implemented in module [`knight_path`](codewars/knight_path.py)
* [Sort binary tree by levels](https://www.codewars.com/kata/52bef5e3588c56132c0003bc)
  * Traverses given tree in BFS order yielding its values
  * Implements [*Breadth-first search*](https://en.wikipedia.org/wiki/Breadth-first_search)
  * Implemented in module [`tree_by_levels`](codewars/tree_by_levels.py)
* [Strip Comments](https://www.codewars.com/kata/51c8e37cee245da6b40000bd)
  * Strips end of line comments from given text and a set of comment
    markers
  * Implemented in module [`strip_comments`](codewars/strip_comments.py)
* [Strongly connected components](https://www.codewars.com/kata/5f74a3b1acfbb20033e5b7d9)
  * Finds strongly connected components in given directed graph
  * Implements [*Tarjan's SCC algorithm*](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm)
  * Implemented in module [`scc`](codewars/scc.py)
* [Sum of Intervals](https://www.codewars.com/kata/52b7ed099cdc285c300001cd)
  * Compute the total length of `n` overlapping intervals in
    `O(n*log(n))` time
  * Implemented in module [`sum_intervals`](codewars/sum_intervals.py)
* [The learning game - Machine Learning #1](https://www.codewars.com/kata/5695995cc26a1e90fe00004d)
  * Implements a machine which learns how to respond to a set of
    commands by selecting correct action to handle them
  * Implements a simple greedy actor for deterministic environments and
    [(Bernoulli) MAB](https://en.wikipedia.org/wiki/Multi-armed_bandit)
    (**UCB-1**) for stochastic environments/actions
  * Implemented in module [`learning_game`](codewars/learning_game.py)
* [The observed PIN](https://www.codewars.com/kata/5263c6999e0f40dee200059d)
  * Finds all possible pins on a numpad given observed sequence where each
    observed digit might be wrong
  * Implements a recursive [*Depth-first Search*](https://en.wikipedia.org/wiki/Depth-first_search)
  * Implemented in module [`pins`](codewars/pins.py)

## 5 kyu
* [Directions Reduction](https://www.codewars.com/kata/550f22f4d758534c1100025a)
  * Shortens given path of North, East, South, West directions by
    removing directly opposite pairs
  * Comprehension, `Deque` and `IntEnum`
  * Implemented in module [`directions_reduction`](codewars/directions_reduction.py)
* [Human Readable Time](https://www.codewars.com/kata/52685f7382004e774f0001f7)
  * Converts given number of seconds into a string in `HH:MM:SS` format
  * Implemented in module [`readable_time`](codewars/readable_time.py)
* [Intro to Statistics - Part 1: A Five figure summary](https://www.codewars.com/kata/555c7fa8d8cb57834a000028)
  * Computes the
    [five-number summary](https://en.wikipedia.org/wiki/Five-number_summary)
    over a series of samples
  * The usage of `numpy`/`pandas` is disallowed for this Kata
  * Implemented in module [`five_figure_summary`](codewars/five_figure_summary.py)
* [Mean without outliers](https://www.codewars.com/kata/5962d557be3f8bb0ca000010)
  * Computes the mean of given sample stripped from outliers defined as
    a multiple of standard deviations form the sample mean.
  * Implemented in module [`clean_mean`](codewars/clean_mean.py)
* [Number of trailing zeros of N!](https://www.codewars.com/kata/52f787eb172a8b4ae1000a34)
  * Computes the number of trailing zeros in `n!` without evaluating it
  * Implemented in module [`factorial_zeros`](codewars/factorial_zeros.py)
* [RGB To Hex Conversion](https://www.codewars.com/kata/513e08acc600c94f01000001)
  * Formats given RGB triplet as a hexadecimal string
  * Implemented in module [`rgb_hex`](codewars/rgb_hex.py)
* [Simple assembler interpreter](https://www.codewars.com/kata/58e24788e24ddee28e000053)
  * A simple interpreter of assembler which supports:
    `mov x y`, `inc x`, `dec x`, and `jnz x y`
  * Implemented in module [`simple_assembler`](codewars/simple_assembler.py)
* [Valid Parentheses](https://www.codewars.com/kata/52774a314c2333f0a7000688)
  * Checks whether given expression has balanced (matching) parentheses
  * Implemented in module [`parentheses`](codewars/parentheses.py)
* [What's a Perfect Power anyway?](https://www.codewars.com/kata/54d4c8b08776e4ad92000835)
  * Checks whether given integer is a [*perfect power*](https://en.wikipedia.org/wiki/Perfect_power)
  * Iteration, binary search, fast exponentiation
  * Implemented in module [`perfect_power`](codewars/perfect_power.py)
* [Where my anagrams at?](https://www.codewars.com/kata/523a86aa4230ebb5420001e1)
  * Finds all *anagrams* of a word from a list
  * List comprehension and word frequency via `Counter`
  * Implemented in module [`anagrams`](codewars/anagrams.py)

## 6 kyu
* [1RM Calculator](https://www.codewars.com/kata/595bbea8a930ac0b91000130)
  * Calculates projected [*one-repetition maximum (1RM)*](https://en.wikipedia.org/wiki/One-repetition_maximum)
  * Implemented in module [`rm_calculator`](codewars/rm_calculator.py)
* [Build a pile of Cubes](https://www.codewars.com/kata/5592e3bd57b64d00f3000047)
  * Calculates no. cubes in hierarchical structure of given its volume
  * Implemented in module [`cube_pile`](codewars/cube_pile.py)
* [Decode the Morse code](https://www.codewars.com/kata/54b724efac3d5402db00065e)
  * [*Morse code*](https://en.wikipedia.org/wiki/Morse_code) decoder
  * String parsing and manipulation, `join`, `map` and comprehensions
  * Implemented in module [`decode_morse`](codewars/decode_morse.py)
* [Detect Pangram](https://www.codewars.com/kata/545cedaa9943f7fe7b000048)
  * Checks whether given string is a *pangram*
  * Dictionary comprehension and efficient occurrence checking
  * Implemented in module [`pangram`](codewars/pangram.py)
* [Elimination Tournament](https://www.codewars.com/kata/5f631ed489e0e101a70c70a0)
  * Simulates a rank-based elimination tournament
  * List manipulation, comprehension and aggregation
  * Implemented in module [`tourney`](codewars/tourney.py)
* [Format a string of names like 'Bart, Lisa & Maggie'](https://www.codewars.com/kata/53368a47e38700bd8300030d)
  * Simple `List`, `Dict` and `str` manipulation
  * Implemented in module [`name_list`](codewars/name_list.py)
* [Is a number prime?](https://www.codewars.com/kata/5262119038c0985a5b00029f)
  * Checks whether given number is a prime number
  * Implemented in module [`prime`](codewars/prime.py)
* [Meeting](https://www.codewars.com/kata/59df2f8f08c6cec835000012)
  * Sorting and simple string manipulation
  * Implemented in module [`meeting`](codewars/meeting.py)
* [Simple Fun #83: MineSweeper](https://www.codewars.com/kata/58952e29f0902eae8b0000cb)
  * Given a flagged *MineSweeper* board, count the number of neighboring
    mines for each board position
  * Implemented in module [`minesweeper`](codewars/minesweeper.py)
* [Sums of Parts](https://www.codewars.com/kata/5ce399e0047a45001c853c2b)
  * Computes all cumulative sums of given list of numbers
  * Simple enumeration and aggregation
  * Implemented in module [`parts_sums`](codewars/parts_sums.py)
* [Who likes it?](https://www.codewars.com/kata/5266876b8f4bf2da9b000362)
  * Converts list of names to Facebook's "like" text
  * Simple list manipulation, matching cases and string formatting
  * Implemented in module [`likes`](codewars/likes.py)
