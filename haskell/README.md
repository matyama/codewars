# Haskell Kata

## 2 kyu
* [Algebraic Isomorphism](https://www.codewars.com/kata/5917f22dd2563a36a200009c)
  * Implements bunch of algebraic rules which give the name to
    [*Algebraic Data Types*](https://en.wikipedia.org/wiki/Algebraic_data_type)
  * Algebraic rules are satisfied by ADTs under *isomorphism*
  * This is a follow-up to the *Isomorphism* kata and is implemented in
    module [`AlgebraicISO`](src/AlgebraicISO.hs)

## 3 kyu
* [Huffman Encoding](https://www.codewars.com/kata/54cf7f926b85dcc4e2000d9d)
  * Implements[*Huffman coding*](https://en.wikipedia.org/wiki/Huffman_coding)
  * Implemented in module [`Huffman`](src/Huffman.hs)
* [Isomorphism](https://www.codewars.com/kata/5922543bf9c15705d0000020)
  * Definition of an [*isomorphism*](https://en.wikipedia.org/wiki/Isomorphism)
  * Validation of properties of isomorphisms: equivalence relation,
    lifting to a `Functor` and `Bifunctor`, lowering isomorphisms over
    `Maybe` and evidence of the impossibility to lower from an `Either`
  * Implemented in module [`ISO`](src/ISO.hs)

## 4 kyu
* [Sort binary tree by levels](https://www.codewars.com/kata/52bef5e3588c56132c0003bc)
  * Traverses given tree in BFS order yielding its values
  * Implements [*Breadth-first search*](https://en.wikipedia.org/wiki/Breadth-first_search)
  * Implemented in module [`TreeByLevels`](src/TreeByLevels.hs)
* [Sum of Intervals](https://www.codewars.com/kata/52b7ed099cdc285c300001cd)
  * Compute the total length of `n` overlapping intervals in
    `O(n*log(n))` time
  * Implemented in module [`SumOfIntervals`](src/SumOfIntervals.hs)
* [Twice linear](https://www.codewars.com/kata/5672682212c8ecf83e000050)
  * Finds the nth item of an ordered set that is closed on two linear
    transformations of its elements in `O(n)` time
  * Implemented in module [`DblLinear`](src/DblLinear.hs)

## 5 kyu
* [Don't Drink the Water](https://www.codewars.com/kata/562e6df5cf2d3908ad00019e)
  * 2D list manipulation, `chunksOf`, `foldr` and pattern matching
  * Implemented in module [`DontDrinkTheWater`](src/DontDrinkTheWater.hs)
* [Josephus Permutation](https://www.codewars.com/kata/5550d638a99ddb113e0000a2)
  * Implements a function that returns a [*Josephus permutation*](https://en.wikipedia.org/wiki/Josephus_problem)
  * Implemented in module [`Josephus`](src/Josephus.hs)
* [Pick peaks](https://www.codewars.com/kata/5279f6fe5ab7f447890006a7)
  * Finds positions and values of *local maxima* in a numeric array
  * Data declaration, `deriving`, `Monoid` instance and `mconcat`
  * Implemented in module [`PickPeak`](src/PickPeak.hs)

## 6 kyu
* [Consecutive strings](https://www.codewars.com/kata/56a5d994ac971f1ac500003e)
  * Windowing, pattern guards, `maximumBy` and string manipulation
  * Implemented in module [`Longestconsec`](src/Longestconsec.hs)
* [Convert string to camel case](https://www.codewars.com/kata/517abf86da9663f1d2000003)
  * Converts a string to *Upper Camel Case* (a.k.a *Pascal case*)
  * Simple recursion, pattern matching and string manipulation
  * Implemented in module [`CamelCase`](src/CamelCase.hs)
* [Does my number look big in this?](https://www.codewars.com/kata/5287e858c6b5a9678200083c)
  * Checks whether a given number is [*narcissistic*](https://en.wikipedia.org/wiki/Narcissistic_number)
  * Implemented in module [`Narcissistic`](src/Narcissistic.hs)
* [Duplicate Encoder](https://www.codewars.com/kata/54b42f9314d9229fd6000d9c)
  * Encodes characters of a string depending on their duplicity
  * Filtering with built-in function memoization
  * Implemented in module [`Dups`](src/Dups.hs)
* [Highest Scoring Word](https://www.codewars.com/kata/57eb8fcdf670e99d9b000272)
  * Finds highest scoring word based on simple char-based scoring function
  * `maximumBy`, `comparing` and `splitOn`
  * Implemented in module [`HighestScoringWord`](src/HighestScoringWord.hs)

## 7 kyu
* [Credit Card Mask](https://www.codewars.com/kata/5412509bd436bd33920011bc)
  * Masks all but the last four characters of a string
  * String matching, concatenation and pattern guards
	* Implemented in module [`Maskify`](src/Maskify.hs)
* [You're a square!](https://www.codewars.com/kata/54c27a33fb7da0db0100040e)
  * Given an integral number, determines if it's a [*square number*](https://en.wikipedia.org/wiki/Square_number)
  * Function composition, list comprehension and filtering
  * Implemented in module [`Square`](src/Square.hs)
