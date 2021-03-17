; Helper Variables and Macros

if !JetsOfTimeRando != 0
    !PowerTabTextPointer = $F75723
else
    !PowerTabTextPointer = $CC304B
endif

!PowerTabTextW #= read2(!PowerTabTextPointer)
!PowerTabText  #= (bank(!PowerTabTextPointer)<<16)+!PowerTabTextW

macro NumberToText(value)
    !value #= <value>

    ; Sign
    if <value> < 0
        db $EB
        !value #= -!value
    else
        db $EC
    endif
    !db_count #= !db_count+1

    ; Digits
    %_NumberToText(!value)
endmacro

!n = 0
!db_count = 0
macro _NumberToText(value)
    !n #= !n+1
    !{value!{n}} #= <value>
    if !{value!{n}} >= 10
       !{n!{n}} = !n
       %_NumberToText(safediv(!{value!{n!{n}}}, 10, 0))
       !{value!{n}} #= !{value!{n}}%10
    endif
    db $D4+!{value!{n}}
    !db_count #= !db_count+1
    !n #= !n-1
endmacro

macro WriteText(start_point)
    !memory_location := <start_point>
    !byte #= read1(!memory_location)
    while !byte != 0
        db !byte
        !memory_location #= !memory_location+1
        !byte #= read1(!memory_location)
    endif
    db 0
endmacro

; Tab Text

org !PowerTabTextPointer
ItemDescriptionPointers:
    dw PowerTabText
    dw MagicTabText
    dw SpeedTabText

org !PowerTabText
PowerTabText:
    db $AF, $62, $26 ; Power
    %NumberToText(!PowerIncrease)
    db $E0, $B2, $CD, $BA, $C6, $25, $BA ; /Stamina
    %NumberToText(!StaminaIncrease)
if !PowerTabIncreasesHit != 0
    db $E0, $A7, $2E ; /Hit
    %NumberToText(!HitIncrease)
    !db_count #= !db_count+4
endif
    db $00 ; Null
    !db_count #= !db_count+13

MagicTabText:
    db $AC, $BA, $C0, $C2, $BC ; Magic
    %NumberToText(!MagicIncrease)
    db $E0, $AC, $E8, $A3, $BE, $BF, $33, $5A ; /M.Defense
    %NumberToText(!MagicDefenseIncrease)
    db $00 ; Null
    !db_count #= !db_count+16

SpeedTabText:
    db $B2, $C9, $6D, $BD ; Speed
    %NumberToText(!SpeedIncrease)
if !PowerTabIncreasesHit == 0
    db $E0, $A7, $2E ; /Hit
    %NumberToText(!HitIncrease)
    !db_count #= !db_count+4
endif
    db $E0, $A4, $CF, $BA, $BD, $BE ; /Evade
    %NumberToText(!EvadeIncrease)
    db $00 ; Null
    !db_count #= !db_count+13

; Move other text

!OriginalTabDbCount = 30
!BK = bank(!PowerTabText)<<16
!db_diff #= !db_count-!OriginalTabDbCount
function text_pointer(offset) = !BK+read2(!PowerTabTextPointer+6+(offset*2))+!db_diff
function original_text_pointer(offset) = !BK+read2(!PowerTabTextPointer+6+(offset*2))

org !PowerTabTextPointer+6
    !i #= 0
    while !i <= 31
        dw text_pointer(!i)
        !i #= !i+1
    endif

!i #= 0
while !i <= 31
    org text_pointer(!i)
        %WriteText(original_text_pointer(!i))
    !i #= !i+1
endif

