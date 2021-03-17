hirom

; Settings

incsrc "./settings.asm"
incsrc "./tabs/settings.asm"
incsrc "./expgoldtech/settings.asm"

; Rom Modifications

if !IncreaseExpGoldTech != 0
incsrc "./expgoldtech/hook.asm"
endif

if !BetterTabs != 0
incsrc "./tabs/hook.asm"
incsrc "./tabs/text.asm"
endif

; Free spaces / New routines

org $C37A30
warnpc $C38000

org $CDF9C0
if !BetterTabs != 0
incsrc "./tabs/routine.asm"
endif

if !IncreaseExpGoldTech != 0
incsrc "./expgoldtech/routine.asm"
endif

warnpc $CE0000

