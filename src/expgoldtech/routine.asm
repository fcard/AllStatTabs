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
    if !GradualExpIncrease != 0
        PHY
        SEP #$20
        LDA $5E3F
        if !GradualExpMin <= 0
            CMP #$14 : BCS + ; if pc1 level is less than 20
                LDY #$0001
                BRA .startMultiplication
            +
        endif
        if !GradualExpMin <= 1
            CMP #$28 : BCS + ; if pc1 level is less than 40
                LDY #$0002
                BRA .startMultiplication
            +
        endif
        if !GradualExpMin <= 2
            CMP #$3C : BCS + ; if pc1 level is less than 60
                LDY #$0003
                BRA .startMultiplication
            +
        endif
        if !GradualExpMin <= 3
            CMP #$50 : BCS + ; if pc1 level is less than 80
                LDY #$0004
                BRA .startMultiplication
            +
        endif
        LDY #$0005
        .startMultiplication
            REP #$20
            LDA $CC5E00, X
        .multiplicationLoop
            DEY : BEQ .endMultiplication
            ASL : BCS .expOverLimit
        .endMultiplication
        PLY
    else
        %ShiftPointsLeft($CC5E00, 2, .expOverLimit) ; multiply experience by 4
    endif

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

