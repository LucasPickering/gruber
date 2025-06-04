module Main exposing (..)

import Browser
import Css
import Html exposing (Html)
import Html.Styled exposing (button, div, main_, span, text, toUnstyled)
import Html.Styled.Attributes exposing (css)
import Html.Styled.Events exposing (onClick)


main : Program () Model Msg
main =
    Browser.sandbox
        { init = init
        , update = update
        , view = view
        }


type alias Model =
    { counter : Int }


init : Model
init =
    { counter = 0 }


type Msg
    = Increment
    | Decrement


update : Msg -> Model -> Model
update msg model =
    case msg of
        Increment ->
            { model | counter = model.counter + 1 }

        Decrement ->
            { model | counter = model.counter - 1 }


view : Model -> Html Msg
view model =
    let
        buttonStyle =
            css
                []
    in
    toUnstyled <|
        main_ [ css [] ]
            [ div
                [ css [] ]
                [ button
                    [ buttonStyle, onClick Decrement ]
                    [ text "---" ]
                , div
                    [ css [] ]
                    [ span
                        [ css [] ]
                        [ text (String.fromInt model.counter) ]
                    ]
                , button
                    [ buttonStyle, onClick Increment ]
                    [ text "+++" ]
                ]
            ]
