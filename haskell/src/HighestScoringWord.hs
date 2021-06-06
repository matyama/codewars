module HighestScoringWord where

import           Data.List                      ( maximumBy )
import           Data.List.Split                ( splitOn )
import           Data.Ord                       ( comparing )

high :: String -> String
high ""  = ""
high str = maximumBy (comparing score) $ reverse words'
  where words' = splitOn " " str

score :: String -> Int
score "" = 0
score w  = sum $ map position w

position :: Char -> Int
position c = fromEnum c - 96
