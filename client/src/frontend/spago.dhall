{ name = "client"
, dependencies =
  [ "aff"
  , "console"
  , "debug"
  , "effect"
  , "halogen"
  , "maybe"
  , "prelude"
  , "psci-support"
  ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs" ]
}
