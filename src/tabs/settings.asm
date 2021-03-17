; 1 = tabs increase more stats
%Setting(BetterTabs, 1, bool)

; 1 = power tab will increase hit instead of speed tab
%Setting(PowerTabIncreasesHit, 0, bool)

; 1 = compatible with jets of time randomizer, incompatible with normal game
%Setting(JetsOfTimeRando, 0, bool)

; 1 = allow stats to be decreased by tabs
%Setting(AllowStatDecrease, 0, bool)

%Setting(PowerIncrease,        2, sbyte)
%Setting(StaminaIncrease,      2, sbyte)
%Setting(SpeedIncrease,        1, sbyte)
%Setting(MagicIncrease,        2, sbyte)
%Setting(HitIncrease,          2, sbyte)
%Setting(EvadeIncrease,        2, sbyte)
%Setting(MagicDefenseIncrease, 2, sbyte)

; Setting Assertions

if !PowerTabIncreasesHit != 0
    !HitTab = "Power"
else
    !HitTab = "Speed"
endif

assert !PowerIncrease <= 99, "Power tab cannot increase more than 99 power"
assert !PowerIncrease >= -99, "Power tab cannot decrease more than 99 power"
assert !StaminaIncrease <= 99, "Power tab cannot increase more than 99 stamina"
assert !StaminaIncrease >= -99, "Power tab cannot decrease more than 99 stamina"
assert !SpeedIncrease <= 16, "Speed tab cannot increase more than 16 speed"
assert !SpeedIncrease >= -16, "Speed tab cannot decrease more than 16 speed"
assert !MagicIncrease <= 99, "Magic tab cannot increase more than 99 magic"
assert !HitIncrease <= 99, "!HitTab tab cannot increase more than 99 hit"
assert !HitIncrease >= -99, "!HitTab tab cannot decrease more than 99 hit"
assert !EvadeIncrease <= 99, "Speed tab cannot increase more than 99 evade"
assert !EvadeIncrease >= -99, "Speed tab cannot decrease more than 99 evade"
assert !MagicDefenseIncrease <= 99, "Magic tab cannot increase more than 99 magic defense"
assert !MagicDefenseIncrease >= -99, "Magic tab cannot decrease more than 99 magic defense"

if !AllowStatDecrease == 0
    assert !PowerIncrease >= 0, "Power tab cannot decrease power"
    assert !StaminaIncrease >= 0, "Power tab cannot decrease stamina"
    assert !SpeedIncrease >= 0, "Speed tab cannot decrease speed"
    assert !MagicIncrease >= 0, "Magic tab cannot decrease magic"
    assert !HitIncrease >= 0, "!HitTab cannot decrease hit"
    assert !EvadeIncrease >= 0, "Speed tab cannot decrease evade"
    assert !MagicDefenseIncrease >= 0, "Magic tab cannot decrease magic defense"
endif

