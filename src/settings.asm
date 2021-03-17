!Chr_0 = "\0"
!Chr_9 = "\t"
!Chr_10 = "\n"
!Chr_11 = "\v"
!Chr_12 = "\f"
!Chr_28 = "\e"
!Chr_32 = " "
!Chr_33 = "!"
!Chr_35 = "#"
!Chr_36 = "$"
!Chr_37 = "%"
!Chr_38 = "&"
!Chr_39 = "'"
!Chr_40 = "("
!Chr_41 = ")"
!Chr_42 = "*"
!Chr_43 = "+"
!Chr_44 = ","
!Chr_45 = "-"
!Chr_46 = "."
!Chr_47 = "/"
!Chr_48 = "0"
!Chr_49 = "1"
!Chr_50 = "2"
!Chr_51 = "3"
!Chr_52 = "4"
!Chr_53 = "5"
!Chr_54 = "6"
!Chr_55 = "7"
!Chr_56 = "8"
!Chr_57 = "9"
!Chr_58 = ":"
!Chr_59 = ";"
!Chr_60 = "<"
!Chr_61 = "="
!Chr_62 = ">"
!Chr_63 = "?"
!Chr_64 = "@"
!Chr_65 = "A"
!Chr_66 = "B"
!Chr_67 = "C"
!Chr_68 = "D"
!Chr_69 = "E"
!Chr_70 = "F"
!Chr_71 = "G"
!Chr_72 = "H"
!Chr_73 = "I"
!Chr_74 = "J"
!Chr_75 = "K"
!Chr_76 = "L"
!Chr_77 = "M"
!Chr_78 = "N"
!Chr_79 = "O"
!Chr_80 = "P"
!Chr_81 = "Q"
!Chr_82 = "R"
!Chr_83 = "S"
!Chr_84 = "T"
!Chr_85 = "U"
!Chr_86 = "V"
!Chr_87 = "W"
!Chr_88 = "X"
!Chr_89 = "Y"
!Chr_90 = "Z"
!Chr_91 = "["
!Chr_92 = "\\"
!Chr_93 = "]"
!Chr_94 = "^"
!Chr_95 = "_"
!Chr_96 = "`"
!Chr_97 = "a"
!Chr_98 = "b"
!Chr_99 = "c"
!Chr_100 = "d"
!Chr_101 = "e"
!Chr_102 = "f"
!Chr_103 = "g"
!Chr_104 = "h"
!Chr_105 = "i"
!Chr_106 = "j"
!Chr_107 = "k"
!Chr_108 = "l"
!Chr_109 = "m"
!Chr_110 = "n"
!Chr_111 = "o"
!Chr_112 = "p"
!Chr_113 = "q"
!Chr_114 = "r"
!Chr_115 = "s"
!Chr_116 = "t"
!Chr_117 = "u"
!Chr_118 = "v"
!Chr_119 = "w"
!Chr_120 = "x"
!Chr_121 = "y"
!Chr_122 = "z"
!Chr_123 = "{"
!Chr_124 = "|"
!Chr_125 = "|"
!Chr_126 = "}"
!Chr_127 = "~"

macro Chr(byte, result)
    !Chr_byte #= <byte>
    !<result> = !{Chr_!{Chr_byte}}
endmacro

macro SettingsIgnoreSpaces(file, memory)
    !byte #= readfile1("<file>", !<memory>, 0)
    while !byte == $20 || !byte == $09
        !<memory> #= !<memory>+1
        !byte #= readfile1("<file>", !<memory>, 0)
    endif
endmacro

function keypart(byte) = or(and(greaterequal(byte, $30), lessequal(byte, $39)),
                            or(and(greaterequal(byte, $41), lessequal(byte, $5A)),
                               or(and(greaterequal(byte, $61), lessequal(byte, $7A)),
                                  equal(byte, $5F))))

macro SettingsReadKey(file, memory, key)
    !<key> := ""
    !byte #= readfile1("<file>", !<memory>, 0)
    while keypart(!byte)
        %Chr(!byte, chr)
        !<key> := "!<key>!chr"
        !<memory> #= !<memory>+1
        !byte #= readfile1("<file>", !<memory>, 0)
    endif
