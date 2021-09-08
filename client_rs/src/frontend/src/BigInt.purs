module BigInt (BigInt, mod, div, fromInt, fromString, toString, toNumber) where

import Prelude

import Data.Maybe (Maybe(..))

foreign import data BigInt :: Type

foreign import _add :: BigInt -> BigInt -> BigInt
foreign import _sub :: BigInt -> BigInt -> BigInt
foreign import _mul :: BigInt -> BigInt -> BigInt
foreign import div :: BigInt -> BigInt -> BigInt
foreign import _eq :: BigInt -> BigInt -> Boolean
foreign import _compare :: BigInt -> BigInt -> Int
foreign import toString :: Int -> BigInt -> String
foreign import _fromString
  :: forall a
   . (a -> Maybe a)
  -> Maybe a
  -> String
  -> Maybe BigInt
foreign import mod :: BigInt -> BigInt -> BigInt
foreign import fromInt :: Int -> BigInt
foreign import toNumber :: BigInt -> Number

fromString :: String -> Maybe BigInt
fromString = _fromString Just Nothing

instance Eq BigInt where
  eq = _eq

instance Ord BigInt where
  compare n1 n2 = case _compare n1 n2 of
    1 -> GT
    0 -> EQ
    _ -> LT

instance Show BigInt where
  show = toString 10

instance Semiring BigInt where
  zero = fromInt 0
  one = fromInt 1
  add = _add
  mul = _mul

instance Ring BigInt where
  sub = _sub
