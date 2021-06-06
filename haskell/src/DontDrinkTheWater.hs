module DontDrinkTheWater
  ( separateLiquids
  ) where


import           Data.List.Split                ( chunksOf )

separateLiquids :: [[Char]] -> [[Char]]
separateLiquids []    = []
separateLiquids glass = chunksOf width $ sortLiquids glass
  where width = length $ head glass

sortLiquids :: [[Char]] -> [Char]
sortLiquids = concat . foldr add [[], [], [], []] . concat
 where
  add 'O' [o, a, w, h] = ['O' : o, a, w, h]
  add 'A' [o, a, w, h] = [o, 'A' : a, w, h]
  add 'W' [o, a, w, h] = [o, a, 'W' : w, h]
  add 'H' [o, a, w, h] = [o, a, w, 'H' : h]
  add _   liquids      = liquids
