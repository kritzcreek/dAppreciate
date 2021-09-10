module API where

import Prelude

import BigInt (BigInt)
import BigInt as BigInt
import Effect.Aff (Aff)
import Effect.Aff.Compat (EffectFnAff, fromEffectFnAff)

type Principal = String

newtype Cycles = Cycles BigInt

foreign import toFixed :: Int -> Number -> String

billion :: BigInt
billion = BigInt.fromInt 1000000000

instance Show Cycles where
  show (Cycles c) = do
    let billionCycles = BigInt.div c billion
    toFixed 2 (BigInt.toNumber billionCycles / 1000.0) <> "TC"

type DonationReceiver =
  { receiver :: Principal
  , beneficiaries :: Array Principal
  }

type PendingDonation =
  { receiver :: Principal
  , count :: Int
  }

type PendingDonations =
  { pending :: Array PendingDonation
  , amount :: Cycles
  , balance :: Cycles
  }

foreign import listDonationsImpl :: EffectFnAff PendingDonations
foreign import approveDonationsImpl :: EffectFnAff Unit
foreign import setDonationAmountImpl :: Cycles -> (EffectFnAff Unit)

listDonations :: Aff PendingDonations
listDonations = fromEffectFnAff listDonationsImpl

approveDonations :: Aff Unit
approveDonations = fromEffectFnAff approveDonationsImpl

setDonationAmount :: Cycles -> Aff Unit
setDonationAmount cy = fromEffectFnAff (setDonationAmountImpl cy)
