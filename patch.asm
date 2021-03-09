hirom

; Settings

!IncreasedExpGoldTech = 0 ; 1 = 4x exp, 8x gold, 4x tech points
!BetterTabs = 1 ; 1 = tabs increase more stats
!PowerTabIncreasesHit = 0 ; power tab will increase hit instead of speed tab
!JetsOfTimeRando = 0 ; 1 = compatible with jets of time randomizer, incompatible with normal game

!PowerIncrease = 2
!StaminaIncrease = 2
!SpeedIncrease = 1
!MagicIncrease = 2
!HitIncrease = 2
!EvadeIncrease = 2
!MagicDefenseIncrease = 2

; Setting Assertions

if !PowerTabIncreasesHit != 0
    !HitTab = "Power"
else
    !HitTab = "Speed"
endif

assert !PowerIncrease <= 99, "Power tab cannot increase more than 99 power"
assert !PowerIncrease >= 0, "Power tab cannot decrease power"
assert !StaminaIncrease <= 99, "Power tab cannot increase more than 99 stamina"
assert !StaminaIncrease >= 0, "Power tab cannot decrease stamina"
assert !SpeedIncrease <= 16, "Speed tab cannot increase more than 16 speed"
assert !SpeedIncrease >= 0, "Speed tab cannot decrease speed"
assert !MagicIncrease <= 99, "Magic tab cannot increase more than 99 magic"
assert !HitIncrease <= 99, "!HitTab tab cannot increase more than 99 hit"
assert !HitIncrease >= 0, "!HitTab tab cannot decrease hit"
assert !EvadeIncrease <= 99, "Speed tab cannot increase more than 99 evade"
assert !EvadeIncrease >= 0, "Speed tab cannot decrease evade"
assert !MagicDefenseIncrease <= 99, "Magic tab cannot increase more than 99 magic defense"
assert !MagicDefenseIncrease >= 0, "Magic tab cannot decrease magic defense"

; Increased Exp/Gold/Tech points

if !IncreasedExpGoldTech != 0
org $FDABD6
    JML AddPoints
org $FDAC05
AddPoints.ReturnPoint:
endif

; Better tabs

if !BetterTabs != 0
org $C2B2F8
    JML TabIncrease
org $C2B301
    TabIncrease.Return:

org $C2B298
    JML TabCondition
org $C2B2A1
    TabCondition.Allow:
org $C2B2A6
    TabCondition.Disallow:

if !JetsOfTimeRando != 0
org $F75723
else
org $CC304B
endif
ItemDescriptionPointers:
    dw PowerTabText
    dw MagicTabText
    dw SpeedTabText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw D0ItemText
    dw RaceLoggerText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText
    dw DEItemText

if !JetsOfTimeRando != 0
org $F75DDB
else
org $CC36C1
endif


!n = 0
macro NumberToText(value)
    !n #= !n+1
    !{value!{n}} #= <value>
    if !{value!{n}} >= 10
       !{n!{n}} = !n
       %NumberToText(safediv(!{value!{n!{n}}}, 10, 0))
       !{value!{n}} #= !{value!{n}}%10
    endif
    db $D4+!{value!{n}}
    !n #= !n-1
endmacro

PowerTabText:
    db $AF, $62, $26, $EC ; Power+
    %NumberToText(!PowerIncrease)
    db $E0, $B2, $CD, $BA, $C6, $25, $BA, $EC ; /Stamina+
    %NumberToText(!StaminaIncrease)
if !PowerTabIncreasesHit != 0
    db $E0, $A7, $2E, $EC ; /Hit+
    %NumberToText(!HitIncrease)
endif
    db $00 ; Null

MagicTabText:
    db $AC, $BA, $C0, $C2, $BC, $EC ; Magic+
    %NumberToText(!MagicIncrease)
    db $E0, $AC, $E8, $A3, $BE, $BF, $33, $5A, $EC ; /M.Defense+
    %NumberToText(!MagicDefenseIncrease)
    db $00 ; Null

SpeedTabText:
    db $B2, $C9, $6D, $BD, $EC ; Speed+
    %NumberToText(!SpeedIncrease)
if !PowerTabIncreasesHit == 0
    db $E0, $A7, $2E, $EC ; /Hit+
    %NumberToText(!HitIncrease)
endif
    db $E0, $A4, $CF, $BA, $BD, $BE, $EC ; /Evade+
    %NumberToText(!EvadeIncrease)
    db $00 ; Null

D0ItemText:
    db $00

