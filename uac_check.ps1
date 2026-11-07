$key = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"
Get-ItemProperty $key | Select-Object ConsentPromptBehaviorAdmin, ConsentPromptBehaviorUser, EnableLUA, PromptOnSecureDesktop, FilterAdministratorToken | Format-List
