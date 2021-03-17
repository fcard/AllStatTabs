# AllStatTabs

Hack for Chrono Trigger that make tabs increase all stats (plus options for increased experience/gold/tech points as bonus)

The source patch requires Asar. Use it like this:
```sh
$ asar src/patch.asm <chrono-trigger-rom.smc>
```
This will modify the rom to be the patched rom, so you should use a copy.

# Settings

The repository comes with a `settings.conf` file that, if either placed in the same directory as the patch.asm file or in the working directory when calling asar, will affect the hack in various ways. The repository also has a GUI to generate and change those settings; amd64 Windows and Linux binaries are provided in the releases page; for information on building it, see `Building the GUI` below.

The settings are:

### BetterTabs
Required for every other tab setting to work. Set to 1 to enable, 0 to disable.

### PowerTabIncreasesHit
Power tab will increase the hit stat instead of the speed tab. Set to 1 to enable, 0 to disable.

### AllowStatDecrease
Allow stat increases to be set to negative values. If disabled, this will cause a compilation error if they are below 0. Used as a safeguard, mostly. Set to 1 to enable, 0 to disable.

### JetsOfTime

Enable to make the hack compatible with the Jets of Time randomizer. This in turn makes it
incompatible with the normal game. This currently only affects the text routines. Set to 1 to enable, 0 to disable.

### PowerIncrease, StaminaIncrease, etc
Changes the amount each tab increases, e.g. `PowerIncrease = 3` will make the power tab increase 3 power. Maximum is 99 for all stats except speed, for which it is 16. Minimum is either 0 or its maximum negated, depending on `AllowStatDecrease`.

### IncreaseExpGoldTech
Enable to increase exp/gold/tech points gain. By default, it multiplies experience by 4, gold by 8, and tech points by 4. Set to 1 to enable, 0 to disable.

### GradualExpIncrease
This option changes experience growth to match the leading party member's level: when they are any level below 20, the growth is normal; between 20 and 39, experience is doubled; between 40 and 59, it is multiplied by 4; between 60 and 79, it is multiplied by 8; finally, between 80 and 99, it is multiplied by 16. Set to 1 to enable, 0 to disable.

### GradualExpMin
This setting will affect the above setting in the following way: if set to `n`, then the experience growth between levels between `1` and `(n*20)-1` will be the same as the growth between levels `n*20` and `((n+1)*20)-1`. E.g. if set to `1`, then a player with levels from `1` to `39` will have the experience multiplied by 4; if set to `2`, then a player with levels from `1` to `59` will have the experience multiplied by 8. It can be set to values from 0 to 4, inclusive.

# Building the GUI

This hack comes with a GUI to help you change the settings of the hack and apply it to your rom. It's a rust crate contained in the `gui` directory. To build it, first off, you need to download this repository's git submodule:
```sh
$ git submodule init
$ git submodule update --recursive
```
Then, download Rust and Cargo. You can find them here: https://www.rust-lang.org/learn/get-started. Finally, go to the gui directory in this repo in a terminal and do:
```sh
$ cargo build --release
```
This will build a executable in `gui/target/release`.

