module PickPeakSpec
  ( spec
  ) where

import           PickPeak                       ( PickedPeaks(..)
                                                , pickPeaks
                                                )
import           Test.Hspec

spec :: Spec
spec = do
  it "[3, 2, 3, 6, 4, 1, 2, 3, 2, 1, 2, 3]" $ do
    pickPeaks [3, 2, 3, 6, 4, 1, 2, 3, 2, 1, 2, 3]
      `shouldBe` PickedPeaks { pos = [3, 7], peaks = [6, 3] }
  it "plateau" $ do
    pickPeaks [1, 2, 2, 2, 1] `shouldBe` PickedPeaks { pos = [1], peaks = [2] }
  it "no peak" $ do
    pickPeaks [1, 2, 2, 2, 3] `shouldBe` PickedPeaks { pos = [], peaks = [] }
    pickPeaks [1, 2, 2, 2, 2] `shouldBe` PickedPeaks { pos = [], peaks = [] }
