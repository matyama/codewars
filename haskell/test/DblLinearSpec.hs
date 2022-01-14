module DblLinearSpec where

import           DblLinear                      ( dblLinear )

import           Test.Hspec
import           Text.Printf                    ( printf )

testDblLinear :: Int -> Integer -> Spec
testDblLinear n u =
  it (printf "should return dblLinear for n: %d " n) $ dblLinear n `shouldBe` u

spec :: Spec
spec = do
  describe "dblLinear" $ do
    testDblLinear 10 22
    testDblLinear 20 57
    testDblLinear 30 91
    testDblLinear 50 175
