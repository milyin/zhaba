module FieldDesc exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Form.Validate exposing (Validation)
import Form.Validate as V
import Form exposing (Form)
import Form.Input
import Form.Error exposing (Error)

type alias FieldDesc e a =
    { label: String
    , name: String
    , validation: Validation e a
    }

-- utility functions for using FieldDesc instead of string field name in Form library

fielddesc : FieldDesc e a -> Validation e a
fielddesc desc = V.field desc.name desc.validation

andMapDesc : FieldDesc e a -> Validation e (a -> b) -> Validation e b
andMapDesc desc partialValidation = V.andMap (fielddesc desc) partialValidation

getStateString : FieldDesc e a -> Form e o -> (FieldDesc e a, Form.FieldState e String)
getStateString desc = \frm -> (desc, Form.getFieldAsString desc.name frm)

getStateBool : FieldDesc e a -> Form e o -> (FieldDesc e a, Form.FieldState e Bool)
getStateBool desc = \frm -> (desc, Form.getFieldAsBool desc.name frm)

type Pair b = Pair b b
isEqual : FieldDesc e b -> FieldDesc e b -> Error e -> a -> Validation e a
isEqual desc1 desc2 err pass fields = let
            pval = V.succeed Pair
                |> andMapDesc desc1
                |> andMapDesc desc2
            fail = (V.field desc1.name <| V.fail err) fields
        in case (pval fields) of
            Ok (Pair v1 v2) ->
                if v1 == v2
                    then Ok pass
                    else fail
            Err _ -> Ok pass

-- utility validation functions
optional : Validation e String -> Validation e String
optional v = V.oneOf [ V.emptyString, v ]

-- default html representations for form fields

errorFor : FieldDesc e a -> Form.FieldState e v -> Html msg
errorFor desc state =
    case state.error of
        Just error ->
            div [class "error" ] [ text (toString error)]
        Nothing ->
            text ""

inputWith : (Form.Msg -> m) -> Form.Input.Input e v -> (FieldDesc e a, Form.FieldState e v) -> Html m
inputWith msgmap formInput (desc, state) = Html.map msgmap <| div []
    [ label [] [ text desc.label]
    , formInput state []
    , errorFor desc state
    ]
