module Main where

import Prelude

import API as API
import Data.Maybe (Maybe(..), maybe)
import Effect (Effect)
import Effect.Aff (Aff)
import Halogen (liftAff)
import Halogen as H
import Halogen.Aff as HA
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Halogen.VDom.Driver (runUI)

main :: Effect Unit
main = HA.runHalogenAff do
  body <- HA.awaitBody
  runUI component unit body

type State =
  { pending :: Maybe API.PendingDonations
  }

data Action = Initialize | ApproveDonations

renderHeader :: forall act slots. State -> HH.HTML act slots
renderHeader { pending } = HH.header_
  [ HH.text "dApprecation Client"
  , HH.section
      [ HP.id "budget" ]
      [ budget "Daily Budget" (maybe "Loading" (show <<< _.amount) pending)
      , budget "Balance" (maybe "Loading" (show <<< _.balance) pending)
      ]
  ]
  where
  budget label cycles = HH.div
    [ HP.class_ (HH.ClassName "budget") ]
    [ HH.p
        [ HP.class_ (HH.ClassName "cycles") ]
        [ HH.text cycles ]
    , HH.text label
    ]

renderMain :: forall act slots. State -> HH.HTML act slots
renderMain state = HH.main_
  [ HH.section
      [ HP.id "todaysdApprs" ]
      [ HH.h2_ [ HH.text "Todays dApprs" ]
      , HH.div
          [ HP.id "dApprs" ]
          (map renderDAppr (maybe [] _.pending state.pending))
      ]
  ]

renderFooter :: forall slots. State -> HH.HTML slots Action
renderFooter _ = HH.footer_
  [ HH.button [ HP.id "menuApprove", HE.onClick \_ -> ApproveDonations ] [ HH.text "Approve Donations" ] ]

renderDAppr :: forall act slots. API.PendingDonation -> HH.HTML act slots
renderDAppr pending = HH.div
  [ HP.class_ (HH.ClassName "dAppr") ]
  [ HH.text pending.receiver ]

component :: forall q i o. H.Component q i o Aff
component =
  H.mkComponent
    { initialState
    , render
    , eval: H.mkEval $ H.defaultEval { handleAction = handleAction, initialize = Just Initialize }
    }
  where
  initialState _ = { pending: Nothing }

  render state = HH.div [ HP.id "app" ]
    [ renderHeader state
    , renderMain state
    , renderFooter state
    ]

  handleAction = case _ of
    Initialize -> void $ H.fork do
      pending <- liftAff API.listDonations
      H.put { pending: Just pending }
    ApproveDonations -> do
      liftAff API.approveDonations
