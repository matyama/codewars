module DupsSpec
  ( spec
  ) where

import           Dups                           ( duplicateEncode )
import           Test.Hspec

spec :: Spec
spec = do
  it "encodes the codes" $ do
    duplicateEncode "din" `shouldBe` "((("
    duplicateEncode "recede" `shouldBe` "()()()"
    duplicateEncode "Success" `shouldBe` ")())())"
    duplicateEncode "CodeWarrior" `shouldBe` "()(((())())"
    duplicateEncode "Supralapsarian" `shouldBe` ")()))()))))()("
    duplicateEncode "iiiiii" `shouldBe` "))))))"
