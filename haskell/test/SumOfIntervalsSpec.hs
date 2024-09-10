module SumOfIntervalsSpec (
    spec,
) where

import SumOfIntervals (sumOfIntervals)
import Test.Hspec

spec :: Spec
spec = do
    it "Example tests" $ do
        sumOfIntervals [(1, 5)] `shouldBe` 4
        sumOfIntervals [(1, 5), (10, 15), (-1, 3)] `shouldBe` 11
        sumOfIntervals [(1, 5), (6, 10)] `shouldBe` 8
        sumOfIntervals [(1, 5), (1, 5)] `shouldBe` 4
        sumOfIntervals [(1, 4), (7, 10), (3, 5)] `shouldBe` 7

    it "Large intervals" $ do
        sumOfIntervals [(-1000000000, 1000000000)] `shouldBe` 2000000000
        sumOfIntervals [(0, 20), (-100000000, 10), (30, 40)] `shouldBe` 100000030
