@echo off
Title SquareRoot
:StartSquareRoot
cls
set /A var=2
set /A var1=1
if 1 EQU 1 (goto OneEqualOne) ELSE (goto how)
:OneEqualOne
set /A number=%RANDOM%

call :SquareRoot %number%

goto StartSquareRoot

:SquareRoot
SETLOCAL EnableDelayedExpansion
set root=1
set /a sqr=%root%*%root%
:Loop
if %sqr% LSS %number% (
	set /a root=!root!+1
        set /a sqr=!root!*!root!
        goto Loop
    	)

)
(EndLocal && set answer=%root% && exit /B)