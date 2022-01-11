{-# LANGUAGE LambdaCase #-}
module AlgebraicISO where

import           Data.Maybe                     ( isJust )
import           Data.Tuple                     ( swap )
import           Data.Void

import           ISO

{-
   [Algebraic data type](https://en.wikipedia.org/wiki/Algebraic_data_type) got
   the name because it satisfies a lot of algebraic rules under isomorphism.

   Arithmetic of types is based on type's
   [cardinality](https://en.wikipedia.org/wiki/Cardinality):
    - 'Void' does not have any representable value, thus corresponds to 0
    - '()' has single value of the same name, thus corresponds to 1
    - 'Bool' naturally corresponds to 2
    - 'Maybe a' contains the same information as 'a' plus the 'Nothing' case
      and as such corresponds to `1 + a`
    - 'Either a b' combines the information of either 'a' or 'b' which can be
      expressed as `a + b` (note: `Either a () ~= Maybe a`)
    - '(a, b)' combines the information of both 'a' and 'b' which corresponds
      to the (cartesian) product `a * b` 
    - The information expressed by 'a -> b' is best viewed when reading this
      pure function as a table - in order to build it one has to enumerate all
      the `b ^ a` values

   The formal model is called
   [Peano arithmetic](https://en.wikipedia.org/wiki/Peano_axioms).
-}

-- | Multiplication: `a ~= b & c ~= d => a * c ~= b * d`
isoProd :: ISO a b -> ISO c d -> ISO (a, c) (b, d)
isoProd = isoTuple

-- | Addition: `a ~= b & c ~= d => a + c ~= b + d`
isoPlus :: ISO a b -> ISO c d -> ISO (Either a c) (Either b d)
isoPlus = isoEither

-- | `a ~= b => S a ~= S b`
isoS :: ISO a b -> ISO (Maybe a) (Maybe b)
isoS = isoMaybe

-- | Exponentiation: `a ~= b & c ~= d => c ^ a ~= d ^ b`
isoPow :: ISO a b -> ISO c d -> ISO (a -> c) (b -> d)
isoPow = isoFunc

-- | Commutativity of addition: `a + b ~= b + a`
plusComm :: ISO (Either a b) (Either b a)
plusComm =
  ( \case
    Left  a -> Right a
    Right b -> Left b
  , \case
    Left  b -> Right b
    Right a -> Left a
  )

-- | Associativity of addition: `a + b + c ~= a + (b + c)`
plusAssoc :: ISO (Either (Either a b) c) (Either a (Either b c))
plusAssoc =
  ( \case
    Left  (Left  a) -> Left a
    Left  (Right b) -> Right (Left b)
    Right c         -> Right (Right c)
  , \case
    Left  a         -> Left (Left a)
    Right (Left  b) -> Left (Right b)
    Right (Right c) -> Right c
  )

-- | Commutativity of multiplication: `a * b ~= b * a`
multComm :: ISO (a, b) (b, a)
multComm = (swap, swap)

-- | Associativity of multiplication: `a * b * c ~= a * (b * c)`
multAssoc :: ISO ((a, b), c) (a, (b, c))
multAssoc = (\((a, b), c) -> (a, (b, c)), \(a, (b, c)) -> ((a, b), c))

-- | Distrobutivity of multiplication over addition:
--   `a * (b + c) ~= a * b + a * c`
dist :: ISO (a, Either b c) (Either (a, b) (a, c))
dist =
  ( \case
    (a, Left b ) -> Left (a, b)
    (a, Right c) -> Right (a, c)
  , \case
    Left  (a, b) -> (a, Left b)
    Right (a, c) -> (a, Right c)
  )

-- | Exponentiation idientity that defines currying:
--   `(c ^ b) ^ a ~= c ^ (a * b)`
curryISO :: ISO (a -> b -> c) ((a, b) -> c)
curryISO = (uncurry, curry)

-- [Peano arithmetic](https://en.wikipedia.org/wiki/Peano_axioms)

-- | Successor of zero: `1 ~= S O`
one :: ISO () (Maybe Void)
one = (const Nothing, const ())

-- | Successor of one: `2 ~= S (S O)`
two :: ISO Bool (Maybe (Maybe Void))
two =
  ( \b -> if b then Just Nothing else Nothing
  , \case
    Just (Just z) -> absurd z
    mmv           -> isJust mmv
  )

-- | Zero is the additive identity: `O + b ~= b`
plusO :: ISO (Either Void b) b
plusO =
  ( \case
    Left  z -> absurd z
    Right b -> b
  , Right
  )

-- | Propagation of addition into the successor relation:
--   `S a + b ~= S (a + b)` i.e. `(1 + a) + b = 1 + (a + b)`
plusS :: ISO (Either (Maybe a) b) (Maybe (Either a b))
plusS =
  ( \case
    Left  Nothing  -> Nothing
    Left  (Just a) -> Just $ Left a
    Right b        -> Just $ Right b
  , \case
    Nothing        -> Left Nothing
    Just (Left  a) -> Left $ Just a
    Just (Right b) -> Right b
  )

-- | Introduction of successor relation from addition: `1 + b ~= S b`
plusSO :: ISO (Either () b) (Maybe b)
plusSO = isoPlus one refl `trans` plusS `trans` isoS plusO

-- | Zero is the absorbing element of multiplication: `O * a ~= O`
multO :: ISO (Void, a) Void
multO = (fst, absurd)

-- | Propagation of multiplication into the successor relation:
--   `S a * b ~= b + a * b` i.e. `(1 + a) * b = b + a * b`
multS :: ISO (Maybe a, b) (Either b (a, b))
multS =
  ( \case
    (Just a , b) -> Right (a, b)
    (Nothing, b) -> Left b
  , \case
    Left  b      -> (Nothing, b)
    Right (a, b) -> (Just a, b)
  )

-- | One is the multiplicative identity: `1 * b ~= b`
multSO :: ISO ((), b) b
multSO =
  isoProd one refl
    `trans` multS
    `trans` isoPlus refl multO
    `trans` plusComm
    `trans` plusO

-- | Exponentiation to the zero-th power: `a ^ O ~= 1`
powO :: ISO (Void -> a) ()
powO = (const (), const absurd)

-- | Incrementing an exponent corresponds to multiplying by the base:
--   `a ^ (S b) ~= a * (a ^ b)` i.e. `a ^ (1 + b) = a * (a ^ b)`
--
--   For the first 'ISO' component we're given a function `mba :: Maybe b -> a`
--    - 'mba' must produce some const. 'a' from 'Nothing' (i.e. the default)
--    - and the rest (i.e. the 'Just' case) is an ordinary 'b -> a' function
--
--   These two cases correspond to the elements of the tuple `(a, b -> a)`.
--
--   For the reverse direction we just 'uncurry' the built-in 'maybe' fuction:
--    - the 'a' component of the pair becomes the default value for 'maybe'
--    - and together with the 'b -> a' funciton 'maybe' produces 'Maybe b -> a'
powS :: ISO (Maybe b -> a) (a, b -> a)
powS = (\mba -> (mba Nothing, mba . Just), uncurry maybe)

-- | One is the right identity of exponentiation: `a ^ 1 = a`
powSO :: ISO (() -> a) a
powSO =
  isoPow one refl
    `trans` powS
    `trans` multComm
    `trans` isoProd powO refl
    `trans` multSO
