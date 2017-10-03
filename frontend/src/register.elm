module Register exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Form exposing (Form)
import Form.Validate as Validate
import Form.Validate exposing (Validation)
import Form.Input as Input

-- util start

type alias FieldDesc e a =
    { name: String
    , validation: Validation e a
    }

field : FieldDesc e a -> Validation e a
field desc = Validate.field desc.name desc.validation

getAsString : FieldDesc e a -> Form e o -> Form.FieldState e String
getAsString desc = Form.getFieldAsString desc.name

getAsBool : FieldDesc e a -> Form e o -> Form.FieldState e Bool
getAsBool desc = Form.getFieldAsBool desc.name

name = FieldDesc "name" Validate.string
email = FieldDesc "email" Validate.email
password = FieldDesc "password" Validate.string

-- util end

type alias Register =
    { name: String
    , email: String
    , password : String
    }


type alias Model =
    { form : Form () Register
    }

init : Model
init = { form = Form.initial [] validation }

validation : Validation () Register
validation = Validate.map3 Register (field name) (field email) (field password)

update : Form.Msg -> Model -> Model
update msg ({form} as model) = { model | form = Form.update validation msg form }

errorFor field =
    case field.liveError of
        Just error ->
            div [class "error" ] [ text (toString error)]
        Nothing ->
            text ""

view : Model -> Html Form.Msg
view {form} = let
        sNAME = (getAsString name) form
        sEMAIL = (getAsString email) form
        sPASSWORD = (getAsString password) form
    in
        div []
            [ label [] [ text "Name" ]
            , Input.textInput sNAME []
            , errorFor sNAME
            , label [] [ text "Email" ]
            , Input.textInput sEMAIL []
            , errorFor sEMAIL
            , label [] [ text "Password" ]
            , Input.passwordInput sPASSWORD []
            , errorFor sPASSWORD
            , button
                [ onClick Form.Submit ]
                [ text "Submit" ]
            ]


