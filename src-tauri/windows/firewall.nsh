!macro ReadAndDeleteExitCode ResultVar FilePath
    FileOpen $R9 "${FilePath}" r
    FileRead $R9 ${ResultVar}
    FileClose $R9
    Delete "${FilePath}"
!macroend

!define ReadAndDeleteExitCode "!insertmacro ReadAndDeleteExitCode"

!macro NSIS_HOOK_POSTINSTALL
    ExecShellWait "open" "$INSTDIR\firewall_helper.exe" "--remove" SW_HIDE $0

    ; For some reason ExecShellWait doesn't return exit code, when process requires elevation.
    ${ReadAndDeleteExitCode} $0 "$INSTDIR\exit-code.txt"

    MessageBox MB_OK "Exit code raw: $0"
    ${If} $0 == "error"
        MessageBox MB_ICONEXCLAMATION "Failed to launch firewall_helper.exe"
    ${ElseIf} $0 != 0
        IntFmt $1 "0x%08X" $0
        StrCpy $2 "Failed to add firewall rule.$\n$\n"
        StrCpy $2 "$2Exit code: $1$\n$\n"
        StrCpy $2 "$2The application may not be reachable on your local network.$\n"
        StrCpy $2 "$2You can add the rule manually in Windows Firewall."
        MessageBox MB_ICONEXCLAMATION "$2"
    ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
    ExecShellWait "open" "$INSTDIR\firewall_helper.exe" "--remove" SW_HIDE $0

    ${ReadAndDeleteExitCode} $0 "$INSTDIR\exit-code.txt"

    ${If} $0 != 0
        IntFmt $1 "0x%08X" $0
        StrCpy $2 "Failed to remove firewall rule.$\n$\n"
        StrCpy $2 "$2Exit code: $1$\n$\n"
        StrCpy $2 "$2You may need to remove it manually from Windows Firewall."
        MessageBox MB_ICONEXCLAMATION "$2"
    ${EndIf}
!macroend