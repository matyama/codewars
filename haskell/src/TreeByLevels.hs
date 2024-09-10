module TreeByLevels where

import Data.Sequence (
    Seq (..),
    singleton,
    (|>),
 )

data TreeNode a = TreeNode
    { left :: Maybe (TreeNode a)
    , right :: Maybe (TreeNode a)
    , value :: a
    }
    deriving (Show)

-- | Traverses given tree in BFS order yielding its values
treeByLevels :: Maybe (TreeNode a) -> [a]
treeByLevels Nothing = []
treeByLevels (Just root) = bfs $ singleton root
  where
    bfs Empty = []
    bfs (n :<| ns) =
        let ns' = maybe ns (ns |>) $ left n
            ns'' = maybe ns' (ns' |>) $ right n
         in value n : bfs ns''
