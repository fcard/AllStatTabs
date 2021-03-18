function is_power_of_two(x) = equal(1<<log2(x),x)

macro EachBit(prefix, value)
  !EachBit_value #= <value>
  !EachBit_i #= 0
  while !EachBit_value > 0
    !{<prefix>!{EachBit_i}} #= !EachBit_value%2
    !EachBit_value #= !EachBit_value>>1
    !EachBit_i #= !EachBit_i+1
  endif
  !<prefix>_length #= !EachBit_i
endmacro


macro MultiplyPoints(value, by, overLimit, eight_bits)
    if is_power_of_two(<by>)
        %ShiftPointsLeft(<value>, log2(<by>), <overLimit>, <eight_bits>)
    else
        if <eight_bits> != 0
            SEP #$20
        endif
        LDA <value>, X : STA $00
        if <eight_bits> != 0
            REP #$20
        endif
        ASL : BCS <overLimit>
        %EachBit(bit, <by>)
        !i #= !bit_length-2
        while !i >= 0
            if !{bit!{i}} == 1
                ADC $00 : BCS <overLimit>
            endif
            if !i > 0
                ASL : BCS <overLimit>
            endif
            !i #= !i-1
        endif
    endif
endmacro

macro ShiftPointsLeft(value, by, overLimit, eight_bits)
    if <eight_bits> != 0
        SEP #$20
    endif
    LDA <value>, X
    if <eight_bits> != 0
        REP #$20
    endif
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
        %MultiplyPoints($CC5E00, !ExpIncrease, .expOverLimit, 0) ; multiply experience
    endif

    CLC : ADC $B28C
    BCC +
    .expOverLimit
        LDA #$FFFF
    +
    STA $B28C

    ; Money
    %MultiplyPoints($CC5E02, !GoldIncrease, .moneyOverLimit, 0)
    CLC : ADC $B2A5
    BCC +
    .moneyOverLimit
        LDA #$FFFF
    +
    STA $B2A5

    ; Tech Points
    TDC
    %MultiplyPoints($CC5E06, !TechIncrease, .techOverLimit, 1)
    CLC : ADC $B2DB
    BCC +
    .techOverLimit
        SEP #$20
        INC $B2DD
        REP #$20
    +
    STA $B2DB
JML AddPoints.ReturnPoint

