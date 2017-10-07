module Zhaba exposing (..)

import Html exposing (..)
import Html.Attributes exposing (style)
import Html.Events exposing (..)
import Register
import Form exposing (Form)
import Maybe exposing (..)

main = Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type Msg = Nop | RegisterFormMsg Register.Msg

type alias Model =
    { registerForm : Register.Model
    }

init : (Model, Cmd Msg)
init = ({
        registerForm = Register.init
    }, Cmd.none)

view : Model -> Html Msg
view model =  Html.map RegisterFormMsg (Register.view model.registerForm)

translate
    : (subMsg -> Msg)
    -> (subModel -> Model)
    -> (subModel, Cmd subMsg)
    -> (Model, Cmd Msg)
translate translMsg  updModel (subModel, msg) =
    (updModel subModel, Cmd.map translMsg msg)

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Nop -> (model, Cmd.none)
        RegisterFormMsg msg ->
            Register.update msg model.registerForm |> translate RegisterFormMsg (\m -> {model | registerForm = m})

subscriptions : Model -> Sub Msg
subscriptions _ = Sub.none