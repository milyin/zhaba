module Fields exposing (..)

import FieldDesc exposing (FieldDesc, optional)
import Form.Validate as V

xNAME = FieldDesc "Name" "name" V.string
xEMAIL = FieldDesc "Email" "email" <| optional V.email
xPASSWORD = FieldDesc "Password" "password" V.string
xPASSWORD2 = FieldDesc "Password" "password2" <| optional V.string


