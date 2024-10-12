use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Utc};
use thiserror::Error;

/// Represents errors that can occur when calculating the date and time of an annual solar event.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum AnnualSolarEventError {
    /// Error when unable to create a valid date with the given year, month, and day.
    #[error("Unable to set the date: {0}")]
    InvalidDateError(i32, u32, u32),

    /// Error for an invalid month number.
    #[error("Invalid month number: {0}")]
    MonthOutOfRange(i32),

    /// Error when unable to create a valid NaiveTime object with the given hour, minute, and
    /// second.
    #[error("Unable to create NaiveTime object from hour {0}, minute {1}, second {2}")]
    NaiveTimeError(u32, u32, u32),

    /// Error for failing to parse a floating-point number.
    #[error("Unable to parse float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// Error when the specified year is out of range (1900–2100).
    #[error("Year out of range: {0}, must be between 1900 and 2100")]
    YearOutOfRange(i32),
}

/// Utility functions for internal calculations related to annual solar events.
mod time_utils {
    use super::{AnnualSolarEventError, JulianDayNumber};

    /// Calculates the month and year based on intermediate values from Julian Day calculations.
    ///
    /// # Arguments
    /// * `e` - The intermediate value `e` from Julian Day calculations.
    /// * `c` - The intermediate value `c` from Julian Day calculations.
    ///
    /// # Returns
    /// A tuple containing the month (u32) and year (i32).
    ///
    /// # Errors
    /// Returns an error if the month is out of range (1-12).
    pub fn calculate_month_and_year(e: i32, c: i32) -> Result<(u32, i32), AnnualSolarEventError> {
        let signed_month = if e < 14 { e - 1 } else { e - 13 };
        let month = match signed_month {
            1..=12 => signed_month as u32,
            _ => return Err(AnnualSolarEventError::MonthOutOfRange(signed_month)),
        };
        let year = if month > 2 { c - 4716 } else { c - 4715 };
        Ok((month, year))
    }

    /// Calculates the day and fraction of the day from intermediate values in Julian Day
    /// calculations.
    ///
    /// # Arguments
    /// * `f` - The fractional part of the Julian Day.
    /// * `b` - The intermediate value `b` from Julian Day calculations.
    /// * `d` - The intermediate value `d` from Julian Day calculations.
    /// * `e` - The intermediate value `e` from Julian Day calculations.
    ///
    /// # Returns
    /// A tuple containing the day (u32) and the fractional part of the day (f64).
    pub fn calculate_day(f: f64, b: i32, d: i32, e: i32) -> (u32, f64) {
        let day_with_decimal: f64 =
            f + (b as f64 - d as f64 - ((e as f64 * 30.600_1) as i32 as f64));
        let day: u32 = day_with_decimal as u32;
        let fraction_of_day: f64 = day_with_decimal - day as f64;
        (day, fraction_of_day)
    }

    /// Calculates the hour, minute, second, and determines if the day should move forward based
    /// on the fractional day.
    ///
    /// # Arguments
    /// * `fraction_of_day` - The fractional part of the day.
    ///
    /// # Returns
    /// A tuple containing the hour (u32), minute (u32), second (u32), and a boolean indicating if
    /// the day should move forward.
    ///
    /// # Errors
    /// Returns an error if the rounding or parsing fails.
    pub fn calculate_hour_minute_second(
        fraction_of_day: f64,
    ) -> Result<(u32, u32, u32, bool), AnnualSolarEventError> {
        let hour_with_decimal: f64 = 24.0 * fraction_of_day;
        let mut hour: u32 = hour_with_decimal as u32;
        let fraction_of_hour: f64 = (hour_with_decimal - hour as f64).to_five_decimals()?;
        let minute_with_decimal: f64 = 60.0 * fraction_of_hour;
        let mut minute: u32 = minute_with_decimal as u32;
        let fraction_of_minute: f64 = 0.01 + minute_with_decimal - minute as f64;
        let mut second: u32 = (60.0 * fraction_of_minute) as u32;
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

        Ok((hour, minute, second, move_day_forward))
    }
}

