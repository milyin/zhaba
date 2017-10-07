for %%i in (%0) do set THIS=%%~dpi
mkdir %THIS%..\static
copy %THIS%index.html %THIS%..\static
elm-make --output=%THIS%..\static\elm.js %THIS%src\zhaba.elm --yes

