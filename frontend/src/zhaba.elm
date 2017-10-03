module Zhaba exposing (..)

import Html exposing (..)
import Html.Attributes exposing (style)
import Html.Events exposing (..)
import Register exposing (..)
import Form exposing (Form)

main = Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type Msg = Nop | RegisterFormMsg Form.Msg

type alias Model =
    { registerForm : Register.Model
    }

init : (Model, Cmd Msg)
init = ({
        registerForm = Register.init
    }, Cmd.none)

view : Model -> Html Msg
view model =  Html.map RegisterFormMsg (Register.view model.registerForm)

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Nop -> (model, Cmd.none)
        RegisterFormMsg msg ->
            ( { model | registerForm = Register.update msg model.registerForm }, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ = Sub.none