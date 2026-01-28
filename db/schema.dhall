-- db/schema.dhall
let CIDR = Text

let Country =
      { name   : Text
      , iso2   : Text
      , cidrs4 : List CIDR
      , cidrs6 : List CIDR
      }

let Continent =
      { name      : Text
      , countries : List Country
      }

in  { version     : Natural
    , generatedAt : Text
    , continents  : List Continent
    }

