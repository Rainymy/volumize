!addincludedir "${__FILEDIR__}"
!include firewall_helper.nsh

!macro NSIS_HOOK_POSTINSTALL
    !define PostInstallID ${__COUNTER__}

    ${PromptFirewallConsent} "postinstall_${PostInstallID}" "ADD" $R6
    ${If} $R6 == 0
        Goto postinstall_done_${PostInstallID}
    ${EndIf}

    ; For some reason ExecShellWait doesn't return exit code, when process requires elevation.
    ExecShellWait "open" "$INSTDIR\firewall_helper.exe" "--add" SW_HIDE $0
    ${ReadAndDeleteExitCode} $0 "$INSTDIR\exit-code.txt"

    ${If} $0 == "error"
        MessageBox MB_ICONEXCLAMATION "Failed to launch firewall_helper.exe"
    ${ElseIf} $0 != 0
        ${ShowFirewallError} $0 "ADD"
    ${EndIf}

    postinstall_done_${PostInstallID}:
    !undef PostInstallID
!macroend

!macro NSIS_HOOK_PREUNINSTALL
    !define PreUninstallID ${__COUNTER__}

    ${PromptFirewallConsent} "preuninstall_${PreUninstallID}" "REMOVE" $R6
    ${If} $R6 == 0
        Goto preuninstall_done_${PreUninstallID}
    ${EndIf}

    ExecShellWait "open" "$INSTDIR\firewall_helper.exe" "--remove" SW_HIDE $0
    ${ReadAndDeleteExitCode} $0 "$INSTDIR\exit-code.txt"

    ${If} $0 == "error"
        MessageBox MB_ICONEXCLAMATION "Failed to launch firewall_helper.exe"
    ${ElseIf} $0 != 0
        ${ShowFirewallError} $0 "REMOVE"
    ${EndIf}

    preuninstall_done_${PreUninstallID}:
    !undef PreUninstallID
!macroend