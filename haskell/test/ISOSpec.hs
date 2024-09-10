{-# LANGUAGE LambdaCase #-}

module ISOSpec where

import ISO

import Data.Either
import Test.Hspec
import Test.QuickCheck

bISO :: ISO Bool Bool
bISO = (not, not)

mISO :: ISO (Maybe [()]) (Maybe Int)
mISO =
    ( \case
        Just xs -> if null xs then Nothing else Just (length xs)
        Nothing -> Just 0
    , \case
        Just 0 -> Nothing
        Just n -> Just (replicate n ())
        Nothing -> Just []
    )

lrl :: ISO a b -> (a -> a)
lrl (ab, ba) = ba . ab

spec :: Spec
spec = do
    describe "subst" $ do
        it "substL" $ do
            substL bISO True `shouldBe` False
            substL bISO False `shouldBe` True
            substL isoBool False `shouldBe` False
            substL isoBool True `shouldBe` True
        it "substR" $ do
            substR bISO True `shouldBe` False
            substR bISO False `shouldBe` True
            substR isoBool True `shouldBe` True
            substR isoBool False `shouldBe` False
        it "lrl isoEU (Left (replicate n ())) == Left (replicate n ())" $
            property $
                \(NonNegative n) ->
                    lrl mISO (Just (replicate n ())) == Just (replicate n ())
        it "isoEU" $ do
            isLeft (substL isoEU (Right ())) `shouldBe` True
        it "lrl isoEU (Left (replicate n ())) == Left (replicate n ())" $
            property $
                \(NonNegative n) ->
                    lrl isoEU (Left (replicate n ())) == Left (replicate n ())
