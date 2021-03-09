# AllStatTabs
Hack for Chrono Trigger that make tabs increase all stats (plus options for increased experience/gold/tech points as bonus)

Requires Asar. Use it like this:
```sh
$ asar patch.asm <chrono-trigger-rom.smc>
```
This will modify the rom to be the patched rom, so you should use a copy.
This is Jets of Time compatible, as long as you change the relevant setting in patch.asm to 1.

You can change the amount each tab increases each stat by changing the `!*Increase` settings, e.g. `!PowerIncrease = 3` will make the power tab increase 3 power. There is also an option to make the power tab increase hit instead of the speed tab, `!PowerTabIncreasesHit`, set it to 1 to enable it.

There is an unrelated option to increase exp/gold/tech points gain, which you can enable by changing `!IncreaseExpGoldTech` to `1`. It multiplies experience by 4, gold by 8, and tech points by 4, and when you reach levels 60 and 80, experience gains grows to 8 and 16, respectively. I mostly added it to speed up rando runs. (I am not a fan of low level runs anyway, and I don't like not being able to buy all the stuff that's all in the shops) If it ever becomes a burden I might separate it into its own repo.

