module Register exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Form exposing (Form)
import Form.Validate exposing (Validation, succeed, fail, andThen, field, customError, oneOf)
import Form.Input as Input
import Fields exposing (..)
import FieldDesc exposing (..)
import Form.Error as Error
import Http
import Json.Decode
import Json.Encode as JE
import Maybe exposing (Maybe)

type Msg
    = FormMsg Form.Msg
    | Submit
    | Response (Result Http.Error String)

type alias Register =
    { name: String
    , email: String
    , password : String
    }

serialize : Register -> JE.Value
serialize r = JE.object
    [ ("name", JE.string r.name)
    , ("email", JE.string r.email)
    , ("password", JE.string r.password)
    ]


postForm : Register -> Http.Request String
postForm r =
    Http.request
        { method = "POST"
        , headers = []
        , url = "/update/register"
        , body = Http.jsonBody (serialize r)
        , expect = Http.expectStringResponse (\resp -> Ok (toString resp))
        , timeout = Nothing
        , withCredentials = True
        }


type MyError = PasswordMismatch

type alias Model =
    { form: Form MyError Register
    , response: String
    }

validation : Validation MyError Register
validation = succeed Register
    |> andThen (isEqual xPASSWORD2 xPASSWORD (customError PasswordMismatch))
    |> andMapDesc xNAME
    |> andMapDesc xEMAIL
    |> andMapDesc xPASSWORD

init : Model
init =
    { form = Form.initial [] validation
    , response = ""
    }

update : Msg -> Model -> (Model, Cmd Msg)
update msg model = case msg of
    FormMsg m -> ({ model | form = Form.update validation m model.form}, Cmd.none)
    Submit -> case Form.getOutput model.form of
        Just r -> (model, Http.send Response <| postForm r)
        Nothing -> (model, Cmd.none)
    Response resp -> ({model | response = (toString resp) }, Cmd.none)

view : Model -> Html Msg
view model = let
        name = getStateString xNAME model.form
        email = getStateString xEMAIL model.form
        password = getStateString xPASSWORD model.form
        password2 = getStateString xPASSWORD2 model.form
    in
        div []
            [ inputWith FormMsg Input.textInput name
            , inputWith FormMsg Input.textInput email
            , inputWith FormMsg Input.passwordInput password
            , inputWith FormMsg Input.passwordInput password2
            , button
                [ onClick Submit ]
                [ text "Submit" ]
            , div [] [ text model.response ]
            ]


