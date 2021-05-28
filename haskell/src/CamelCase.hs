module CamelCase where

import Data.Char
import Data.List
import Data.List.Split 

capitalize :: String -> String
capitalize [] = []
capitalize (h:t) = (toUpper h) : t

delimiter :: String -> Maybe String
delimiter [] = Nothing 
delimiter ('-':_) = Just "-"
delimiter ('_':_) = Just "_"
delimiter (_:s) = delimiter s

toWords :: String -> Maybe String -> [String]
toWords str Nothing = [str]
toWords str (Just d) = splitOn d str

toCamelCase :: String -> String
toCamelCase str = firstWord ++ (concatMap capitalize otherWords)
  where (firstWord:otherWords) = toWords str (delimiter str)
