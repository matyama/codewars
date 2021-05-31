module Square where

isSquare :: Integral n => n -> Bool
isSquare n = not $ null [ x | x <- [0 .. m], x ^ 2 == n ]
  where m = floor $ sqrt $ fromIntegral n
