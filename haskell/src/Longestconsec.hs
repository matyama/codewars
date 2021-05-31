module Longestconsec where

import           Data.List                      ( maximumBy
                                                , tails
                                                )
import           Data.Ord                       ( comparing )

longestConsec :: [String] -> Int -> String
longestConsec strarr k
  | k <= 0 || k > length strarr = ""
  | otherwise                   = maximumBy (comparing length) $ reverse consecs
  where consecs = map concat (consecutiveStrings strarr k)

consecutiveStrings :: [String] -> Int -> [[String]]
consecutiveStrings list@(_ : tail) k | length list == k = [window]
                                     | otherwise        = window : rest
 where
  window = take k list
  rest   = consecutiveStrings tail k
consecutiveStrings _ _ = []
