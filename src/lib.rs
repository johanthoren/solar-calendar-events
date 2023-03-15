use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time};

trait JulianDayNumber {
    fn to_five_decimals(&self) -> f64;
}

impl JulianDayNumber for f64 {
    fn to_five_decimals(&self) -> Self {
        let s = format!("{:.5}", self);
        s.parse().expect(&format!("Unable to parse f64: {}", self))
    }
}

pub trait OffsetDateTimeExt {
    fn from_julian_day(julian_day: f64) -> Self;
}

impl OffsetDateTimeExt for OffsetDateTime {
    fn from_julian_day(jdn: f64) -> OffsetDateTime {
        let j: f64 = jdn.to_five_decimals() + 0.5;
        let z: i32 = j as i32;
        let f: f64 = j - z as f64;
        let a: i32 = if z < 2_299_161 {
            z
        } else {
            let alpha: i32 = ((z as f64 - 1_867_216.25) / 36_524.25) as i32;
            z + 1 + (alpha - ((alpha as f64 / 4.0) as i32))
        };
        let b: i32 = a + 1_524;
        let c: i32 = ((b as f64 - 122.1) / 365.25) as i32;
        let d: i32 = (365.25 * c as f64) as i32;
        let e: i32 = ((b - d) as f64 / 30.6) as i32;
        let month_number: i32 = if e < 14 { e - 1 } else { e - 13 };
        let month = match month_number {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Invalid month number"),
        };
        let year: i32 = if month_number > 2 {
            c - 4_716
        } else {
            c - 4_715
        };
        let day_with_decimal: f64 =
            f + (b as f64 - d as f64 - ((e as f64 * 30.600_1) as i32 as f64));
        let day: u8 = day_with_decimal as u8;
        let fraction_of_day: f64 = day_with_decimal - day as f64;
        let hour_with_decimal: f64 = 24.0 * fraction_of_day;
        let mut hour: u8 = hour_with_decimal as u8;
        let fraction_of_hour: f64 = (hour_with_decimal - hour as f64).to_five_decimals();
        let minute_with_decimal: f64 = 60.0 * fraction_of_hour;
        let mut minute: u8 = minute_with_decimal as u8;
        let fraction_of_minute: f64 = 0.01 + minute_with_decimal - minute as f64;
        let mut second: u8 = (60.0 * fraction_of_minute) as u8;
        let mut move_day_forward = false;
        if second == 60 {
            minute += 1;
            second = 0;
        }
        if minute == 60 {
            hour += 1;
            minute = 0;
        }
        if hour == 24 {
            hour = 0;
            move_day_forward = true;
        }
        PrimitiveDateTime::new(
            if move_day_forward {
                Date::from_calendar_date(year, month, day).expect("Unable to set the date!")
                    + Duration::days(1)
            } else {
                Date::from_calendar_date(year, month, day).expect("Unable to set the date!")
            },
            Time::from_hms(hour, minute, second).expect("Unable to set the time!"),
        )
        .assume_utc()
    }
}

#[derive(Debug)]
pub struct MarchEquinox {
    year: i32,
    julian_day: Option<f64>,
    date_time: Option<OffsetDateTime>,
}

impl MarchEquinox {
    fn calculate_julian_day(&self) -> Option<f64> {
        let m = (self.year as f64 - 2_000.0) / 1_000.0;
        let m2 = m * m;
        let m3 = m2 * m;
        let m4 = m3 * m;
        Some(
            (2_451_623.809_84 + 365_242.374_04 * m + 0.051_69 * m2 - 0.004_11 * m3)
                - (0.000_57 * m4),
        )
    }

    /// Returns an Option containing a new MarchEquinox struct a year within the range 1900 to 2100.
    /// A year outside the range 1900-2100 will return None.
    pub fn new(year: i32) -> Option<MarchEquinox> {
        if year < 1_900 || year > 2_100 {
            return None;
        }
        let mut event = MarchEquinox {
            year,
            julian_day: None,
            date_time: None,
        };
        event.julian_day = event.calculate_julian_day();
        event.date_time = Some(OffsetDateTime::from_julian_day(
            event
                .julian_day
                .expect("Unable to calculate the Julian Day!"),
        ));
        Some(event)
    }

    /// Returns the date and time of the March Equinox as an Option<OffsetDateTime>.
    pub fn date_time(&self) -> Option<OffsetDateTime> {
        self.date_time
    }
}

