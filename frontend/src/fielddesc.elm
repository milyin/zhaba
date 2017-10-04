module FieldDesc exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Form.Validate exposing (Validation)
import Form.Validate as V
import Form exposing (Form)
import Form.Input

type alias FieldDesc e a =
    { label: String
    , name: String
    , validation: Validation e a
    }

-- utility functions for using FieldDesc instead of string field name in Form library

field : FieldDesc e a -> Validation e a
field desc = V.field desc.name desc.validation

andMap : FieldDesc e a -> Validation e (a -> b) -> Validation e b
andMap desc partialValidation = V.andMap (field desc) partialValidation

getStateString : FieldDesc e a -> Form e o -> (FieldDesc e a, Form.FieldState e String)
getStateString desc = \frm -> (desc, Form.getFieldAsString desc.name frm)

getStateBool : FieldDesc e a -> Form e o -> (FieldDesc e a, Form.FieldState e Bool)
getStateBool desc = \frm -> (desc, Form.getFieldAsBool desc.name frm)

-- default html representations for form fields

errorFor : FieldDesc e a -> Form.FieldState e v -> Html msg
errorFor desc state =
    case state.liveError of
        Just error ->
            div [class "error" ] [ text (toString error)]
        Nothing ->
            text ""

inputWith : Form.Input.Input e v -> (FieldDesc e a, Form.FieldState e v) -> Html Form.Msg
inputWith formInput (desc, state) = div []
    [ label [] [ text desc.label]
    , formInput state []
    , errorFor desc state
    ]
