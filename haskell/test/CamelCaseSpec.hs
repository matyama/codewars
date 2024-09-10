module CamelCaseSpec (
    spec,
) where

import CamelCase (toCamelCase)
import Test.Hspec

spec :: SpecWith ()
spec = do
    describe "toCamelCase" $ do
        it "should work for some examples" $ do
            toCamelCase "the_stealth_warrior" `shouldBe` "theStealthWarrior"
            toCamelCase "The-Stealth-Warrior" `shouldBe` "TheStealthWarrior"
            toCamelCase "tdzd" `shouldBe` "tdzd"
