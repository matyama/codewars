module DontDrinkTheWaterSpec (
    spec,
) where

import Control.Monad
import Data.List (intercalate)
import Test.Hspec

import DontDrinkTheWater (separateLiquids)

fixedTests :: [([Char], [[Char]], [[Char]])]
fixedTests =
    [
        ( "Should be able to sort 3 liquids"
        , ["HHWO", "WWOW", "HHOO"]
        , ["OOOO", "WWWW", "HHHH"]
        )
    ,
        ( "Should be able to handle 4 liquids"
        , ["AAOH", "AHWO", "WWAW", "HHOO"]
        , ["OOOO", "AAAA", "WWWW", "HHHH"]
        )
    , ("Should be able to handle one row", ["AHWO"], ["OAWH"])
    ,
        ( "Should be able to handle one column"
        , ["A", "H", "W", "O"]
        , ["O", "A", "W", "H"]
        )
    , ("Should be able to handle empty array", [], [])
    ]

spec :: Spec
spec = do
    describe "Sample tests" $ do
        forM_ fixedTests $ \(name, input, expected) -> do
            it (name <> "\nseparateLiquids " <> show input) $ do
                Cook (separateLiquids input) `shouldBe` Cook expected

newtype Cook
    = Cook [[Char]]
    deriving (Eq)
instance Show Cook where
    show (Cook s) = "\n[ " <> intercalate "\n, " (show <$> s) <> "\n]\n"
