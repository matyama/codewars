module Narcissistic where

narcissistic :: Integral n => n -> Bool
narcissistic n = value n == n

-- Value of the narcissistic function with base 10
value :: Integral n => n -> n
value n = sum $ map (^ k) ds
  where
    ds = digits n
    k = length ds

-- Note: In this context it is fine to return no digits for 0
digits :: Integral n => n -> [n]
digits 0 = []
digits n = digits (n `div` 10) ++ [n `mod` 10]
