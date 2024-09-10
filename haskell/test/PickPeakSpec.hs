module PickPeakSpec (
    spec,
) where

import PickPeak (
    PickedPeaks (..),
    pickPeaks,
 )
import Test.Hspec

spec :: Spec
spec = do
    it "one peak" $ do
        pickPeaks [1, 2, 3, 4, 3, 2, 1]
            `shouldBe` PickedPeaks {pos = [3], peaks = [4]}
    it "few peaks" $ do
        pickPeaks [3, 2, 3, 6, 4, 1, 2, 3, 2, 1, 2, 3]
            `shouldBe` PickedPeaks {pos = [3, 7], peaks = [6, 3]}
    it "many peaks" $
        do
            pickPeaks
                [ 9
                , 9
                , 12
                , -2
                , 11
                , 9
                , 6
                , 14
                , -1
                , -5
                , 12
                , 15
                , 9
                , 7
                , -2
                , -2
                , 10
                , 8
                , -1
                , -1
                , -2
                , 1
                , -1
                , 3
                , 5
                , 14
                , 6
                , 1
                , -2
                , 13
                , 7
                , 5
                , 1
                , 11
                , 15
                , -2
                , -4
                ]
            `shouldBe` PickedPeaks
                { pos = [2, 4, 7, 11, 16, 21, 25, 29, 34]
                , peaks = [12, 11, 14, 15, 10, 1, 14, 13, 15]
                }
    it "plateau" $ do
        pickPeaks [1, 2, 2, 2, 1] `shouldBe` PickedPeaks {pos = [1], peaks = [2]}
    it "no peak" $ do
        pickPeaks [1, 2, 2, 2, 3] `shouldBe` PickedPeaks {pos = [], peaks = []}
        pickPeaks [1, 2, 2, 2, 2] `shouldBe` PickedPeaks {pos = [], peaks = []}
