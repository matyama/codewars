module PickPeak where

import           Data.Monoid
import           Data.Semigroup

data PickedPeaks = PickedPeaks
  { pos   :: [Int]
  , peaks :: [Int]
  }
  deriving (Eq, Show)

instance Semigroup PickedPeaks where
  PickedPeaks { pos = posA, peaks = peaksA } <> PickedPeaks { pos = posB, peaks = peaksB }
    = PickedPeaks { pos = posA ++ posB, peaks = peaksA ++ peaksB }

instance Monoid PickedPeaks where
  mempty = PickedPeaks { pos = [], peaks = [] }

pickPeaks :: [Int] -> PickedPeaks
pickPeaks values = mconcat $ findPeaks values 0 0

findPeaks :: [Int] -> Int -> Int -> [PickedPeaks]
findPeaks []           _ _ = []
findPeaks (_     : []) _ _ = []
findPeaks (_ : _ : []) _ _ = []
findPeaks (left : mid : right : []) x i
  | left == mid  = []
  | mid == right = []
  | otherwise    = [peak (x + 1) left mid right]
findPeaks (left : mid : right : tail) x i
  | left == mid = findPeaks (mid : right : tail) x (i + 1)
  | mid == right = findPeaks (left : right : tail) x (i + 1)
  | otherwise = (peak (x + 1) left mid right)
  : findPeaks (mid : right : tail) (i + 1) (i + 1)

peak :: Int -> Int -> Int -> Int -> PickedPeaks
peak x fl fx fr | fx >= fl && fx >= fr = PickedPeaks { pos = [x], peaks = [fx] }
                | otherwise            = mempty