/// Trait for working with Julian Day numbers and converting them to DateTime<Utc>.
pub trait JulianDateTimeUtc {
    /// Converts a Julian Day number to a `DateTime<Utc>`.
    ///
    /// # Arguments
    /// * `julian_day` - The Julian Day number to convert.
    ///
    /// # Returns
    /// A `DateTime<Utc>` representing the date and time of the Julian Day number.
    ///
    /// # Errors
    /// Returns an error if the conversion fails due to invalid date or time components.
    fn from_julian_day(julian_day: f64) -> Result<Self, AnnualSolarEventError>
    where
        Self: Sized;
}

/// Trait representing the characteristics of an annual solar event (e.g., Equinox or Solstice).
pub trait AnnualSolarEvent {
    /// Creates an instance of the solar event for a given year.
    ///
    /// # Arguments
    /// * `year` - The year for which to calculate the solar event.
    ///
    /// # Returns
    /// An instance of the solar event for the specified year.
    ///
    /// # Errors
    /// Returns an error if the year is out of range (1900-2100) or if the date and time cannot be
    /// calculated.
    ///
    /// # Example
    /// ```
    /// use solar_calendar_events::{AnnualSolarEvent, MarchEquinox};
    ///
    /// let event = MarchEquinox::for_year(2021).unwrap();
    ///
    /// assert_eq!(event.year(), 2021);
    ///
    /// let out_of_range_event = MarchEquinox::for_year(1899);
    ///
    /// assert!(out_of_range_event.is_err());
    ///
    /// assert_eq!(
    ///     out_of_range_event.err(),
    ///     Some(solar_calendar_events::AnnualSolarEventError::YearOutOfRange(1899))
    /// );
    /// ```
    fn for_year(year: i32) -> Result<Self, AnnualSolarEventError>
    where
        Self: Sized;

    /// Returns the date and time of the solar event as a `DateTime<Utc>`.
    ///
    /// # Returns
    /// A `DateTime<Utc>` representing the date and time of the solar event.
    /// ```
    fn date_time(&self) -> DateTime<Utc>;

    /// Returns the Julian Day Number of the solar event.
    ///
    /// # Returns
    /// The Julian Day Number of the solar event as a floating-point number.
    /// ```
    fn julian_day(&self) -> f64;

    /// Returns the year for which the solar event is calculated.
    fn year(&self) -> i32;

    /// Validates whether the given year is within the valid range (1900-2100).
    ///
    /// # Arguments
    /// * `year` - The year to validate.
    ///
    /// # Returns
    /// An `Ok(())` if the year is within the valid range, otherwise an error.
    ///
    /// # Errors
    /// Returns an error if the year is out of range (1900-2100).
    fn year_in_range(year: i32) -> Result<(), AnnualSolarEventError> {
        if !(1_900..=2_100).contains(&year) {
            return Err(AnnualSolarEventError::YearOutOfRange(year));
        }
        Ok(())
    }

    /// Returns constants needed to calculate the Julian Day Number for the solar event.
    ///
    /// # Returns
    /// A tuple containing the base, factor, and coefficients for the Julian Day calculation.
    fn julian_day_constants() -> (f64, f64, f64, f64, f64);

    /// Calculates the Julian Day Number for the event in a given year.
    ///
    /// # Arguments
    /// * `year` - The year for which to calculate the Julian Day Number.
    ///
    /// # Returns
    /// The Julian Day Number as a floating-point number for the event in the specified year.
    fn calculate_julian_day(year: i32) -> f64 {
        let (base, factor, m2_coeff, m3_coeff, m4_coeff) = Self::julian_day_constants();

        let m = (year as f64 - 2000.0) / 1000.0;
        let m2 = m * m;
        let m3 = m2 * m;
        let m4 = m3 * m;

        let f: f64 = base + factor * m + m2_coeff * m2 + m3_coeff * m3 + m4_coeff * m4;

        match f.to_five_decimals() {
            Ok(jd) => jd,
            Err(_) => f,
        }
    }

