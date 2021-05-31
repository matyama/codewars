module Dups where

import           Data.Char                      ( toLower )

-- TODO: memoization
-- https://kseo.github.io/posts/2017-01-14-memoization-in-hasekll.html
-- https://wiki.haskell.org/Memoization
-- https://wiki.haskell.org/Let_vs._Where
duplicateEncode :: String -> String
duplicateEncode str = [ if count c == 1 then '(' else ')' | c <- text ]
 where
  text = map toLower str
  count c = length $ filter (== c) text
