!macro NSIS_HOOK_POSTINSTALL
    nsExec::Exec '"$INSTDIR\helper.exe" --add' $0

    ${If} $0 != 0
        IntFmt $1 "0x%08X" $0
        StrCpy $2 "Failed to add firewall rule.$\n$\n"
        StrCpy $2 "$2Exit code: $1$\n$\n"
        StrCpy $2 "$2The application may not be reachable on your local network.$\n"
        StrCpy $2 "$2You can add the rule manually in Windows Firewall."
        MessageBox MB_ICONEXCLAMATION "$2"
    ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
    ExecWait '"$INSTDIR\helper.exe" --remove' $0

    ${If} $0 != 0
        IntFmt $1 "0x%08X" $0
        StrCpy $2 "Failed to remove firewall rule.$\n$\n"
        StrCpy $2 "$2Exit code: $1$\n$\n"
        StrCpy $2 "$2You may need to remove it manually from Windows Firewall."
        MessageBox MB_ICONEXCLAMATION "$2"
    ${EndIf}
!macroend