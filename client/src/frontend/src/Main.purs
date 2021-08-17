module Main where

import Prelude

import API as API
import Data.Maybe (Maybe(..), fromMaybe, maybe)
import Effect (Effect)
import Effect.Aff (Aff)
import Halogen (liftAff)
import Halogen as H
import Halogen.Aff as HA
import Halogen.HTML as HH
import Halogen.HTML.Properties as HP
import Halogen.VDom.Driver (runUI)

main :: Effect Unit
main = HA.runHalogenAff do
  body <- HA.awaitBody
  runUI component unit body

type State =
  { today :: Maybe API.Today
  }

data Action = Initialize

renderHeader :: forall act slots. State -> HH.HTML act slots
renderHeader { today } = HH.header_
  [ HH.text "dApprecation Client"
  , HH.section
      [ HP.id "budget" ]
      [ budget "Daily Budget" (maybe "Loading" (show <<< _.dailyBudget) today)
      , budget "Balance" (maybe "Loading" (show <<< _.balance) today)
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
          (map renderDAppr (maybe [] _.dApprs state.today))
      ]
  ]

renderFooter :: forall act slots. State -> HH.HTML act slots
renderFooter _ = HH.footer_
  [ HH.button [ HP.id "menuToday" ] [ HH.text "Today" ]
  , HH.button [ HP.id "menuHistory" ] [ HH.text "History" ]
  , HH.button [ HP.id "menuSettings" ] [ HH.text "Settings" ]
  ]

renderDAppr :: forall act slots. API.DAppr -> HH.HTML act slots
renderDAppr dAppr = HH.div
  [ HP.class_ (HH.ClassName "dAppr") ]
  [ HH.text dAppr.name ]

component :: forall q i o. H.Component q i o Aff
component =
  H.mkComponent
    { initialState
    , render
    , eval: H.mkEval $ H.defaultEval { handleAction = handleAction, initialize = Just Initialize }
    }
  where
  initialState _ = { today: Nothing }

  render state = HH.div [ HP.id "app" ]
    [ renderHeader state
    , renderMain state
    , renderFooter state
    ]

  handleAction = case _ of
    Initialize -> void $ H.fork do
      tdy <- liftAff API.today
      H.put { today: Just tdy }
