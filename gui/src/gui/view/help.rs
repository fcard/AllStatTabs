pub const BETTER_TABS: &str =
"Enable to allow tabs to increase more stats, both in variety and \
 quantity; the power tab will increase power and stamina, the magic \
 tab will increase magic and magic defense, and the speed tab will \
 increase speed, hit and evasion. This option is also required for \
 all the other options in this category to take effect.";

pub const POWER_TAB_INCREASES_HIT: &str =
"Enable to make the power tab affect the hit stat instead of the speed tab.";

pub const JETS_OF_TIME: &str =
"Enable to make the hack compatible with the Jets of Time randomizer. This \
 in turn makes it incompatible with the normal game. This currently only \
 affects the text routines.";

pub const ALLOW_STAT_DECREASE: &str =
"Enable to allow tabs to decrease stats. A safeguard, mostly. If you disable \
 this while having negative values in the input fields, they won't be reset \
 so you don't lose your changes, but asar won't let you compile the patch \
 if you have negative stat increases and this setting disabled.";

pub const STAT_INCREASE: &str =
"Power Increase, Stamina Increase, etc. The amount to be increased by the tabs \
 if 'Better Tabs' is enabled. It can be 0 to disable that stat from being affected. \
 if all stats are 0 for a tab, then that tab cannot be used. Stat increases won't \
 put a character's stats above maximum or below 1. For all but speed, the maximum \
 increase is 99, and for speed it is 16. If 'Allow Stat Decrease' is enabled, the \
 minimum stat increase is the maximum negated (-16 for speed, -99 otherwise), otherwise \
 the minimum is 0.";

pub const EXPGOLDTECH_ALLOW: &str =
"Enable to multiply all experience, gold and tech points gained through combat. \
 By default, experience is multiplied by 4, gold is multiplied by 8, and tech points \
 are multiplied by 4. This setting is required by all the others settings in this \
 category.";

pub const GRADUAL_EXP: &str =
"Enable to change how experience is multiplied by the above setting, it will \
 be determined by the leading party member's level: When between level 1 and 19,
 experience will be normal; when between 20 and 39, it will be doubled; when \
 between 40 and 59, it will be multiplied by 4; when between 60 and 79, it will \
 be multiplied by 8; finally, when between 80 and 99, it will be multiplied by 16.";

pub const GRADUAL_EXP_MIN: &str =
"Change this to `n` to affect 'Gradual Experience Increase` in the following way: \
 when the leading party member's level is between 1 and (n*20)-1, the experience \
 growth will be the same as the one between levels n*20 through ((n+1)*20)-1. \
 e.g. if this setting is set to 1, then a player at levels between 1 and 39 will \
 have 4 times experience; if this setting is set to 2, then a player at levels \
 between 1 and 59 will have 8 times experience.";

