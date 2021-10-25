{-# LANGUAGE TupleSections #-}

module Huffman
  ( frequencies
  , encode
  , decode
  , Bit(..)
  ) where

import           Data.Map                       ( Map )
import qualified Data.Map                      as M
                                                ( fromListWith
                                                , toList
                                                , fromList
                                                , lookup
                                                )

import           Data.Sequence                  ( Seq(..)
                                                , (|>)
                                                )
import qualified Data.Sequence                 as S
                                                ( fromList
                                                , empty
                                                , sortOn
                                                )

data Bit = Z | O deriving (Eq, Show)

data Tree a = Leaf { weight :: Int, symbol :: a }
            | Node { weight :: Int, left :: Tree a, right :: Tree a }

type Queue a = Seq (Tree a)

-- | Pop head of one of given queues whichever contains lowest-weighted tree.
-- | Returns the minimum tree and updated queue state.
-- | 
-- | Note: By the Huffman tree construction, both queues being empty is an
-- | illegal (and unreachable) state.
popMin :: Queue a -> Queue a -> (Tree a, Queue a, Queue a)
popMin (n :<| nq) Empty      = (n, nq, Empty)
popMin Empty      (l :<| lq) = (l, Empty, lq)
popMin nq@(n :<| nq') lq@(l :<| lq') | weight l <= weight n = (l, nq, lq')
                                     | otherwise            = (n, nq', lq)
popMin _ _ = error "Error: both queues are empty"

-- | Apply single step of the Huffman tree building procedure:
-- |  1. Pop two nodes (or leafs) from the queues with the minimum weights
-- |  2. Construct new node with weights added and the two nodes as sub-trees
-- |  3. Push new node to the back of the node queue
step :: (Queue a, Queue a) -> (Queue a, Queue a)
step (nq, lq) = (nq'' |> n, lq'')
 where
  (l, nq' , lq' ) = popMin nq lq
  (r, nq'', lq'') = popMin nq' lq'
  n               = Node { weight = weight l + weight r, left = l, right = r }

-- | Build the Huffman coding tree given symbol frequencies:
-- |  1. Create a leaf for each symbol and weight in given frequencies
-- |  2. Craate a leaf queue sorted by weight and an initially empty node queue
-- |  3. Repeat `step` until node queue contains single tree
-- |  4. Return final node as the root of the Huffman coding tree
huffmanTree :: [(a, Int)] -> Tree a
huffmanTree = build S.empty . S.sortOn weight . S.fromList . fmap leaf
 where
  leaf (s, w) = Leaf { weight = w, symbol = s }
  build (tree :<| Empty) Empty = tree
  build nq               lq    = let (nq', lq') = step (nq, lq) in build nq' lq'

-- | Calculate symbol frequencies of a text.
frequencies :: Ord a => [a] -> [(a, Int)]
frequencies = M.toList . M.fromListWith (+) . map (, 1)

-- | Encode a sequence using the given frequencies.
encode :: Ord a => [(a, Int)] -> [a] -> Maybe [Bit]
encode []    _       = Nothing
encode [_]   _       = Nothing
encode freqs symbols = concat <$> traverse encodeSymbol symbols
 where
  table = coding $ huffmanTree freqs
  encodeSymbol s = M.lookup s table

-- | Associate each symbol to a coding resulting from given coding tree.
coding :: Ord a => Tree a -> Map a [Bit]
coding t = M.fromList (collect t [] [])
 where
  collect Leaf { symbol = s } bs items = (s, bs) : items
  collect Node { left = l, right = r } bs items =
    collect l (O : bs) (collect r (Z : bs) items)

-- | Decode a bit sequence using the given frequencies.
decode :: Ord a => [(a, Int)] -> [Bit] -> Maybe [a]
decode []    _    = Nothing
decode [_]   _    = Nothing
decode freqs bits = Just (decodeBits bits tree)
 where
  tree = huffmanTree freqs
  decodeBits bs       Leaf { symbol = s } = s : decodeBits bs tree
  decodeBits (O : bs) Node { left = l }   = decodeBits bs l
  decodeBits (Z : bs) Node { right = r }  = decodeBits bs r
  decodeBits []       _                   = []
