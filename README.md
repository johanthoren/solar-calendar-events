# solar-calendar-events

A Rust library designed to calculate equinoxes and solstices for a given year within the range 1900-2100. The accuracy is within a few minutes.

Example usage:

``` rust
let march_equinox_2003 = MarchEquinox::new(2003);
println!("{:#?}", march_equinox_2003);
// Ok(
    // MarchEquinox {
        // julian_day: 2452719.536962585,
        // date_time: 2003-03-21T00:53:13Z,
    // },
// )
```
