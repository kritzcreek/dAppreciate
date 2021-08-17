module API (DAppr, Today, Principal, Cycles(..), today) where

import Prelude

import BigInt (BigInt)
import BigInt as BigInt
import Effect.Aff (Aff)
import Effect.Aff.Compat (EffectFnAff, fromEffectFnAff)

type Principal = String

newtype Cycles = Cycles BigInt

foreign import toFixed :: Int -> Number -> String

instance Show Cycles where
  show (Cycles c) = do
    let tc = BigInt.div c (BigInt.fromInt 1_000_000_000)
    toFixed 2 (BigInt.toNumber tc) <> "TC"

type DAppr =
  { canisterId :: Principal
  , name :: String
  }

type Today =
  { dailyBudget :: Cycles
  , balance :: Cycles
  , dApprs :: Array DAppr
  }

foreign import todayImpl :: EffectFnAff Today

today :: Aff Today
today = fromEffectFnAff todayImpl
