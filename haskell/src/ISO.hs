{-# LANGUAGE LambdaCase #-}
module ISO where

import           Data.Maybe                     ( fromJust
                                                , fromMaybe
                                                )
import           Data.Bifunctor                 ( bimap
                                                , Bifunctor
                                                )
import           Data.Tuple                     ( swap )
import           Data.Void

-- | Definition of an isomorphism as a pair of `(f, f')` where
--   `f . f' = id = f' . f`
type ISO a b = (a -> b, b -> a)

-- | Left projection on an 'ISO a b' providing the function `a -> b`
substL :: ISO a b -> (a -> b)
substL = fst

-- | Right projection on an 'ISO a b' providing the function `b -> a`
substR :: ISO a b -> (b -> a)
substR = snd

-- | Example of an isomorphism on boolean functions.
isoBool :: ISO Bool Bool
isoBool = (id, id)

-- | Evidence that there can be more than one isomorphism.
--   
--   Here shown on two isomorphisms for functions `Bool -> Bool`:
--    1. defined by 'isoBool'
--    2. defined here by 'isoBoolNot'
isoBoolNot :: ISO Bool Bool
isoBoolNot = (not, not)

-- | Isomorphism is reflexive
refl :: ISO a a
refl = (id, id)

-- | Isomorphism is symmetric: `symm . symm = id`
--  
--   Note: `symm :: (a -> b, b -> a) -> (b -> a, a -> b)`
symm :: ISO a b -> ISO b a
symm = swap

-- | Isomorphism is transitive
--  
--   ```haskell
--   trans :: (a -> b, b -> a) -> (b -> c, c -> b) -> (a -> c, c -> a)
--   ```
trans :: ISO a b -> ISO b c -> ISO a c
trans (ab, ba) (bc, cb) = (bc . ab, ba . cb)

-- | Isomorphism lifted to an arbitrary 'Functor':
--   ```haskell
--   isoFunctor :: Functor f => (a -> b, b -> a) -> (f a -> f b, f b -> f a)`
--   ```
isoFunctor :: Functor f => ISO a b -> ISO (f a) (f b)
isoFunctor = bimap (<$>) (<$>)

-- | Isomorphism lifted to a 'List':
--   `isoList :: (a -> b, b -> a) -> ([a] -> [b], [b] -> [a])`
isoList :: ISO a b -> ISO [a] [b]
isoList = isoFunctor

-- | Isomorphism lifted to a 'Maybe':
--   `isoList :: (a -> b, b -> a) -> ([a] -> [b], [b] -> [a])`
isoMaybe :: ISO a b -> ISO (Maybe a) (Maybe b)
isoMaybe = isoFunctor

-- | Two isomorphisms lifted to an 'ISO' over an arbitrary 'Bifunctor':
--   ```haskell
--   isoBifunctor
--     :: (Bifunctor f)
--     => (a -> b, b -> a)
--     -> (c -> d, d -> c)
--     -> (f a c -> f b d, f b d -> f a c)
--   ```
isoBifunctor :: (Bifunctor f) => ISO a b -> ISO c d -> ISO (f a c) (f b d)
isoBifunctor (ab, ba) (cd, dc) = (bimap ab cd, bimap ba dc)

-- | Product of two isomorphisms:
--   ```haskell
--   isoTuple
--     :: (a -> b, b -> a)
--     -> (c -> d, d -> c)
--     -> ((a, c) -> (b, d), (b, d) -> (a, c))
--   ```
isoTuple :: ISO a b -> ISO c d -> ISO (a, c) (b, d)
isoTuple = isoBifunctor

-- | Two isomorphisms lifted to an 'ISO' of 'Either':
--   ```haskell
--   isoEither
--     :: (a -> b, b -> a)
--     -> (c -> d, d -> c)
--     -> (Either a c -> Either b d, Either b d -> Either a c)
--   ```
isoEither :: ISO a b -> ISO c d -> ISO (Either a c) (Either b d)
isoEither = isoBifunctor

-- | Function isomorphism:
--   ```haskell
--   isoFunc
--    :: (a -> b, b -> a)
--    -> (c -> d, d -> c)
--    -> (a -> c -> b -> d, b -> d -> a -> c)
--   ```
isoFunc :: ISO a b -> ISO c d -> ISO (a -> c) (b -> d)
isoFunc (ab, ba) (cd, dc) = (\ac -> cd . ac . ba, \bd -> dc . bd . ab)

-- | Unwrapping an isomorphism on effects (here 'Maybe') is hard and generally
--   impossible.
--
--   Fortunately, 'Maybe' is simple enough and the construction of an 'ISO a b'
--   works out because 'Just' is the only constructor that holds a value.
--
--   WLOG: Given an 'a' value, to get a 'Maybe b' one can first wrap it into a
--   'Just' and then use the 'substL' morphism of the 'ISO'.
--
--   Then to get from 'Maybe b' to 'b':
--    1. 'Just b' can be safely unwrapped into 'b'
--    2. There is just single possible option how to get a 'b' out of 'Nothing'
--       and that is to use the other 'ISO' projection ('substR') and make a
--       'b' value 'fromJust'
--   
--   Because 'substL' and 'substR' are inverses of each other, the 'fromJust'
--   can be justified as the only implementation option.
--
--   ```haskell
--   isoUnMaybe :: (Maybe a -> Maybe b, Maybe b -> Maybe a) -> (a -> b, b -> a)
--   ```
isoUnMaybe :: ISO (Maybe a) (Maybe b) -> ISO a b
isoUnMaybe (mab, mba) =
  ( fromMaybe (fromJust $ mab Nothing) . mab . Just
  , fromMaybe (fromJust $ mba Nothing) . mba . Just
  )

-- | Evidence that there cannot be an isomorphism:
--   ```haskell
--   isoUnEither :: ISO (Either a b) (Either c d) -> ISO a c -> ISO b d
--   ```
--   If there was, then one could get 'Void'
--    - first by getting an `(isoUnEither isoEU) :: ISO () Void`
--    - and then taking 'substL' to pick the 'Void' from it
--   
--   This would be 'absurd'!
--
--   The trick behid the 'isoEU' is the following encoding:
--    - 'Right' variant of the first 'Either' is encoded to 'Left []'
--    - 'Left' on the other hand increments (resp. decrements) its length
--   
--   I.e. the information about the side is encoded into the length of the
--   (infinite) list in the 'Left' side of the latter 'Either'.
isoEU :: ISO (Either [()] ()) (Either [()] Void)
isoEU =
  ( \case
    Left  x -> Left (() : x)
    Right _ -> Left []
  , \case
    Left  []       -> Right ()
    Left  (_ : xs) -> Left xs
    Right x        -> absurd x
  )

-- | Symmetry of two isomorphisms lifted to an 'ISO':
--   ```haskell
--   isoSymm
--     :: ( (a -> b, b -> a) -> (b -> a, a -> b)
--        , (b -> a, a -> b) -> (a -> b, b -> a)
--        )
--   ```
isoSymm :: ISO (ISO a b) (ISO b a)
isoSymm = (symm, symm)
