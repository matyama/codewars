module DblLinear where

import Data.Sequence (
    Seq (..),
    (|>),
 )

type Queue = Seq Integer

dblLinear :: Int -> Integer
dblLinear = step 1 Empty Empty

step :: Integer -> Queue -> Queue -> Int -> Integer
step x _ _ 0 = x
step x ys zs n = step x' ys' zs' (n - 1)
  where
    y = 2 * x + 1
    z = 3 * x + 1
    (x', ys', zs') = update (ys |> y) (zs |> z)

update :: Queue -> Queue -> (Integer, Queue, Queue)
update ys@(y :<| ys') zs@(z :<| zs') = case compare y z of
    LT -> (y, ys', zs)
    EQ -> (y, ys', zs')
    GT -> (z, ys, zs')
update _ _ = error "empty queue"
