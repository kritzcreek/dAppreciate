module API where

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

type DonationReceiver =
  { receiver :: Principal
  , beneficiaries :: Array Principal
  }

type PendingDonation =
  { receiver :: Principal
  , count :: BigInt
  }

type PendingDonations =
  { pending :: Array PendingDonation
  , amount :: Cycles
  , balance :: Cycles
  }

foreign import listDonationsImpl :: EffectFnAff PendingDonations

listDonations :: Aff PendingDonations
listDonations = fromEffectFnAff listDonationsImpl
