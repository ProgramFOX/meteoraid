# Meteoraid

Assists you with processing visual meteor observations. You provide a sequence of events (meteors, changes in field obstructions, etc.),
and Meteoraid generates the CSVs with the meteor count and the distribution, that can be sent to the International Meteor Organization.

Run `meteoraid --help` for the possible command-line arguments.

The specification of the input file is provided in [doc/input.md](./tree/master/doc/input.md).

Example input file:

```lua
2114
period_start
date("11 Aug 2019")
showers(PER, KCG, ANT, SPO)
fieldC(290, 55)
clouds(5)
areas(area14(8))
spo(2)
clouds(0) << 2119
spo(3.5)
clouds(10) << 2123
spo(0.5)
per(1)
clouds(15) << 2127
clouds(0) << 2132
per(1.5)
spo(2)
per(2)
areas(area14(8)) << 2135
spo(2)
spo(3.5)
ant(1.5)
areas(area14(9)) << 2152
per(1.5)
spo(4)
kcg(4)
spo(1.5)
spo(3.5)
2231
areas(area14(8))
per(3.5)
spo(0)
spo(1.5)
per(-1)
per(1)
areas(area14(9)) << 2257
per(2)
per(0.5)
spo(2.5)
per(2.5)
spo(2.5)
period_end << 2317

new_period

period_start
date("11 Aug 2019")
fieldC(336, 52.3)
clouds(0)
areas(area14(9))
showers(PER, KCG, ANT, SPO)
spo(3)
spo(3)
per(3)
per(3.5)
kcg(4)
spo(2)
kcg(-1)
spo(1.5)
per(3.5)
spo(-0.5)
per(2)
per(0)
areas(area14(9)) << 1
spo(3)
per(-2)
spo(3)
per(-1)
per(1.5)
ant(2.5)
per(3)
per(1)
period_end << 12
```

This generates these CSVs as output:

```
DATE UT;START;END;Teff;RA;Dec;F;Lm;ANT;;KCG;;PER;;SPO
11 Aug 2019;2114;2317;2.05;290;55;1.01;5.22;C;1;C;1;C;10;C;13
11 Aug 2019;2317;12;0.9166666666666666;336;52.3;1;5.39;C;1;C;2;C;10;C;7
```

and

```
DATE UT;START;END;SHOWER;-6;-5;-4;-3;-2;-1;0;1;2;3;4;5;6;7
11 Aug 2019;2114;2317;ANT;0;0;0;0;0;0;0;0.5;0.5;0;0;0;0;0
11 Aug 2019;2114;2317;KCG;0;0;0;0;0;0;0;0;0;0;1;0;0;0
11 Aug 2019;2114;2317;PER;0;0;0;0;0;1;0.5;3.5;3.5;1;0.5;0;0;0
11 Aug 2019;2114;2317;SPO;0;0;0;0;0;0;1.5;1.5;5;2.5;2.5;0;0;0
11 Aug 2019;2317;12;ANT;0;0;0;0;0;0;0;0;0.5;0.5;0;0;0;0
11 Aug 2019;2317;12;KCG;0;0;0;0;0;1;0;0;0;0;1;0;0;0
11 Aug 2019;2317;12;PER;0;0;0;0;1;1;1;1.5;1.5;3;1;0;0;0
11 Aug 2019;2317;12;SPO;0;0;0;0;0;0.5;0.5;0.5;1.5;4;0;0;0;0
```
