# AllStatTabs
Hack for Chrono Trigger that make tabs increase all stats (plus options for increased experience/gold/tech points as bonus)

Requires Asar. Use it like this:
```sh
$ asar patch.asm <chrono-trigger-rom.smc>
```
This will modify the rom to be the patched rom, so you should use a copy.
This is Jets of Time compatible, as long as you change the relevant setting in patch.asm to 1.

There is also an option to increase exp/gold/tech points gain, which you can set by changing `!IncreaseExpGoldTech` to `1`. It multiplies experience by 4, gold by 8, and tech points by 4, and when you reach levels 60 and 80, experience gains grows to 8 and 16, respectively. I mostly added it to speed up rando runs. (I am not a fan of low level runs anyway, and I don't like not being able to buy all the stuff that's all in the shops)
