# Haskell Kata

## 3 kyu
* [Huffman Encoding](https://www.codewars.com/kata/54cf7f926b85dcc4e2000d9d)
  * Implements[*Huffman coding*](https://en.wikipedia.org/wiki/Huffman_coding)
  * Implemented in module [`Huffman`](src/Huffman.hs)

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