RaceLoggerText:
    db $AB, $C8, $C0, $72, $EF, $A1, $C2, $C4
    db $BE, $EF, $B1, $BA, $98, $EF, $CC, $BC
    db $38, $BE, $00

DEItemText:
    db $00

endif


; Free spaces / New routines

org $C37A30
warnpc $C38000

org $CDF9C0
if !IncreasedExpGoldTech != 0
macro ShiftPointsLeft(value, by, overLimit)
    LDA <value>, X
    !by #= <by>
    while !by > 0
        ASL : BCS <overLimit>
        !by #= !by-1
    endif
endmacro

AddPoints:
    ; Experience
    LDA $5E3F : AND #$00FF : CMP #$003C : BCS + ; if pc1 level is less than 60
        %ShiftPointsLeft($CC5E00, 2, .expOverLimit) ; multiply experience by 4
        BRA .afterExpMultiplication
    + CMP #$0050 : BCS + ; if pc1 level is less than 80
        %ShiftPointsLeft($CC5E00, 3, .expOverLimit) ; multiply experience by 8
        BRA .afterExpMultiplication
    + ; if pc1 level is equal or above 80
        %ShiftPointsLeft($CC5E00, 4, .expOverLimit) ; multiply experience by 16
    .afterExpMultiplication
    CLC : ADC $B28C
    BCC +
    .expOverLimit
        LDA #$FFFF
    +
    STA $B28C

    ; Money
    %ShiftPointsLeft($CC5E02, 3, .moneyOverLimit)
    CLC : ADC $B2A5
    BCC +
    .moneyOverLimit
        LDA #$FFFF
    +
    STA $B2A5

    ; Tech Points
    TDC
    SEP #$20
    LDA $CC5E06, X
    REP #$20
    ASL #2
    CLC : ADC $B2DB
    BCC +
      SEP #$20
      INC $B2DD
      REP #$20
    +
    STA $B2DB
JML AddPoints.ReturnPoint
endif

if !BetterTabs != 0
macro StatCondition(value, statOffset, max, allow)
     if <value> > 0
         LDA $9A9B+<statOffset> : CMP #<max> : BCC <allow>
     endif
endmacro

TabCondition:
    CPY #$00 : BEQ .powerTab
    CPY #$02 : BEQ .speedTab
    CPY #$03 : BEQ .magicTab
    JML TabCondition.Disallow

    .powerTab
    %StatCondition(!PowerIncrease,   00, $63, .allow)
    %StatCondition(!StaminaIncrease, 01, $63, .allow)
    if !PowerTabIncreasesHit != 0
        %StatCondition(!HitIncrease, 04, $63, .allow)
    endif
    JML TabCondition.Disallow

    .speedTab
    %StatCondition(!SpeedIncrease, 02, $10, .allow)
    %StatCondition(!EvadeIncrease, 05, $63, .allow)
    if !PowerTabIncreasesHit == 0
        %StatCondition(!HitIncrease, 04, $63, .allow)
    endif
    JML TabCondition.Disallow

    .magicTab
    %StatCondition(!MagicIncrease,        03, $63, .allow)
    %StatCondition(!MagicDefenseIncrease, 06, $63, .allow)
    JML TabCondition.Disallow

    .allow
JML TabCondition.Allow


macro IncreaseStat(statOffset, max, by)
    LDA <statOffset>, X : CLC : ADC #<by> : CMP #<max> : BCC ?setStat
        LDA #<max>
    ?setStat:
    STA <statOffset>, X
endmacro

TabIncrease:
    SEP #$20
    LDX $6F
    CMP #$00 : BEQ .powerTab
    CMP #$02 : BEQ .speedTab
    CMP #$03 : BEQ .magicTab
    JML TabIncrease.Return

    .powerTab
    %IncreaseStat($000B, $63, !PowerIncrease)
    %IncreaseStat($000C, $63, !StaminaIncrease)
    if !PowerTabIncreasesHit != 0
        %IncreaseStat($000F, $63, !HitIncrease)
    endif
    JML TabIncrease.Return

    .speedTab
    %IncreaseStat($000D, $10, !SpeedIncrease)
    %IncreaseStat($0010, $63, !EvadeIncrease)
    if !PowerTabIncreasesHit == 0
        %IncreaseStat($000F, $63, !HitIncrease)
    endif
    JML TabIncrease.Return

    .magicTab
    %IncreaseStat($000E, $63, !MagicIncrease)
    %IncreaseStat($0011, $63, !MagicDefenseIncrease)
JML TabIncrease.Return
endif

warnpc $CE0000