endmacro

macro SettingsReadEquals(file, memory)
    !byte #= readfile1("<file>", !<memory>, 0)
    if !byte == $3D
        !<memory> #= !<memory>+1
    else
        error "!byte : Expected '=' between key and value, in settings.conf file"
    endif
endmacro

macro SettingsReadValue(file, memory, value)
    !<value> := ""
    !byte #= readfile1("<file>", !<memory>, 0)
    if !byte == $2D
        !<value> = "-"
        !<memory> #= !<memory>+1
        !byte #= readfile1("<file>", !<memory>, 0)
    elseif !byte == $2B
        !<memory> #= !<memory>+1
        !byte #= readfile1("<file>", !<memory>, 0)
    endif
    while !byte >= $30 && !byte <= $39
        !num #= !byte-$30
        !<value> := "!<value>!num"
        !<memory> #= !<memory>+1
        !byte #= readfile1("<file>", !<memory>, 0)
    endif
    if stringsequal("!<value>", "") || stringsequal("!<value>", "-")
        error "Expected a number after 'key = ', in settings.conf file"
    endif
endmacro

macro SettingsReadKeyValueEnd(file, memory)
    !byte #= readfile1("<file>", !<memory>, 0)
    if !byte == $0A || !byte == $00
        !<memory> #= !<memory>+1
    else
        error "Expected newline or EOF after 'key = value', in settings.conf file, found byte !byte"
    endif
endmacro

macro SettingsReadKeyValue(file, memory)
    %SettingsIgnoreSpaces(<file>, <memory>)
    %SettingsReadKey(<file>, <memory>, key)
    %SettingsIgnoreSpaces(<file>, <memory>)
    %SettingsReadEquals(<file>, <memory>)
    %SettingsIgnoreSpaces(<file>, <memory>)
    %SettingsReadValue(<file>, <memory>, value)
    %SettingsIgnoreSpaces(<file>, <memory>)
    %SettingsReadKeyValueEnd(<file>, <memory>)
    !{Setting_!{key}} #= !value
endmacro

macro SettingsReadAll(file)
  !SettingsReadAll_memory #= 0
  while !SettingsReadAll_memory < filesize("<file>")
      %SettingsReadKeyValue(<file>, SettingsReadAll_memory)
  endif
endmacro

if canreadfile("settings.conf", 0, 0)
    %SettingsReadAll("settings.conf")
    !SettingsDefault = 0
else
    !SettingsDefault = 1
endif

macro typecheck(value, type, min, max)
    if <value> < <min> || <value> > <max>
        error "<value> is not a valid <type>"
    endif
endmacro

macro typecheck2(value, type, min, max)
    if <value> < <min> || <value> > <max>
        error "<value> is neither a valid signed nor unsigned <type>"
    endif
endmacro

macro Setting(var, default, type)
    if !SettingsDefault != 0 || not(defined("Setting_<var>"))
        !<var> = <default>
    else
        !value := !Setting_<var>
        if stringsequal("<type>", "bool")
            if !value != 0
                !value = 1
            endif
        elseif stringsequal("<type>", "byte")
            %typecheck2(!value, byte, -$7F, $FF)
        elseif stringsequal("<type>", "sbyte")
            %typecheck(!value, sbyte, -$7F, $7F)
        elseif stringsequal("<type>", "ubyte")
            %typecheck(!value, ubyte, 0, $FF)
        elseif stringsequal("<type>", "word")
            %typecheck2(!value, word, -$7FFF, $FFFF)
        elseif stringsequal("<type>", "sword")
            %typecheck(!value, sword, -$7FFF, $7FFF)
        elseif stringsequal("<type>", "uword")
            %typecheck(!value, uword, 0, $FFFF)
        elseif stringsequal("<type>", "long")
            %typecheck2(!value, long, -$7FFFFF, $FFFFFF)
        elseif stringsequal("<type>", "slong")
            %typecheck(!value, slong, -$7FFFFF, $7FFFFF)
        elseif stringsequal("<type>", "ulong")
            %typecheck(!value, ulong, 0, $FFFFFF)
        endif
        !<var> := !value
    endif
endmacro

