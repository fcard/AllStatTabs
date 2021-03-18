; 1 = multiply exp/gold/tech points. default: 4x exp, 8x gold, 4x tech points
%Setting(IncreaseExpGoldTech, 0, bool)

; 1 = 2x exp for each 20 levels (requires !IncreasedExpGoldTech)
%Setting(GradualExpIncrease, 0, bool)

; n = levels <= 20*n are ignored for !GradualExpIncrease
%Setting(GradualExpMin, 0, sbyte)

; n = number to multiply exp by (unless GradualExpIncrease is enabled)
%Setting(ExpIncrease, 4, sbyte)

; n = number to multiply gold by
%Setting(GoldIncrease, 8, sbyte)

; n = number to multiply tech points by
%Setting(TechIncrease, 4, sbyte)

