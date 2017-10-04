module Fields exposing (..)

import FieldDesc exposing (FieldDesc)
import Form.Validate as V

xNAME = FieldDesc "Name" "name" V.string
xEMAIL = FieldDesc "Email" "email" V.email
xPASSWORD = FieldDesc "Password" "password" V.string