    /// Converts a Julian Day number to a `DateTime<Utc>`.
    ///
    /// # Arguments
    /// * `jd` - The Julian Day number to convert.
    ///
    /// # Returns
    /// A `DateTime<Utc>` representing the date and time of the Julian Day number.
    ///
    /// # Errors
    /// Returns an error if the conversion fails due to invalid date or time components.
    fn utc_from_julian(jd: f64) -> Result<DateTime<Utc>, AnnualSolarEventError> {
        DateTime::<Utc>::from_julian_day(jd)
    }
}

/// Trait for working with floating-point numbers to round them to five decimal places.
trait JulianDayNumber {
    /// Rounds the value to five decimal places.
    ///
    /// Returns an error if the rounding or parsing fails.
    fn to_five_decimals(&self) -> Result<f64, AnnualSolarEventError>;
}

impl JulianDayNumber for f64 {
    fn to_five_decimals(&self) -> Result<Self, AnnualSolarEventError> {
        let s = format!("{:.5}", self);
        s.parse().map_err(AnnualSolarEventError::ParseFloatError)
    }
}

impl JulianDateTimeUtc for DateTime<Utc> {
    /// Converts a Julian Day number to a `DateTime<Utc>`.
    ///
    /// Returns an error if the conversion fails due to invalid date or time components.
    fn from_julian_day(jdn: f64) -> Result<DateTime<Utc>, AnnualSolarEventError> {
        let j: f64 = jdn.to_five_decimals()? + 0.5;
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
        let (month, year) = time_utils::calculate_month_and_year(e, c)?;
        let (day, fraction_of_day) = time_utils::calculate_day(f, b, d, e);
        let (hour, minute, second, move_day_forward) =
            time_utils::calculate_hour_minute_second(fraction_of_day)?;

        let naive_date = match NaiveDate::from_ymd_opt(year, month, day) {
            Some(d) => {
                if move_day_forward {
                    d + TimeDelta::days(1)
                } else {
                    d
                }
            }
            None => return Err(AnnualSolarEventError::InvalidDateError(year, month, day)),
        };

        let naive_time: NaiveTime = match NaiveTime::from_hms_opt(hour, minute, second) {
            Some(t) => t,
            None => return Err(AnnualSolarEventError::NaiveTimeError(hour, minute, second)),
        };

        Ok(DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(naive_date, naive_time),
            Utc,
        ))
    }
}

/// Represents the March Equinox for a specific year.
#[derive(Debug)]
pub struct MarchEquinox {
    julian_day: f64,
    date_time: DateTime<Utc>,
}

impl AnnualSolarEvent for MarchEquinox {
    fn for_year(year: i32) -> Result<Self, AnnualSolarEventError> {
        Self::year_in_range(year)?;
        let julian_day = Self::calculate_julian_day(year);
        let date_time = Self::utc_from_julian(julian_day)?;
        Ok(Self {
            julian_day,
            date_time,
        })
    }

    fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    fn julian_day(&self) -> f64 {
        self.julian_day
    }

    fn year(&self) -> i32 {
        self.date_time.year()
    }

    fn julian_day_constants() -> (f64, f64, f64, f64, f64) {
        (
            2_451_623.809_84,
            365_242.374_04,
            0.051_69,
            -0.004_11,
            -0.000_57,
        )
    }
}

/// Represents the June Solstice for a specific year.
#[derive(Debug)]
pub struct JuneSolstice {
    julian_day: f64,
    date_time: DateTime<Utc>,
}

impl AnnualSolarEvent for JuneSolstice {
    fn for_year(year: i32) -> Result<Self, AnnualSolarEventError> {
        Self::year_in_range(year)?;
        let julian_day = Self::calculate_julian_day(year);
        let date_time = Self::utc_from_julian(julian_day)?;
        Ok(Self {
            julian_day,
            date_time,
        })
    }

    fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    fn julian_day(&self) -> f64 {
        self.julian_day
    }

    fn year(&self) -> i32 {
        self.date_time.year()
    }

    fn julian_day_constants() -> (f64, f64, f64, f64, f64) {
        (
            2_451_716.567_67,
            365_241.626_03,
            0.003_25,
            0.008_88,
            0.000_30,
        )
    }
}

/// Represents the September Equinox for a specific year.
#[derive(Debug)]
pub struct SeptemberEquinox {
    julian_day: f64,
    date_time: DateTime<Utc>,
}

impl AnnualSolarEvent for SeptemberEquinox {
    fn for_year(year: i32) -> Result<Self, AnnualSolarEventError> {
        Self::year_in_range(year)?;
        let julian_day = Self::calculate_julian_day(year);
        let date_time = Self::utc_from_julian(julian_day)?;

        Ok(Self {
            julian_day,
            date_time,
        })
    }

    fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    fn julian_day(&self) -> f64 {
        self.julian_day
    }

    fn year(&self) -> i32 {
        self.date_time.year()
    }

    fn julian_day_constants() -> (f64, f64, f64, f64, f64) {
        (
            2_451_810.217_15,
            365_242.017_67,
            0.003_37,
            -0.000_78,
            -0.115_75,
        )
    }
}

/// Represents the December Solstice for a specific year.
#[derive(Debug)]
pub struct DecemberSolstice {
    julian_day: f64,
    date_time: DateTime<Utc>,
}

impl AnnualSolarEvent for DecemberSolstice {
    fn for_year(year: i32) -> Result<Self, AnnualSolarEventError> {
        Self::year_in_range(year)?;
        let julian_day = Self::calculate_julian_day(year);
        let date_time = Self::utc_from_julian(julian_day)?;

        Ok(Self {
            julian_day,
            date_time,
        })
    }

    fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    fn julian_day(&self) -> f64 {
        self.julian_day
    }

    fn year(&self) -> i32 {
        self.date_time.year()
    }

    fn julian_day_constants() -> (f64, f64, f64, f64, f64) {
        (
            2_451_900.059_52,
            365_242.740_49,
            0.000_32,
            -0.062_23,
            -0.008_23,
        )
    }
}

/// Contains all four solar events (March Equinox, June Solstice, September Equinox, and December
/// Solstice) for a given year.
#[derive(Debug)]
pub struct AnnualSolarEvents {
    march_equinox: MarchEquinox,
    june_solstice: JuneSolstice,
    september_equinox: SeptemberEquinox,
    december_solstice: DecemberSolstice,
}

impl AnnualSolarEvents {
    /// Creates a new `AnnualSolarEvents` instance for the specified year, which contains all four
    /// solar events.
    ///
    /// Returns an error if the year is outside the valid range.
    pub fn for_year(year: i32) -> Result<Self, AnnualSolarEventError> {
        Ok(Self {
            march_equinox: MarchEquinox::for_year(year)?,
            june_solstice: JuneSolstice::for_year(year)?,
            september_equinox: SeptemberEquinox::for_year(year)?,
            december_solstice: DecemberSolstice::for_year(year)?,
        })
    }

    /// Returns a reference to the March Equinox event.
    pub fn march_equinox(&self) -> &MarchEquinox {
        &self.march_equinox
    }

    /// Returns a reference to the June Solstice event.
    pub fn june_solstice(&self) -> &JuneSolstice {
        &self.june_solstice
    }

    /// Returns a reference to the September Equinox event.
    pub fn september_equinox(&self) -> &SeptemberEquinox {
        &self.september_equinox
    }

    /// Returns a reference to the December Solstice event.
    pub fn december_solstice(&self) -> &DecemberSolstice {
        &self.december_solstice
    }

    /// Returns the year of these solar events.
    pub fn year(&self) -> i32 {
        self.march_equinox.year()
    }
}
