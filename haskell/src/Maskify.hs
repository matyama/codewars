module Maskify where

maskify :: String -> String
maskify str = hide str ""

hide :: String -> String -> String
hide [] result = result
hide str@(_ : rest) result | length str <= 4 = result ++ str
                           | otherwise       = hide rest (result ++ "#")
