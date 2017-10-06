module Register exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Form exposing (Form)
import Form.Validate exposing (Validation, succeed, fail, andThen, field, string, customError, oneOf)
import Form.Input as Input
import Fields exposing (..)
import FieldDesc exposing (..)
import Form.Error as Error

type alias Register =
    { name: String
    , email: String
    , password : String
    }

type MyError = PasswordMismatch

type alias Model = Form MyError Register

validation : Validation MyError Register
-- validation = (field xPASSWORD2.name (fail (Error.value Error.InvalidFormat)))
validation = succeed Register
    |> andThen (isEqual xPASSWORD2 xPASSWORD (customError PasswordMismatch))
    |> andMapDesc xNAME
    |> andMapDesc xEMAIL
    |> andMapDesc xPASSWORD

        --    in vreg |>
--    |> fielddesc xPASSWORD2 string

init : Model
init = Form.initial [] validation

update : Form.Msg -> Model -> Model
update msg = Form.update validation msg

view : Model -> Html Form.Msg
view model = let
        name = getStateString xNAME model
        email = getStateString xEMAIL model
        password = getStateString xPASSWORD model
        password2 = getStateString xPASSWORD2 model
    in
        div []
            [ inputWith Input.textInput name
            , inputWith Input.textInput email
            , inputWith Input.passwordInput password
            , inputWith Input.passwordInput password2
            , button
                [ onClick Form.Submit ]
                [ text "Submit" ]
            ]


