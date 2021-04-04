macro StatCondition(value, statOffset, max, allow)
     if <value> > 0
         LDA $9A9B+<statOffset> : CMP #<max> : BCC <allow>
     elseif <value> < 0
         LDA $9A9B+<statOffset> : CMP #$01 : BCS <allow>
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


macro _IncreaseStat(statOffset, max, by, reg)
    if <by> != 0
        if <by> < 0
            !by = -<by>
            LDA <statOffset>, <reg> : SEC : SBC.b #!by
                BEQ ?loadMin
                BCC ?loadMin
                BRA ?setStat
            ?loadMin:
                LDA #$01
        else
            LDA <statOffset>, <reg> : CLC : ADC.b #<by> : CMP #<max> : BCC ?setStat
                LDA #<max>
        endif
        ?setStat:
            STA <statOffset>, <reg>
    endif
endmacro

macro IncreaseStat(statOffset, max, by)
    %_IncreaseStat(<statOffset>, <max>, <by>, X)
    %_IncreaseStat(<statOffset>-$B+$2F, <max>, <by>, Y)
endmacro

TabIncrease:
    SEP #$20
    PHY
    LDX $6F
    LDY $00
    CMP #$00 : BEQ .powerTab
    CMP #$02 : BEQ .speedTabShort
    CMP #$03 : BEQ .magicTabShort
    JML TabIncrease.Return

    .speedTabShort
    BRL .speedTab

    .magicTabShort
    BRL .magicTab

    .powerTab
    %IncreaseStat($000B, $63, !PowerIncrease)
    %IncreaseStat($000C, $63, !StaminaIncrease)
    if !PowerTabIncreasesHit != 0
        %IncreaseStat($000F, $63, !HitIncrease)
    endif
    PLY
    JML TabIncrease.Return

    .speedTab
    %IncreaseStat($000D, $10, !SpeedIncrease)
    %IncreaseStat($0010, $63, !EvadeIncrease)
    if !PowerTabIncreasesHit == 0
        %IncreaseStat($000F, $63, !HitIncrease)
    endif
    PLY
    JML TabIncrease.Return

    .magicTab
    %IncreaseStat($000E, $63, !MagicIncrease)
    %IncreaseStat($0011, $63, !MagicDefenseIncrease)
    PLY
JML TabIncrease.Return
