hirom

; Settings

!BetterTabs = 1 ; 1 = tabs increase more stats
!IncreasedExpGoldTech = 0 ; 1 = 4x exp, 8x money, 4x tech points
!JetsOfTimeRando = 0 ; 1 = compatible with jets of time rando, incompatible with normal game

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
PowerTabText:
    db $AF, $62, $26, $EC, $D6, $E0, $B2, $CD
    db $BA, $C6, $25, $BA, $EC, $D6, $00

MagicTabText:
    db $AC, $BA, $C0, $C2, $BC, $EC, $D6, $E0
    db $AC, $E8, $A3, $BE, $BF, $33, $5A, $EC
    db $D6, $00

SpeedTabText:
    db $B2, $C9, $6D, $BD, $EC, $D5, $E0, $A7
    db $2E, $EC, $D6, $E0, $A4, $CF, $BA, $BD
    db $BE, $EC, $D6, $00

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

TabCondition:
    CPY #$00 : BEQ .powerTab
    CPY #$02 : BEQ .speedTab
    CPY #$03 : BEQ .magicTab
    JML TabCondition.Disallow

    .powerTab
    LDA $9A9B+00 : CMP #$63 : BCC .allow ; power
    LDA $9A9B+01 : CMP #$63 : BCC .allow ; stamina
    JML TabCondition.Disallow

    .speedTab
    LDA $9A9B+02 : CMP #$10 : BCC .allow ; speed
    LDA $9A9B+04 : CMP #$63 : BCC .allow ; hit
    LDA $9A9B+05 : CMP #$63 : BCC .allow ; evade
    JML TabCondition.Disallow

    .magicTab
    LDA $9A9B+03 : CMP #$63 : BCC .allow ; magic
    LDA $9A9B+06 : CMP #$63 : BCC .allow ; magic defense
    JML TabCondition.Disallow

    .allow
JML TabCondition.Allow


macro IncreaseStatBy2(statOffset, max)
    LDA <statOffset>, X : CMP #<max> : BCS ?afterStat
        INC <statOffset>, X
        CMP #(<max>-1) : BCS ?afterStat
        INC <statOffset>, X
    ?afterStat:
endmacro

macro IncreaseStatBy1(statOffset, max)
    LDA <statOffset>, X : CMP #<max> : BCS ?afterStat
        INC <statOffset>, X
    ?afterStat:
endmacro

TabIncrease:
    SEP #$20
    LDX $6F
    CMP #$00 : BEQ .powerTab
    CMP #$02 : BEQ .speedTab
    CMP #$03 : BEQ .magicTab
    JML TabIncrease.Return

    .powerTab
    %IncreaseStatBy2($000B, $63) ; power
    %IncreaseStatBy2($000C, $63) ; stamina
    JML TabIncrease.Return

    .speedTab
    %IncreaseStatBy1($000D, $10) ; speed
    %IncreaseStatBy2($000F, $63) ; hit
    %IncreaseStatBy2($0010, $63) ; evade
    JML TabIncrease.Return

    .magicTab
    %IncreaseStatBy2($000E, $63) ; magic
    %IncreaseStatBy2($0011, $63) ; magic defense
JML TabIncrease.Return

warnpc $CE0000



