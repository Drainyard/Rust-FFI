@echo off

WHERE cl 
IF %ERRORLEVEL% NEQ 0 call %VCVARSALL% x64

setlocal ENABLEDELAYEDEXPANSION

set CommonCompilerFlags=/MD -fp:fast -fp:except- -nologo -Oi -W4 -Gm- -GR- -EHa- -FC -Z7 /Fo"counter" /c 
set CommonLinkerFlags=
set ExtraLinkerFlags=/NODEFAULTLIB:"LIBCMT" -incremental:no -opt:ref

IF NOT EXIST bin mkdir bin
pushd bin

cl %CommonCompilerFlags% ..\src\lib.cpp
lib /NOLOGO ..\bin\counter.obj 

popd
