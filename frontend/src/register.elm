module Register exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Form exposing (Form)
import Form.Validate exposing (Validation, succeed)
import Form.Input as Input
import Fields exposing (..)
import FieldDesc exposing (..)

type alias Register =
    { name: String
    , email: String
    , password : String
    }

type alias Model = Form () Register

validation : Validation () Register
validation = succeed Register
    |> andMap xNAME
    |> andMap xEMAIL
    |> andMap xPASSWORD

init : Model
init = Form.initial [] validation

update : Form.Msg -> Model -> Model
update msg = Form.update validation msg

view : Model -> Html Form.Msg
view model = let
        name = getStateString xNAME model
        email = getStateString xEMAIL model
        password = getStateString xPASSWORD model
    in
        div []
            [ inputWith Input.textInput name
            , inputWith Input.textInput email
            , inputWith Input.passwordInput password
            , button
                [ onClick Form.Submit ]
                [ text "Submit" ]
            ]


