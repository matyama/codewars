module Josephus where

josephus :: [a] -> Int -> [a]
josephus xs k = reverse $ generatePerm xs [] 0
 where
  generatePerm []   p _ = p
  generatePerm rest p i = generatePerm (left ++ right) (removed : p) pos
   where
    pos                     = (i + k - 1) `mod` length rest
    (left, removed : right) = splitAt pos rest
