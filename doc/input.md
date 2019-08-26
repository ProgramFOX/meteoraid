Every line in the input file either specifies a timestamp or an event.

## Timestamps

Timestamps are written as integers. The last two digits specify the minutes,
the digits before that specify the hours. For example, `2250` is 22:50,
`0133` and `133` both equal 1:33 and `15`, `015` and `0015` all equal
00:15.

You can use a full line in the input file to declare a timestamp, and then
that timestamp will be associated with all events until you declare a new one.

A timestamp can also be written on the same line as an event, separated with
`<<`. For example:

```lua
clouds(10) << 2310
```

This line is entirely equivalent to this sequence of two lines:

```lua
2310
clouds(10)
```

So in both of these representations, the timestamp `2310` will be associated
with all events until a new timestamp is specified.

## Comments

Everything after `--` is a comment and is ignored while parsing the input file.

```lua
2210 -- partial-line comment
-- full-line comment
```

## Periods

An observation session is split in periods. A period is started with
`period_start` and ended with `period_end`. Between two periods, a `new_period`
'event' is required.

An example of a session with two periods:

```lua
2252
period_start
-- list of events here
period_end << 2345

new_period

period_start << 0055
-- list of events here
period_end << 123
```

If there is no break between the two period, the timestamp for the second
`period_start` can be omitted:

```lua
period_start << 2250
-- ...
period_end << 2345

new_period

period_start
-- now this period started at 2345
-- ...
```

## List of events

### Date - `date`

**Required once per period.**

Declares the date at which the period started. There is no validation - just
pick a format that the IMO supports in their CSV files, preferrably `M d Y`
(e.g. `Aug 11 2019`) or `d M Y` (e.g. `11 Aug 2019`).

```lua
date("11 Aug 2019")
```

### Clouds (or other field obstructions) - `clouds`

**Required at the start of the period.** Can be used as much as you wish
elsewhere in the period.

Declares the percentage of clouds (or other field obstructions).

```lua
clouds(5)
```

### Showers - `showers`

**Required once per period.**

Declares which showers you observed, by their three-letter IMO code.
Note that sporadic meteors (SPO) are not automatically included!
You can provide up to 12 showers. All shower codes must be in uppercase.

```lua
showers(PER, KCG, ANT, SPO)
```

### Field - `fieldC`

**Required once at the beginning of your period.**

Declares your field (the sky part you're looking at). Takes right ascension as
first argument and declination as second argument. Floating-point numbers are
accepted.


```lua
fieldC(336, 52.3)
```

*The C in the function name stands for "coordinates", and is intended to
separate this function from a planned function where you would be able to
specify a field by a star name.*

### Counting stars in areas for limiting magnitude - `areas`

*Note: what I call "areas" here is also referred to as "fields" in the IMO
handbook, but I chose to go with "areas" to avoid any confusion with "field"
as used above.*

**Required at the start of the period.** Can be used as much as you wish
elsewhere in the period.

Declare how many stars you counted in which areas, so the limiting magnitude
can be calculated.

```lua
areas(area14(11), area7(10))
```

You can specify 1 to 12 areas. **It's important that you specify all counts in
the same `areas` function rather than in separate calls.** If you write the
counts separately, Meteoraid will calculate a different limiting magnitude for
each of them and only care about the last one (because the previous ones are
considered "applicable for zero minutes"). So, **DO NOT DO THIS:**

```lua
areas(area14(11)) -- this will be ignored!
areas(area7(10))  -- and only this will be used
```

### Meteors

You can declare a meteor by its three-letter IMO code (in lowercase!) with the
magnitude as argument.

```lua
per(3) -- a Perseid of magnitude 3
spor(0.5) -- a sporadic meteor of magnitude 0.5
kcg(-1) -- a Kappa Cygnid of magnitude -1
```

Note that you can only declare meteors of the showers that you are observing,
as specified using `showers`.

### Breaks - `break_start` and `break_end`

Declares breaks during your period. No events can happen between breaks.

```lua
break_start
break_end << 2357
```