#[derive(Debug)]
pub struct JuneSolstice {
    year: i32,
    julian_day: Option<f64>,
    date_time: Option<OffsetDateTime>,
}

impl JuneSolstice {
    fn calculate_julian_day(&self) -> Option<f64> {
        let m = (self.year as f64 - 2_000.0) / 1_000.0;
        let m2 = m * m;
        let m3 = m2 * m;
        let m4 = m3 * m;
        Some(
            (2_451_716.567_67 + 365_241.626_03 * m + 0.003_25 * m2 + 0.008_88 * m3)
                - (0.000_30 * m4),
        )
    }

    /// Returns an Option containing a new JuneSolstice struct for a year within the range 1900 to
    /// 2100. A year outside the range 1900-2100 will return None.
    pub fn new(year: i32) -> Option<JuneSolstice> {
        if year < 1_900 || year > 2_100 {
            return None;
        }
        let mut event = JuneSolstice {
            year,
            julian_day: None,
            date_time: None,
        };
        event.julian_day = event.calculate_julian_day();
        event.date_time = Some(OffsetDateTime::from_julian_day(
            event
                .julian_day
                .expect("Unable to calculate the Julian Day!"),
        ));
        Some(event)
    }

    /// Returns the date and time of the June Solstice as an Option<OffsetDateTime>.
    pub fn date_time(&self) -> Option<OffsetDateTime> {
        self.date_time
    }
}

#[derive(Debug)]
pub struct SeptemberEquinox {
    year: i32,
    julian_day: Option<f64>,
    date_time: Option<OffsetDateTime>,
}

impl SeptemberEquinox {
    fn calculate_julian_day(&self) -> Option<f64> {
        let m = (self.year as f64 - 2_000.0) / 1_000.0;
        let m2 = m * m;
        let m3 = m2 * m;
        let m4 = m3 * m;
        Some(
            (2_451_810.217_15 + 365_242.017_67 * m + 0.003_37 * m3 - 0.000_78 * m4)
                - (0.115_75 * m2),
        )
    }

    /// Returns an Option containing a new SeptemberEquinox struct for a year within the range 1900
    /// to 2100. A year outside the range 1900-2100 will return None.
    pub fn new(year: i32) -> Option<SeptemberEquinox> {
        if year < 1_900 || year > 2_100 {
            return None;
        }
        let mut event = SeptemberEquinox {
            year,
            julian_day: None,
            date_time: None,
        };
        event.julian_day = event.calculate_julian_day();
        event.date_time = Some(OffsetDateTime::from_julian_day(
            event
                .julian_day
                .expect("Unable to calculate the Julian Day!"),
        ));
        Some(event)
    }

    /// Returns the date and time of the September Equinox as an Option<OffsetDateTime>.
    pub fn date_time(&self) -> Option<OffsetDateTime> {
        self.date_time
    }
}

#[derive(Debug)]
pub struct DecemberSolstice {
    year: i32,
    julian_day: Option<f64>,
    date_time: Option<OffsetDateTime>,
}

impl DecemberSolstice {
    fn calculate_julian_day(&self) -> Option<f64> {
        let m = (self.year as f64 - 2_000.0) / 1_000.0;
        let m2 = m * m;
        let m3 = m2 * m;
        let m4 = m3 * m;
        Some(
            (2_451_900.0595_2 + 365_242.7404_9 * m + 0.0003_2 * m4) - 0.0622_3 * m2 - 0.008_23 * m3,
        )
    }

    /// Returns an Option containing a new DecemberSolstice struct for a year within the range 1900
    /// to 2100. A year outside the range 1900-2100 will return None.
    pub fn new(year: i32) -> Option<DecemberSolstice> {
        if year < 1_900 || year > 2_100 {
            return None;
        }
        let mut event = DecemberSolstice {
            year,
            julian_day: None,
            date_time: None,
        };
        event.julian_day = event.calculate_julian_day();
        event.date_time = Some(OffsetDateTime::from_julian_day(
            event
                .julian_day
                .expect("Unable to calculate the Julian Day!"),
        ));
        Some(event)
    }

    /// Returns the date and time of the December Solstice as an Option<OffsetDateTime>.
    pub fn date_time(&self) -> Option<OffsetDateTime> {
        self.date_time
    }
}
