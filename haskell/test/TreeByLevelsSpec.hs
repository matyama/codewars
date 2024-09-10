module TreeByLevelsSpec where

import Test.Hspec

import TreeByLevels (
    TreeNode (..),
    treeByLevels,
 )

buildTree :: [a] -> Maybe (TreeNode a)
buildTree l = fst $ walk $ split 1 l
  where
    split _ [] = []
    split n x = h : split (2 * n) t where (h, t) = splitAt n x
    walk [] = (Nothing, [])
    walk ls@([] : _) = (Nothing, ls)
    walk ((h : t) : ls) = (Just $ TreeNode lt rt h, t : ls'')
      where
        (lt, ls') = walk ls
        (rt, ls'') = walk ls'

testBuildTree :: (Show a, Eq a) => [a] -> Expectation
testBuildTree x = treeByLevels (buildTree x) `shouldBe` x

spec :: Spec
spec = do
    describe "treeByLevels on static trees" $ do
        it "returns [] given Nothing" $ treeByLevels Nothing `shouldBe` ([] :: [()])
        it "handles tree with one element" $
            treeByLevels (Just $ TreeNode Nothing Nothing 1)
                `shouldBe` [1]
        it "sorts example tree 1" $ testBuildTree [2, 8, 9, 1, 3, 4, 5]
