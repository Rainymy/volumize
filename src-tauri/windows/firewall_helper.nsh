!ifndef FIREWALL_HELPER_NSH
!define FIREWALL_HELPER_NSH

    ; Reads the exit code from a file and deletes it after reading.
    ; - MUST AVOID USING $R9
    !macro ReadAndDeleteExitCode ResultVar FilePath
        Push $R9
        FileOpen $R9 "${FilePath}" r
        FileRead $R9 ${ResultVar}
        FileClose $R9
        Delete "${FilePath}"
        Pop $R9
    !macroend
    !define ReadAndDeleteExitCode "!insertmacro ReadAndDeleteExitCode"

    !macro ShowFirewallError ExitCode Action
        Push $R7
        Push $R8

        StrCpy $R7 "The helper program could not [${Action}] the firewall rule$\n"
        StrCpy $R7 "$R7 $\tExit code: (${ExitCode})$\n"
        StrCpy $R7 "$R7$\n"
        MessageBox MB_ICONEXCLAMATION "$R7"

        Pop $R8
        Pop $R7
    !macroend
    !define ShowFirewallError "!insertmacro ShowFirewallError"

    !macro PromptFirewallConsent LabelPrefix Action Accepted
        Push $R7
        StrCpy $R7 "Windows Firewall — [${Action}] rule$\n"
        StrCpy $R7 "$R7$\n"
        StrCpy $R7 "$R7The installer needs admin rights to [${Action}] a firewall rule so the app is reachable on your local network.$\n"
        StrCpy $R7 "$R7$\n"
        StrCpy $R7 "$R7  Yes - run the helper now (UAC prompt will appear)$\n"
        StrCpy $R7 "$R7  No - skip, I will manage the firewall myself"

        MessageBox MB_YESNO|MB_ICONQUESTION \
            "$R7" \
            IDYES lbl_${LabelPrefix}_yes IDNO lbl_${LabelPrefix}_no
        lbl_${LabelPrefix}_yes: ; Agreed
            StrCpy ${Accepted} 1
            Goto lbl_${LabelPrefix}_done
        lbl_${LabelPrefix}_no: ; Disagreed
            StrCpy ${Accepted} 0
        lbl_${LabelPrefix}_done:

        Pop $R7
    !macroend
    !define PromptFirewallConsent "!insertmacro PromptFirewallConsent"

!endif ; FIREWALL_HELPER_NSH