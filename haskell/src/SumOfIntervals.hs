module SumOfIntervals (
    sumOfIntervals,
) where

import Data.List (sortOn)
import Data.Ord

sumOfIntervals :: [(Int, Int)] -> Int
sumOfIntervals = fst . foldr addInterval (0, Nothing) . sortOn (Down . fst)
  where
    addInterval (a, b) (s, Nothing) = (s + (b - a), Just b)
    addInterval (a, b) (s, Just end) =
        let s' = s + max 0 (b - max a end)
            end' = max b end
         in (s', Just end')
