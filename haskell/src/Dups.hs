module Dups where

import Data.Char (toLower)

duplicateEncode :: String -> String
duplicateEncode text = [if occurences c == 1 then '(' else ')' | c <- codes]
  where
    codes = map (fromEnum . toLower) text
    occurences = (map count [0 ..] !!)
      where
        count c = length $ filter (== c) codes
