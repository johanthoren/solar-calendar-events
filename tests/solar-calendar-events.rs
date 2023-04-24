#[cfg(test)]
mod tests {
    use solar_calendar_events::*;
    use time::{macros::datetime, Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

    #[test]
    fn test_events_before_1900_or_after_2100() {
        assert!(MarchEquinox::new(1899).is_none());
        assert!(MarchEquinox::new(2101).is_none());
        assert!(JuneSolstice::new(1899).is_none());
        assert!(JuneSolstice::new(2101).is_none());
        assert!(SeptemberEquinox::new(1899).is_none());
        assert!(SeptemberEquinox::new(2101).is_none());
        assert!(DecemberSolstice::new(1899).is_none());
        assert!(DecemberSolstice::new(2101).is_none());
    }

    #[test]
    fn date_time_from_julian_day_number_2451435_0_is_1999_09_13_12_0_0() {
        let result = OffsetDateTime::from_julian_day(2451435.0);
        let time = PrimitiveDateTime::new(
            Date::from_calendar_date(1999, Month::September, 13).unwrap(),
            Time::from_hms(12, 0, 0).unwrap(),
        )
        .assume_utc();

        assert_eq!(result, time);
    }

    #[test]
    fn date_time_from_julian_day_number_2455435_0_is_2010_08_26_12_0_0() {
        let result = OffsetDateTime::from_julian_day(2455435.0);
        let time = PrimitiveDateTime::new(
            Date::from_calendar_date(2010, Month::August, 26).unwrap(),
            Time::from_hms(12, 0, 0).unwrap(),
        )
        .assume_utc();

        assert_eq!(result, time);
    }

    #[test]
    fn date_time_from_julian_day_number_2415435_452_is_1901_02_19_22_50_53() {
        let result = OffsetDateTime::from_julian_day(2415435.452);
        let time = PrimitiveDateTime::new(
            Date::from_calendar_date(1901, Month::February, 19).unwrap(),
            Time::from_hms(22, 50, 53).unwrap(),
        )
        .assume_utc();

        assert_eq!(result, time);
    }

    // The following tests are some actual observations of the March equinox,
    // here we aim to be within 10 minutes.
    fn match_march_equinox_to_observation(year: i32, observation: OffsetDateTime) {
        let event = MarchEquinox::new(year).unwrap();
        let event_time = event.date_time().unwrap();
        let diff = (event_time - observation).whole_seconds();

        println!("{} - {} = {}", event_time, observation, diff);
        assert!(diff.abs() <= 600);
    }

    #[test]
    fn march_equinox_of_1995_is_within_10_minutes_of_observation() {
        match_march_equinox_to_observation(1995, datetime!(1995-03-21 02:16:00 UTC));
    }

    #[test]
    fn march_equinox_of_2000_is_within_10_minutes_of_observation() {
        match_march_equinox_to_observation(2000, datetime!(2000-03-20 07:36:00 UTC));
    }

    #[test]
    fn march_equinox_of_2005_is_within_10_minutes_of_observation() {
        match_march_equinox_to_observation(2005, datetime!(2005-03-20 12:35:00 UTC));
    }

    #[test]
    fn march_equinox_of_2010_is_within_10_minutes_of_observation() {
        match_march_equinox_to_observation(2010, datetime!(2010-03-20 17:33:00 UTC));
    }

    #[test]
    fn march_equinox_date_time_is_close_to_nasa_date_time() {
        let differences = NASA_MARCH_EQUINOXES_1900_2089
            .iter()
            .map(|nasa| {
                MarchEquinox::new(nasa.year())
                    .unwrap_or_else(|| panic!("Cannot calculate March equinox of {}", &nasa.year()))
                    .date_time()
                    .unwrap_or_else(|| panic!("Cannot calculate March equinox of {}", &nasa.year()))
                    - *nasa
            })
            .collect::<Vec<_>>();

        let average_difference = differences.iter().map(|d| d.whole_seconds()).sum::<i64>() as f64
            / differences.len() as f64;
        println!("Average difference in seconds: {}", average_difference);
        assert!(average_difference.abs() <= 300.0);

        for nasa_calculation in NASA_MARCH_EQUINOXES_1900_2089 {
            let event = MarchEquinox::new(nasa_calculation.year()).unwrap();
            let event_time = event.date_time().unwrap();
            let diff = (event_time - nasa_calculation).whole_seconds();

            println!("{} - {} = {}", event_time, nasa_calculation, diff);
            // The difference should be less than 20 minutes. Note that the NASA
            // numbers are not observations but another set of calculations.
            assert!(diff.abs() <= 1200);
        }
    }

    #[test]
    fn june_solstice_date_time_is_close_to_nasa_date_time() {
        let differences = NASA_JUNE_SOLSTICES_1900_2089
            .iter()
            .map(|nasa| {
                JuneSolstice::new(nasa.year())
                    .unwrap_or_else(|| panic!("Cannot calculate June Solstice of {}", &nasa.year()))
                    .date_time()
                    .unwrap()
                    - *nasa
            })
            .collect::<Vec<_>>();

        let average_difference = differences.iter().map(|d| d.whole_seconds()).sum::<i64>() as f64
            / differences.len() as f64;
        println!("Average difference in seconds: {}", average_difference);
        assert!(average_difference.abs() <= 300.0);

        for nasa_calculation in NASA_JUNE_SOLSTICES_1900_2089 {
            let event = JuneSolstice::new(nasa_calculation.year()).unwrap();
            let event_time = event.date_time().unwrap();
            let diff = (event_time - nasa_calculation).whole_seconds();

            println!("{} - {} = {}", event_time, nasa_calculation, diff);
            // The difference should be less than 24 minutes. Note that the NASA
            // numbers are not observations but another set of calculations.
            // The accurracy of the SummerSolstice calculations seem to be just
            // a bit worse than the other calculations.
            assert!(diff.abs() <= 1440);
        }
    }

    #[test]
    fn september_equinox_date_time_is_close_to_nasa_date_time() {
        let differences = NASA_SEPTEMBER_EQUINOXES_1900_2089
            .iter()
            .map(|nasa| {
                SeptemberEquinox::new(nasa.year())
                    .unwrap_or_else(|| {
                        panic!("Cannot calculate September Equinox of {}", &nasa.year())
                    })
                    .date_time()
                    .unwrap()
                    - *nasa
            })
            .collect::<Vec<_>>();

        let average_difference = differences.iter().map(|d| d.whole_seconds()).sum::<i64>() as f64
            / differences.len() as f64;
        println!("Average difference in seconds: {}", average_difference);
        assert!(average_difference.abs() <= 300.0);

        for nasa_calculation in NASA_SEPTEMBER_EQUINOXES_1900_2089 {
            let event = SeptemberEquinox::new(nasa_calculation.year()).unwrap();
            let event_time = event.date_time().unwrap();
            let diff = (event_time - nasa_calculation).whole_seconds();

            println!("{} - {} = {}", event_time, nasa_calculation, diff);
            // The difference should be less than 20 minutes. Note that the NASA
            // numbers are not observations but another set of calculations.
            assert!(diff.abs() <= 1200);
        }
    }

    #[test]
    fn december_solstice_date_time_is_close_to_nasa_date_time() {
        let differences = NASA_DECEMBER_SOLSTICES_1900_2089
            .iter()
            .map(|nasa| {
                DecemberSolstice::new(nasa.year())
                    .unwrap_or_else(|| {
                        panic!("Cannot calculate December Solstice of {}", &nasa.year())
                    })
                    .date_time()
                    .unwrap()
                    - *nasa
            })
            .collect::<Vec<_>>();

        let average_difference = differences.iter().map(|d| d.whole_seconds()).sum::<i64>() as f64
            / differences.len() as f64;
        println!("Average difference in seconds: {}", average_difference);
        assert!(average_difference.abs() <= 300.0);

        for nasa_calculation in NASA_DECEMBER_SOLSTICES_1900_2089 {
            let event = DecemberSolstice::new(nasa_calculation.year()).unwrap();
            let event_time = event.date_time().unwrap();
            let diff = (event_time - nasa_calculation).whole_seconds();

            println!("{} - {} = {}", event_time, nasa_calculation, diff);
            // The difference should be less than 20 minutes. Note that the NASA
            // numbers are not observations but another set of calculations.
            assert!(diff.abs() <= 1200);
        }
    }

    // https://data.giss.nasa.gov/modelE/ar5plots/srvernal.html

    const NASA_MARCH_EQUINOXES_1900_2089: [OffsetDateTime; 190] = [
        datetime!(1900-03-21 01:30:00 UTC),
        datetime!(1901-03-21 07:19:00 UTC),
        datetime!(1902-03-21 13:08:00 UTC),
        datetime!(1903-03-21 18:58:00 UTC),
        datetime!(1904-03-21 00:47:00 UTC),
        datetime!(1905-03-21 06:36:00 UTC),
        datetime!(1906-03-21 12:25:00 UTC),
        datetime!(1907-03-21 18:14:00 UTC),
        datetime!(1908-03-21 00:04:00 UTC),
        datetime!(1909-03-21 05:53:00 UTC),
        datetime!(1910-03-21 11:42:00 UTC),
        datetime!(1911-03-21 17:31:00 UTC),
        datetime!(1912-03-20 23:20:00 UTC),
        datetime!(1913-03-21 05:10:00 UTC),
        datetime!(1914-03-21 10:59:00 UTC),
        datetime!(1915-03-21 16:48:00 UTC),
        datetime!(1916-03-20 22:37:00 UTC),
        datetime!(1917-03-21 04:26:00 UTC),
        datetime!(1918-03-21 10:16:00 UTC),
        datetime!(1919-03-21 16:05:00 UTC),
        datetime!(1920-03-20 21:54:00 UTC),
        datetime!(1921-03-21 03:43:00 UTC),
        datetime!(1922-03-21 09:32:00 UTC),
        datetime!(1923-03-21 15:22:00 UTC),
        datetime!(1924-03-20 21:11:00 UTC),
        datetime!(1925-03-21 03:00:00 UTC),
        datetime!(1926-03-21 08:49:00 UTC),
        datetime!(1927-03-21 14:38:00 UTC),
        datetime!(1928-03-20 20:28:00 UTC),
        datetime!(1929-03-21 02:17:00 UTC),
        datetime!(1930-03-21 08:06:00 UTC),
        datetime!(1931-03-21 13:55:00 UTC),
        datetime!(1932-03-20 19:44:00 UTC),
        datetime!(1933-03-21 01:34:00 UTC),
        datetime!(1934-03-21 07:23:00 UTC),
        datetime!(1935-03-21 13:12:00 UTC),
        datetime!(1936-03-20 19:01:00 UTC),
        datetime!(1937-03-21 00:50:00 UTC),
        datetime!(1938-03-21 06:40:00 UTC),
        datetime!(1939-03-21 12:29:00 UTC),
        datetime!(1940-03-20 18:18:00 UTC),
        datetime!(1941-03-21 00:07:00 UTC),
        datetime!(1942-03-21 05:56:00 UTC),
        datetime!(1943-03-21 11:46:00 UTC),
        datetime!(1944-03-20 17:35:00 UTC),
        datetime!(1945-03-20 23:24:00 UTC),
        datetime!(1946-03-21 05:13:00 UTC),
        datetime!(1947-03-21 11:02:00 UTC),
        datetime!(1948-03-20 16:52:00 UTC),
        datetime!(1949-03-20 22:41:00 UTC),
        datetime!(1950-03-21 04:30:00 UTC),
        datetime!(1951-03-21 10:19:00 UTC),
        datetime!(1952-03-20 16:08:00 UTC),
        datetime!(1953-03-20 21:58:00 UTC),
        datetime!(1954-03-21 03:47:00 UTC),
        datetime!(1955-03-21 09:36:00 UTC),
        datetime!(1956-03-20 15:25:00 UTC),
        datetime!(1957-03-20 21:14:00 UTC),
        datetime!(1958-03-21 03:04:00 UTC),
        datetime!(1959-03-21 08:53:00 UTC),
        datetime!(1960-03-20 14:42:00 UTC),
        datetime!(1961-03-20 20:31:00 UTC),
        datetime!(1962-03-21 02:20:00 UTC),
        datetime!(1963-03-21 08:10:00 UTC),
        datetime!(1964-03-20 13:59:00 UTC),
        datetime!(1965-03-20 19:48:00 UTC),
        datetime!(1966-03-21 01:37:00 UTC),
        datetime!(1967-03-21 07:26:00 UTC),
        datetime!(1968-03-20 13:16:00 UTC),
        datetime!(1969-03-20 19:05:00 UTC),
        datetime!(1970-03-21 00:54:00 UTC),
        datetime!(1971-03-21 06:43:00 UTC),
        datetime!(1972-03-20 12:32:00 UTC),
        datetime!(1973-03-20 18:22:00 UTC),
        datetime!(1974-03-21 00:11:00 UTC),
        datetime!(1975-03-21 06:00:00 UTC),
        datetime!(1976-03-20 11:49:00 UTC),
        datetime!(1977-03-20 17:38:00 UTC),
        datetime!(1978-03-20 23:28:00 UTC),
        datetime!(1979-03-21 05:17:00 UTC),
        datetime!(1980-03-20 11:06:00 UTC),
        datetime!(1981-03-20 16:55:00 UTC),
        datetime!(1982-03-20 22:44:00 UTC),
        datetime!(1983-03-21 04:34:00 UTC),
        datetime!(1984-03-20 10:23:00 UTC),
        datetime!(1985-03-20 16:12:00 UTC),
        datetime!(1986-03-20 22:01:00 UTC),
        datetime!(1987-03-21 03:50:00 UTC),
        datetime!(1988-03-20 09:40:00 UTC),
        datetime!(1989-03-20 15:29:00 UTC),
        datetime!(1990-03-20 21:18:00 UTC),
        datetime!(1991-03-21 03:07:00 UTC),
        datetime!(1992-03-20 08:56:00 UTC),
        datetime!(1993-03-20 14:46:00 UTC),
        datetime!(1994-03-20 20:35:00 UTC),
        datetime!(1995-03-21 02:24:00 UTC),
        datetime!(1996-03-20 08:13:00 UTC),
        datetime!(1997-03-20 14:02:00 UTC),
        datetime!(1998-03-20 19:52:00 UTC),
        datetime!(1999-03-21 01:41:00 UTC),
        datetime!(2000-03-20 07:30:00 UTC),
        datetime!(2001-03-20 13:19:00 UTC),
        datetime!(2002-03-20 19:08:00 UTC),
        datetime!(2003-03-21 00:58:00 UTC),
        datetime!(2004-03-20 06:47:00 UTC),
        datetime!(2005-03-20 12:36:00 UTC),
        datetime!(2006-03-20 18:25:00 UTC),
        datetime!(2007-03-21 00:14:00 UTC),
        datetime!(2008-03-20 06:04:00 UTC),
        datetime!(2009-03-20 11:53:00 UTC),
        datetime!(2010-03-20 17:42:00 UTC),
        datetime!(2011-03-20 23:31:00 UTC),
        datetime!(2012-03-20 05:20:00 UTC),
        datetime!(2013-03-20 11:10:00 UTC),
        datetime!(2014-03-20 16:59:00 UTC),
        datetime!(2015-03-20 22:48:00 UTC),
        datetime!(2016-03-20 04:37:00 UTC),
        datetime!(2017-03-20 10:26:00 UTC),
        datetime!(2018-03-20 16:16:00 UTC),
        datetime!(2019-03-20 22:05:00 UTC),
        datetime!(2020-03-20 03:54:00 UTC),
        datetime!(2021-03-20 09:43:00 UTC),
        datetime!(2022-03-20 15:32:00 UTC),
        datetime!(2023-03-20 21:22:00 UTC),
        datetime!(2024-03-20 03:11:00 UTC),
        datetime!(2025-03-20 09:00:00 UTC),
        datetime!(2026-03-20 14:49:00 UTC),
        datetime!(2027-03-20 20:38:00 UTC),
        datetime!(2028-03-20 02:28:00 UTC),
        datetime!(2029-03-20 08:17:00 UTC),
        datetime!(2030-03-20 14:06:00 UTC),
        datetime!(2031-03-20 19:55:00 UTC),
        datetime!(2032-03-20 01:44:00 UTC),
        datetime!(2033-03-20 07:34:00 UTC),
        datetime!(2034-03-20 13:23:00 UTC),
        datetime!(2035-03-20 19:12:00 UTC),
        datetime!(2036-03-20 01:01:00 UTC),
        datetime!(2037-03-20 06:50:00 UTC),
        datetime!(2038-03-20 12:40:00 UTC),
        datetime!(2039-03-20 18:29:00 UTC),
        datetime!(2040-03-20 00:18:00 UTC),
        datetime!(2041-03-20 06:07:00 UTC),
        datetime!(2042-03-20 11:56:00 UTC),
        datetime!(2043-03-20 17:46:00 UTC),
        datetime!(2044-03-19 23:35:00 UTC),
        datetime!(2045-03-20 05:24:00 UTC),
        datetime!(2046-03-20 11:13:00 UTC),
        datetime!(2047-03-20 17:02:00 UTC),
        datetime!(2048-03-19 22:52:00 UTC),
        datetime!(2049-03-20 04:41:00 UTC),
        datetime!(2050-03-20 10:30:00 UTC),
        datetime!(2051-03-20 16:19:00 UTC),
        datetime!(2052-03-19 22:08:00 UTC),
        datetime!(2053-03-20 03:58:00 UTC),
        datetime!(2054-03-20 09:47:00 UTC),
        datetime!(2055-03-20 15:36:00 UTC),
        datetime!(2056-03-19 21:25:00 UTC),
        datetime!(2057-03-20 03:14:00 UTC),
        datetime!(2058-03-20 09:04:00 UTC),
        datetime!(2059-03-20 14:53:00 UTC),
        datetime!(2060-03-19 20:42:00 UTC),
        datetime!(2061-03-20 02:31:00 UTC),
        datetime!(2062-03-20 08:20:00 UTC),
        datetime!(2063-03-20 14:10:00 UTC),
        datetime!(2064-03-19 19:59:00 UTC),
        datetime!(2065-03-20 01:48:00 UTC),
        datetime!(2066-03-20 07:37:00 UTC),
        datetime!(2067-03-20 13:26:00 UTC),
        datetime!(2068-03-19 19:16:00 UTC),
        datetime!(2069-03-20 01:05:00 UTC),
        datetime!(2070-03-20 06:54:00 UTC),
        datetime!(2071-03-20 12:43:00 UTC),
        datetime!(2072-03-19 18:32:00 UTC),
        datetime!(2073-03-20 00:22:00 UTC),
        datetime!(2074-03-20 06:11:00 UTC),
        datetime!(2075-03-20 12:00:00 UTC),
        datetime!(2076-03-19 17:49:00 UTC),
        datetime!(2077-03-19 23:38:00 UTC),
        datetime!(2078-03-20 05:28:00 UTC),
        datetime!(2079-03-20 11:17:00 UTC),
        datetime!(2080-03-19 17:06:00 UTC),
        datetime!(2081-03-19 22:55:00 UTC),
        datetime!(2082-03-20 04:44:00 UTC),
        datetime!(2083-03-20 10:34:00 UTC),
        datetime!(2084-03-19 16:23:00 UTC),
        datetime!(2085-03-19 22:12:00 UTC),
        datetime!(2086-03-20 04:01:00 UTC),
        datetime!(2087-03-20 09:50:00 UTC),
        datetime!(2088-03-19 15:40:00 UTC),
        datetime!(2089-03-19 21:29:00 UTC),
    ];

    const NASA_JUNE_SOLSTICES_1900_2089: [OffsetDateTime; 190] = [
        datetime!(1900-06-21 21:30:00 UTC),
        datetime!(1901-06-22 03:18:00 UTC),
        datetime!(1902-06-22 09:06:00 UTC),
        datetime!(1903-06-22 14:54:00 UTC),
        datetime!(1904-06-21 20:43:00 UTC),
        datetime!(1905-06-22 02:31:00 UTC),
        datetime!(1906-06-22 08:19:00 UTC),
        datetime!(1907-06-22 14:07:00 UTC),
        datetime!(1908-06-21 19:55:00 UTC),
        datetime!(1909-06-22 01:43:00 UTC),
        datetime!(1910-06-22 07:31:00 UTC),
        datetime!(1911-06-22 13:20:00 UTC),
        datetime!(1912-06-21 19:08:00 UTC),
        datetime!(1913-06-22 00:56:00 UTC),
        datetime!(1914-06-22 06:44:00 UTC),
        datetime!(1915-06-22 12:32:00 UTC),
        datetime!(1916-06-21 18:20:00 UTC),
        datetime!(1917-06-22 00:08:00 UTC),
        datetime!(1918-06-22 05:57:00 UTC),
        datetime!(1919-06-22 11:45:00 UTC),
        datetime!(1920-06-21 17:33:00 UTC),
        datetime!(1921-06-21 23:21:00 UTC),
        datetime!(1922-06-22 05:09:00 UTC),
        datetime!(1923-06-22 10:57:00 UTC),
        datetime!(1924-06-21 16:45:00 UTC),
        datetime!(1925-06-21 22:34:00 UTC),
        datetime!(1926-06-22 04:22:00 UTC),
        datetime!(1927-06-22 10:10:00 UTC),
        datetime!(1928-06-21 15:58:00 UTC),
        datetime!(1929-06-21 21:46:00 UTC),
        datetime!(1930-06-22 03:34:00 UTC),
        datetime!(1931-06-22 09:22:00 UTC),
        datetime!(1932-06-21 15:11:00 UTC),
        datetime!(1933-06-21 20:59:00 UTC),
        datetime!(1934-06-22 02:47:00 UTC),
        datetime!(1935-06-22 08:35:00 UTC),
        datetime!(1936-06-21 14:23:00 UTC),
        datetime!(1937-06-21 20:11:00 UTC),
        datetime!(1938-06-22 01:59:00 UTC),
        datetime!(1939-06-22 07:48:00 UTC),
        datetime!(1940-06-21 13:36:00 UTC),
        datetime!(1941-06-21 19:24:00 UTC),
        datetime!(1942-06-22 01:12:00 UTC),
        datetime!(1943-06-22 07:00:00 UTC),
        datetime!(1944-06-21 12:48:00 UTC),
        datetime!(1945-06-21 18:36:00 UTC),
        datetime!(1946-06-22 00:25:00 UTC),
        datetime!(1947-06-22 06:13:00 UTC),
        datetime!(1948-06-21 12:01:00 UTC),
        datetime!(1949-06-21 17:49:00 UTC),
        datetime!(1950-06-21 23:37:00 UTC),
        datetime!(1951-06-22 05:25:00 UTC),
        datetime!(1952-06-21 11:13:00 UTC),
        datetime!(1953-06-21 17:01:00 UTC),
        datetime!(1954-06-21 22:50:00 UTC),
        datetime!(1955-06-22 04:38:00 UTC),
        datetime!(1956-06-21 10:26:00 UTC),
        datetime!(1957-06-21 16:14:00 UTC),
        datetime!(1958-06-21 22:02:00 UTC),
        datetime!(1959-06-22 03:50:00 UTC),
        datetime!(1960-06-21 09:38:00 UTC),
        datetime!(1961-06-21 15:27:00 UTC),
        datetime!(1962-06-21 21:15:00 UTC),
        datetime!(1963-06-22 03:03:00 UTC),
        datetime!(1964-06-21 08:51:00 UTC),
        datetime!(1965-06-21 14:39:00 UTC),
        datetime!(1966-06-21 20:27:00 UTC),
        datetime!(1967-06-22 02:15:00 UTC),
        datetime!(1968-06-21 08:03:00 UTC),
        datetime!(1969-06-21 13:52:00 UTC),
        datetime!(1970-06-21 19:40:00 UTC),
        datetime!(1971-06-22 01:28:00 UTC),
        datetime!(1972-06-21 07:16:00 UTC),
        datetime!(1973-06-21 13:04:00 UTC),
        datetime!(1974-06-21 18:52:00 UTC),
        datetime!(1975-06-22 00:40:00 UTC),
        datetime!(1976-06-21 06:29:00 UTC),
        datetime!(1977-06-21 12:17:00 UTC),
        datetime!(1978-06-21 18:05:00 UTC),
        datetime!(1979-06-21 23:53:00 UTC),
        datetime!(1980-06-21 05:41:00 UTC),
        datetime!(1981-06-21 11:29:00 UTC),
        datetime!(1982-06-21 17:17:00 UTC),
        datetime!(1983-06-21 23:05:00 UTC),
        datetime!(1984-06-21 04:54:00 UTC),
        datetime!(1985-06-21 10:42:00 UTC),
        datetime!(1986-06-21 16:30:00 UTC),
        datetime!(1987-06-21 22:18:00 UTC),
        datetime!(1988-06-21 04:06:00 UTC),
        datetime!(1989-06-21 09:54:00 UTC),
        datetime!(1990-06-21 15:42:00 UTC),
        datetime!(1991-06-21 21:31:00 UTC),
        datetime!(1992-06-21 03:19:00 UTC),
        datetime!(1993-06-21 09:07:00 UTC),
        datetime!(1994-06-21 14:55:00 UTC),
        datetime!(1995-06-21 20:43:00 UTC),
        datetime!(1996-06-21 02:31:00 UTC),
        datetime!(1997-06-21 08:19:00 UTC),
        datetime!(1998-06-21 14:07:00 UTC),
        datetime!(1999-06-21 19:56:00 UTC),
        datetime!(2000-06-21 01:44:00 UTC),
        datetime!(2001-06-21 07:32:00 UTC),
        datetime!(2002-06-21 13:20:00 UTC),
        datetime!(2003-06-21 19:08:00 UTC),
        datetime!(2004-06-21 00:56:00 UTC),
        datetime!(2005-06-21 06:44:00 UTC),
        datetime!(2006-06-21 12:32:00 UTC),
        datetime!(2007-06-21 18:21:00 UTC),
        datetime!(2008-06-21 00:09:00 UTC),
        datetime!(2009-06-21 05:57:00 UTC),
        datetime!(2010-06-21 11:45:00 UTC),
        datetime!(2011-06-21 17:33:00 UTC),
        datetime!(2012-06-20 23:21:00 UTC),
        datetime!(2013-06-21 05:09:00 UTC),
        datetime!(2014-06-21 10:57:00 UTC),
        datetime!(2015-06-21 16:46:00 UTC),
        datetime!(2016-06-20 22:34:00 UTC),
        datetime!(2017-06-21 04:22:00 UTC),
        datetime!(2018-06-21 10:10:00 UTC),
        datetime!(2019-06-21 15:58:00 UTC),
        datetime!(2020-06-20 21:46:00 UTC),
        datetime!(2021-06-21 03:34:00 UTC),
        datetime!(2022-06-21 09:23:00 UTC),
        datetime!(2023-06-21 15:11:00 UTC),
        datetime!(2024-06-20 20:59:00 UTC),
        datetime!(2025-06-21 02:47:00 UTC),
        datetime!(2026-06-21 08:35:00 UTC),
        datetime!(2027-06-21 14:23:00 UTC),
        datetime!(2028-06-20 20:11:00 UTC),
        datetime!(2029-06-21 01:59:00 UTC),
        datetime!(2030-06-21 07:48:00 UTC),
        datetime!(2031-06-21 13:36:00 UTC),
        datetime!(2032-06-20 19:24:00 UTC),
        datetime!(2033-06-21 01:12:00 UTC),
        datetime!(2034-06-21 07:00:00 UTC),
        datetime!(2035-06-21 12:48:00 UTC),
        datetime!(2036-06-20 18:36:00 UTC),
        datetime!(2037-06-21 00:24:00 UTC),
        datetime!(2038-06-21 06:13:00 UTC),
        datetime!(2039-06-21 12:01:00 UTC),
        datetime!(2040-06-20 17:49:00 UTC),
        datetime!(2041-06-20 23:37:00 UTC),
        datetime!(2042-06-21 05:25:00 UTC),
        datetime!(2043-06-21 11:13:00 UTC),
        datetime!(2044-06-20 17:01:00 UTC),
        datetime!(2045-06-20 22:49:00 UTC),
        datetime!(2046-06-21 04:37:00 UTC),
        datetime!(2047-06-21 10:26:00 UTC),
        datetime!(2048-06-20 16:14:00 UTC),
        datetime!(2049-06-20 22:02:00 UTC),
        datetime!(2050-06-21 03:50:00 UTC),
        datetime!(2051-06-21 09:38:00 UTC),
        datetime!(2052-06-20 15:26:00 UTC),
        datetime!(2053-06-20 21:14:00 UTC),
        datetime!(2054-06-21 03:02:00 UTC),
        datetime!(2055-06-21 08:51:00 UTC),
        datetime!(2056-06-20 14:39:00 UTC),
        datetime!(2057-06-20 20:27:00 UTC),
        datetime!(2058-06-21 02:15:00 UTC),
        datetime!(2059-06-21 08:03:00 UTC),
        datetime!(2060-06-20 13:51:00 UTC),
        datetime!(2061-06-20 19:39:00 UTC),
        datetime!(2062-06-21 01:27:00 UTC),
        datetime!(2063-06-21 07:16:00 UTC),
        datetime!(2064-06-20 13:04:00 UTC),
        datetime!(2065-06-20 18:52:00 UTC),
        datetime!(2066-06-21 00:40:00 UTC),
        datetime!(2067-06-21 06:28:00 UTC),
        datetime!(2068-06-20 12:16:00 UTC),
        datetime!(2069-06-20 18:04:00 UTC),
        datetime!(2070-06-20 23:52:00 UTC),
        datetime!(2071-06-21 05:41:00 UTC),
        datetime!(2072-06-20 11:29:00 UTC),
        datetime!(2073-06-20 17:17:00 UTC),
        datetime!(2074-06-20 23:05:00 UTC),
        datetime!(2075-06-21 04:53:00 UTC),
        datetime!(2076-06-20 10:41:00 UTC),
        datetime!(2077-06-20 16:29:00 UTC),
        datetime!(2078-06-20 22:17:00 UTC),
        datetime!(2079-06-21 04:05:00 UTC),
        datetime!(2080-06-20 09:54:00 UTC),
        datetime!(2081-06-20 15:42:00 UTC),
        datetime!(2082-06-20 21:30:00 UTC),
        datetime!(2083-06-21 03:18:00 UTC),
        datetime!(2084-06-20 09:06:00 UTC),
        datetime!(2085-06-20 14:54:00 UTC),
        datetime!(2086-06-20 20:42:00 UTC),
        datetime!(2087-06-21 02:30:00 UTC),
        datetime!(2088-06-20 08:19:00 UTC),
        datetime!(2089-06-20 14:07:00 UTC),
    ];

    const NASA_SEPTEMBER_EQUINOXES_1900_2089: [OffsetDateTime; 190] = [
        datetime!(1900-09-23 12:04:00 UTC),
        datetime!(1901-09-23 17:53:00 UTC),
        datetime!(1902-09-23 23:42:00 UTC),
        datetime!(1903-09-24 05:31:00 UTC),
        datetime!(1904-09-23 11:19:00 UTC),
        datetime!(1905-09-23 17:08:00 UTC),
        datetime!(1906-09-23 22:57:00 UTC),
        datetime!(1907-09-24 04:46:00 UTC),
        datetime!(1908-09-23 10:34:00 UTC),
        datetime!(1909-09-23 16:23:00 UTC),
        datetime!(1910-09-23 22:12:00 UTC),
        datetime!(1911-09-24 04:01:00 UTC),
        datetime!(1912-09-23 09:49:00 UTC),
        datetime!(1913-09-23 15:38:00 UTC),
        datetime!(1914-09-23 21:27:00 UTC),
        datetime!(1915-09-24 03:15:00 UTC),
        datetime!(1916-09-23 09:04:00 UTC),
        datetime!(1917-09-23 14:53:00 UTC),
        datetime!(1918-09-23 20:42:00 UTC),
        datetime!(1919-09-24 02:30:00 UTC),
        datetime!(1920-09-23 08:19:00 UTC),
        datetime!(1921-09-23 14:08:00 UTC),
        datetime!(1922-09-23 19:57:00 UTC),
        datetime!(1923-09-24 01:45:00 UTC),
        datetime!(1924-09-23 07:34:00 UTC),
        datetime!(1925-09-23 13:23:00 UTC),
        datetime!(1926-09-23 19:12:00 UTC),
        datetime!(1927-09-24 01:00:00 UTC),
        datetime!(1928-09-23 06:49:00 UTC),
        datetime!(1929-09-23 12:38:00 UTC),
        datetime!(1930-09-23 18:26:00 UTC),
        datetime!(1931-09-24 00:15:00 UTC),
        datetime!(1932-09-23 06:04:00 UTC),
        datetime!(1933-09-23 11:53:00 UTC),
        datetime!(1934-09-23 17:41:00 UTC),
        datetime!(1935-09-23 23:30:00 UTC),
        datetime!(1936-09-23 05:19:00 UTC),
        datetime!(1937-09-23 11:08:00 UTC),
        datetime!(1938-09-23 16:56:00 UTC),
        datetime!(1939-09-23 22:45:00 UTC),
        datetime!(1940-09-23 04:34:00 UTC),
        datetime!(1941-09-23 10:22:00 UTC),
        datetime!(1942-09-23 16:11:00 UTC),
        datetime!(1943-09-23 22:00:00 UTC),
        datetime!(1944-09-23 03:49:00 UTC),
        datetime!(1945-09-23 09:37:00 UTC),
        datetime!(1946-09-23 15:26:00 UTC),
        datetime!(1947-09-23 21:15:00 UTC),
        datetime!(1948-09-23 03:03:00 UTC),
        datetime!(1949-09-23 08:52:00 UTC),
        datetime!(1950-09-23 14:41:00 UTC),
        datetime!(1951-09-23 20:30:00 UTC),
        datetime!(1952-09-23 02:18:00 UTC),
        datetime!(1953-09-23 08:07:00 UTC),
        datetime!(1954-09-23 13:56:00 UTC),
        datetime!(1955-09-23 19:44:00 UTC),
        datetime!(1956-09-23 01:33:00 UTC),
        datetime!(1957-09-23 07:22:00 UTC),
        datetime!(1958-09-23 13:11:00 UTC),
        datetime!(1959-09-23 18:59:00 UTC),
        datetime!(1960-09-23 00:48:00 UTC),
        datetime!(1961-09-23 06:37:00 UTC),
        datetime!(1962-09-23 12:25:00 UTC),
        datetime!(1963-09-23 18:14:00 UTC),
        datetime!(1964-09-23 00:03:00 UTC),
        datetime!(1965-09-23 05:52:00 UTC),
        datetime!(1966-09-23 11:40:00 UTC),
        datetime!(1967-09-23 17:29:00 UTC),
        datetime!(1968-09-22 23:18:00 UTC),
        datetime!(1969-09-23 05:06:00 UTC),
        datetime!(1970-09-23 10:55:00 UTC),
        datetime!(1971-09-23 16:44:00 UTC),
        datetime!(1972-09-22 22:33:00 UTC),
        datetime!(1973-09-23 04:21:00 UTC),
        datetime!(1974-09-23 10:10:00 UTC),
        datetime!(1975-09-23 15:59:00 UTC),
        datetime!(1976-09-22 21:47:00 UTC),
        datetime!(1977-09-23 03:36:00 UTC),
        datetime!(1978-09-23 09:25:00 UTC),
        datetime!(1979-09-23 15:14:00 UTC),
        datetime!(1980-09-22 21:02:00 UTC),
        datetime!(1981-09-23 02:51:00 UTC),
        datetime!(1982-09-23 08:40:00 UTC),
        datetime!(1983-09-23 14:28:00 UTC),
        datetime!(1984-09-22 20:17:00 UTC),
        datetime!(1985-09-23 02:06:00 UTC),
        datetime!(1986-09-23 07:54:00 UTC),
        datetime!(1987-09-23 13:43:00 UTC),
        datetime!(1988-09-22 19:32:00 UTC),
        datetime!(1989-09-23 01:21:00 UTC),
        datetime!(1990-09-23 07:09:00 UTC),
        datetime!(1991-09-23 12:58:00 UTC),
        datetime!(1992-09-22 18:47:00 UTC),
        datetime!(1993-09-23 00:35:00 UTC),
        datetime!(1994-09-23 06:24:00 UTC),
        datetime!(1995-09-23 12:13:00 UTC),
        datetime!(1996-09-22 18:01:00 UTC),
        datetime!(1997-09-22 23:50:00 UTC),
        datetime!(1998-09-23 05:39:00 UTC),
        datetime!(1999-09-23 11:28:00 UTC),
        datetime!(2000-09-22 17:16:00 UTC),
        datetime!(2001-09-22 23:05:00 UTC),
        datetime!(2002-09-23 04:54:00 UTC),
        datetime!(2003-09-23 10:42:00 UTC),
        datetime!(2004-09-22 16:31:00 UTC),
        datetime!(2005-09-22 22:20:00 UTC),
        datetime!(2006-09-23 04:08:00 UTC),
        datetime!(2007-09-23 09:57:00 UTC),
        datetime!(2008-09-22 15:46:00 UTC),
        datetime!(2009-09-22 21:34:00 UTC),
        datetime!(2010-09-23 03:23:00 UTC),
        datetime!(2011-09-23 09:12:00 UTC),
        datetime!(2012-09-22 15:01:00 UTC),
        datetime!(2013-09-22 20:49:00 UTC),
        datetime!(2014-09-23 02:38:00 UTC),
        datetime!(2015-09-23 08:27:00 UTC),
        datetime!(2016-09-22 14:15:00 UTC),
        datetime!(2017-09-22 20:04:00 UTC),
        datetime!(2018-09-23 01:53:00 UTC),
        datetime!(2019-09-23 07:41:00 UTC),
        datetime!(2020-09-22 13:30:00 UTC),
        datetime!(2021-09-22 19:19:00 UTC),
        datetime!(2022-09-23 01:07:00 UTC),
        datetime!(2023-09-23 06:56:00 UTC),
        datetime!(2024-09-22 12:45:00 UTC),
        datetime!(2025-09-22 18:33:00 UTC),
        datetime!(2026-09-23 00:22:00 UTC),
        datetime!(2027-09-23 06:11:00 UTC),
        datetime!(2028-09-22 11:59:00 UTC),
        datetime!(2029-09-22 17:48:00 UTC),
        datetime!(2030-09-22 23:37:00 UTC),
        datetime!(2031-09-23 05:26:00 UTC),
        datetime!(2032-09-22 11:14:00 UTC),
        datetime!(2033-09-22 17:03:00 UTC),
        datetime!(2034-09-22 22:52:00 UTC),
        datetime!(2035-09-23 04:40:00 UTC),
        datetime!(2036-09-22 10:29:00 UTC),
        datetime!(2037-09-22 16:18:00 UTC),
        datetime!(2038-09-22 22:06:00 UTC),
        datetime!(2039-09-23 03:55:00 UTC),
        datetime!(2040-09-22 09:44:00 UTC),
        datetime!(2041-09-22 15:32:00 UTC),
        datetime!(2042-09-22 21:21:00 UTC),
        datetime!(2043-09-23 03:10:00 UTC),
        datetime!(2044-09-22 08:58:00 UTC),
        datetime!(2045-09-22 14:47:00 UTC),
        datetime!(2046-09-22 20:36:00 UTC),
        datetime!(2047-09-23 02:24:00 UTC),
        datetime!(2048-09-22 08:13:00 UTC),
        datetime!(2049-09-22 14:02:00 UTC),
        datetime!(2050-09-22 19:50:00 UTC),
        datetime!(2051-09-23 01:39:00 UTC),
        datetime!(2052-09-22 07:28:00 UTC),
        datetime!(2053-09-22 13:16:00 UTC),
        datetime!(2054-09-22 19:05:00 UTC),
        datetime!(2055-09-23 00:54:00 UTC),
        datetime!(2056-09-22 06:42:00 UTC),
        datetime!(2057-09-22 12:31:00 UTC),
        datetime!(2058-09-22 18:20:00 UTC),
        datetime!(2059-09-23 00:08:00 UTC),
        datetime!(2060-09-22 05:57:00 UTC),
        datetime!(2061-09-22 11:46:00 UTC),
        datetime!(2062-09-22 17:34:00 UTC),
        datetime!(2063-09-22 23:23:00 UTC),
        datetime!(2064-09-22 05:12:00 UTC),
        datetime!(2065-09-22 11:00:00 UTC),
        datetime!(2066-09-22 16:49:00 UTC),
        datetime!(2067-09-22 22:38:00 UTC),
        datetime!(2068-09-22 04:26:00 UTC),
        datetime!(2069-09-22 10:15:00 UTC),
        datetime!(2070-09-22 16:04:00 UTC),
        datetime!(2071-09-22 21:52:00 UTC),
        datetime!(2072-09-22 03:41:00 UTC),
        datetime!(2073-09-22 09:30:00 UTC),
        datetime!(2074-09-22 15:18:00 UTC),
        datetime!(2075-09-22 21:07:00 UTC),
        datetime!(2076-09-22 02:56:00 UTC),
        datetime!(2077-09-22 08:44:00 UTC),
        datetime!(2078-09-22 14:33:00 UTC),
        datetime!(2079-09-22 20:22:00 UTC),
        datetime!(2080-09-22 02:10:00 UTC),
        datetime!(2081-09-22 07:59:00 UTC),
        datetime!(2082-09-22 13:48:00 UTC),
        datetime!(2083-09-22 19:36:00 UTC),
        datetime!(2084-09-22 01:25:00 UTC),
        datetime!(2085-09-22 07:14:00 UTC),
        datetime!(2086-09-22 13:02:00 UTC),
        datetime!(2087-09-22 18:51:00 UTC),
        datetime!(2088-09-22 00:39:00 UTC),
        datetime!(2089-09-22 06:28:00 UTC),
    ];

    const NASA_DECEMBER_SOLSTICES_1900_2089: [OffsetDateTime; 190] = [
        datetime!(1900-12-22 06:32:00 UTC),
        datetime!(1901-12-22 12:22:00 UTC),
        datetime!(1902-12-22 18:12:00 UTC),
        datetime!(1903-12-23 00:01:00 UTC),
        datetime!(1904-12-22 05:51:00 UTC),
        datetime!(1905-12-22 11:41:00 UTC),
        datetime!(1906-12-22 17:31:00 UTC),
        datetime!(1907-12-22 23:20:00 UTC),
        datetime!(1908-12-22 05:10:00 UTC),
        datetime!(1909-12-22 11:00:00 UTC),
        datetime!(1910-12-22 16:50:00 UTC),
        datetime!(1911-12-22 22:39:00 UTC),
        datetime!(1912-12-22 04:29:00 UTC),
        datetime!(1913-12-22 10:19:00 UTC),
        datetime!(1914-12-22 16:09:00 UTC),
        datetime!(1915-12-22 21:59:00 UTC),
        datetime!(1916-12-22 03:48:00 UTC),
        datetime!(1917-12-22 09:38:00 UTC),
        datetime!(1918-12-22 15:28:00 UTC),
        datetime!(1919-12-22 21:18:00 UTC),
        datetime!(1920-12-22 03:07:00 UTC),
        datetime!(1921-12-22 08:57:00 UTC),
        datetime!(1922-12-22 14:47:00 UTC),
        datetime!(1923-12-22 20:37:00 UTC),
        datetime!(1924-12-22 02:26:00 UTC),
        datetime!(1925-12-22 08:16:00 UTC),
        datetime!(1926-12-22 14:06:00 UTC),
        datetime!(1927-12-22 19:56:00 UTC),
        datetime!(1928-12-22 01:45:00 UTC),
        datetime!(1929-12-22 07:35:00 UTC),
        datetime!(1930-12-22 13:25:00 UTC),
        datetime!(1931-12-22 19:15:00 UTC),
        datetime!(1932-12-22 01:04:00 UTC),
        datetime!(1933-12-22 06:54:00 UTC),
        datetime!(1934-12-22 12:44:00 UTC),
        datetime!(1935-12-22 18:34:00 UTC),
        datetime!(1936-12-22 00:23:00 UTC),
        datetime!(1937-12-22 06:13:00 UTC),
        datetime!(1938-12-22 12:03:00 UTC),
        datetime!(1939-12-22 17:53:00 UTC),
        datetime!(1940-12-21 23:42:00 UTC),
        datetime!(1941-12-22 05:32:00 UTC),
        datetime!(1942-12-22 11:22:00 UTC),
        datetime!(1943-12-22 17:12:00 UTC),
        datetime!(1944-12-21 23:01:00 UTC),
        datetime!(1945-12-22 04:51:00 UTC),
        datetime!(1946-12-22 10:41:00 UTC),
        datetime!(1947-12-22 16:31:00 UTC),
        datetime!(1948-12-21 22:20:00 UTC),
        datetime!(1949-12-22 04:10:00 UTC),
        datetime!(1950-12-22 10:00:00 UTC),
        datetime!(1951-12-22 15:50:00 UTC),
        datetime!(1952-12-21 21:39:00 UTC),
        datetime!(1953-12-22 03:29:00 UTC),
        datetime!(1954-12-22 09:19:00 UTC),
        datetime!(1955-12-22 15:09:00 UTC),
        datetime!(1956-12-21 20:58:00 UTC),
        datetime!(1957-12-22 02:48:00 UTC),
        datetime!(1958-12-22 08:38:00 UTC),
        datetime!(1959-12-22 14:27:00 UTC),
        datetime!(1960-12-21 20:17:00 UTC),
        datetime!(1961-12-22 02:07:00 UTC),
        datetime!(1962-12-22 07:57:00 UTC),
        datetime!(1963-12-22 13:46:00 UTC),
        datetime!(1964-12-21 19:36:00 UTC),
        datetime!(1965-12-22 01:26:00 UTC),
        datetime!(1966-12-22 07:16:00 UTC),
        datetime!(1967-12-22 13:05:00 UTC),
        datetime!(1968-12-21 18:55:00 UTC),
        datetime!(1969-12-22 00:45:00 UTC),
        datetime!(1970-12-22 06:35:00 UTC),
        datetime!(1971-12-22 12:24:00 UTC),
        datetime!(1972-12-21 18:14:00 UTC),
        datetime!(1973-12-22 00:04:00 UTC),
        datetime!(1974-12-22 05:54:00 UTC),
        datetime!(1975-12-22 11:43:00 UTC),
        datetime!(1976-12-21 17:33:00 UTC),
        datetime!(1977-12-21 23:23:00 UTC),
        datetime!(1978-12-22 05:13:00 UTC),
        datetime!(1979-12-22 11:02:00 UTC),
        datetime!(1980-12-21 16:52:00 UTC),
        datetime!(1981-12-21 22:42:00 UTC),
        datetime!(1982-12-22 04:31:00 UTC),
        datetime!(1983-12-22 10:21:00 UTC),
        datetime!(1984-12-21 16:11:00 UTC),
        datetime!(1985-12-21 22:01:00 UTC),
        datetime!(1986-12-22 03:50:00 UTC),
        datetime!(1987-12-22 09:40:00 UTC),
        datetime!(1988-12-21 15:30:00 UTC),
        datetime!(1989-12-21 21:20:00 UTC),
        datetime!(1990-12-22 03:09:00 UTC),
        datetime!(1991-12-22 08:59:00 UTC),
        datetime!(1992-12-21 14:49:00 UTC),
        datetime!(1993-12-21 20:39:00 UTC),
        datetime!(1994-12-22 02:28:00 UTC),
        datetime!(1995-12-22 08:18:00 UTC),
        datetime!(1996-12-21 14:08:00 UTC),
        datetime!(1997-12-21 19:57:00 UTC),
        datetime!(1998-12-22 01:47:00 UTC),
        datetime!(1999-12-22 07:37:00 UTC),
        datetime!(2000-12-21 13:27:00 UTC),
        datetime!(2001-12-21 19:16:00 UTC),
        datetime!(2002-12-22 01:06:00 UTC),
        datetime!(2003-12-22 06:56:00 UTC),
        datetime!(2004-12-21 12:46:00 UTC),
        datetime!(2005-12-21 18:35:00 UTC),
        datetime!(2006-12-22 00:25:00 UTC),
        datetime!(2007-12-22 06:15:00 UTC),
        datetime!(2008-12-21 12:04:00 UTC),
        datetime!(2009-12-21 17:54:00 UTC),
        datetime!(2010-12-21 23:44:00 UTC),
        datetime!(2011-12-22 05:34:00 UTC),
        datetime!(2012-12-21 11:23:00 UTC),
        datetime!(2013-12-21 17:13:00 UTC),
        datetime!(2014-12-21 23:03:00 UTC),
        datetime!(2015-12-22 04:53:00 UTC),
        datetime!(2016-12-21 10:42:00 UTC),
        datetime!(2017-12-21 16:32:00 UTC),
        datetime!(2018-12-21 22:22:00 UTC),
        datetime!(2019-12-22 04:11:00 UTC),
        datetime!(2020-12-21 10:01:00 UTC),
        datetime!(2021-12-21 15:51:00 UTC),
        datetime!(2022-12-21 21:41:00 UTC),
        datetime!(2023-12-22 03:30:00 UTC),
        datetime!(2024-12-21 09:20:00 UTC),
        datetime!(2025-12-21 15:10:00 UTC),
        datetime!(2026-12-21 20:59:00 UTC),
        datetime!(2027-12-22 02:49:00 UTC),
        datetime!(2028-12-21 08:39:00 UTC),
        datetime!(2029-12-21 14:29:00 UTC),
        datetime!(2030-12-21 20:18:00 UTC),
        datetime!(2031-12-22 02:08:00 UTC),
        datetime!(2032-12-21 07:58:00 UTC),
        datetime!(2033-12-21 13:48:00 UTC),
        datetime!(2034-12-21 19:37:00 UTC),
        datetime!(2035-12-22 01:27:00 UTC),
        datetime!(2036-12-21 07:17:00 UTC),
        datetime!(2037-12-21 13:06:00 UTC),
        datetime!(2038-12-21 18:56:00 UTC),
        datetime!(2039-12-22 00:46:00 UTC),
        datetime!(2040-12-21 06:36:00 UTC),
        datetime!(2041-12-21 12:25:00 UTC),
        datetime!(2042-12-21 18:15:00 UTC),
        datetime!(2043-12-22 00:05:00 UTC),
        datetime!(2044-12-21 05:54:00 UTC),
        datetime!(2045-12-21 11:44:00 UTC),
        datetime!(2046-12-21 17:34:00 UTC),
        datetime!(2047-12-21 23:24:00 UTC),
        datetime!(2048-12-21 05:13:00 UTC),
        datetime!(2049-12-21 11:03:00 UTC),
        datetime!(2050-12-21 16:53:00 UTC),
        datetime!(2051-12-21 22:42:00 UTC),
        datetime!(2052-12-21 04:32:00 UTC),
        datetime!(2053-12-21 10:22:00 UTC),
        datetime!(2054-12-21 16:12:00 UTC),
        datetime!(2055-12-21 22:01:00 UTC),
        datetime!(2056-12-21 03:51:00 UTC),
        datetime!(2057-12-21 09:41:00 UTC),
        datetime!(2058-12-21 15:30:00 UTC),
        datetime!(2059-12-21 21:20:00 UTC),
        datetime!(2060-12-21 03:10:00 UTC),
        datetime!(2061-12-21 09:00:00 UTC),
        datetime!(2062-12-21 14:49:00 UTC),
        datetime!(2063-12-21 20:39:00 UTC),
        datetime!(2064-12-21 02:29:00 UTC),
        datetime!(2065-12-21 08:18:00 UTC),
        datetime!(2066-12-21 14:08:00 UTC),
        datetime!(2067-12-21 19:58:00 UTC),
        datetime!(2068-12-21 01:47:00 UTC),
        datetime!(2069-12-21 07:37:00 UTC),
        datetime!(2070-12-21 13:27:00 UTC),
        datetime!(2071-12-21 19:17:00 UTC),
        datetime!(2072-12-21 01:06:00 UTC),
        datetime!(2073-12-21 06:56:00 UTC),
        datetime!(2074-12-21 12:46:00 UTC),
        datetime!(2075-12-21 18:35:00 UTC),
        datetime!(2076-12-21 00:25:00 UTC),
        datetime!(2077-12-21 06:15:00 UTC),
        datetime!(2078-12-21 12:05:00 UTC),
        datetime!(2079-12-21 17:54:00 UTC),
        datetime!(2080-12-20 23:44:00 UTC),
        datetime!(2081-12-21 05:34:00 UTC),
        datetime!(2082-12-21 11:23:00 UTC),
        datetime!(2083-12-21 17:13:00 UTC),
        datetime!(2084-12-20 23:03:00 UTC),
        datetime!(2085-12-21 04:52:00 UTC),
        datetime!(2086-12-21 10:42:00 UTC),
        datetime!(2087-12-21 16:32:00 UTC),
        datetime!(2088-12-20 22:22:00 UTC),
        datetime!(2089-12-21 04:11:00 UTC),
    ];
}

// The data below is from nasa.gov:
//
// Solar Events in Greenwich Mean Time
//   Tropical Year = 365.2425 (days)

//          Vernal      Summer      Autumnal     Winter
//    Year     Equinox    Solstice     Equinox     Solstice   Perihelion    Aphelion
//    1900  3/21  1:30  6/21 21:30  9/23 12:04  12/22  6:32   1/02  0:04   7/03 14:58
//    1901  3/21  7:19  6/22  3:18  9/23 17:53  12/22 12:22   1/02  6:18   7/03 21:12
//    1902  3/21 13:08  6/22  9:06  9/23 23:42  12/22 18:12   1/02 12:32   7/04  3:26
//    1903  3/21 18:58  6/22 14:54  9/24  5:31  12/23  0:01   1/02 18:46   7/04  9:40
//    1904  3/21  0:47  6/21 20:43  9/23 11:19  12/22  5:51   1/03  1:00   7/03 15:54
//    1905  3/21  6:36  6/22  2:31  9/23 17:08  12/22 11:41   1/02  7:14   7/03 22:08
//    1906  3/21 12:25  6/22  8:19  9/23 22:57  12/22 17:31   1/02 13:28   7/04  4:22
//    1907  3/21 18:14  6/22 14:07  9/24  4:46  12/22 23:20   1/02 19:42   7/04 10:36
//    1908  3/21  0:04  6/21 19:55  9/23 10:34  12/22  5:10   1/03  1:55   7/03 16:50
//    1909  3/21  5:53  6/22  1:43  9/23 16:23  12/22 11:00   1/02  8:09   7/03 23:04
//    1910  3/21 11:42  6/22  7:31  9/23 22:12  12/22 16:50   1/02 14:23   7/04  5:18
//    1911  3/21 17:31  6/22 13:20  9/24  4:01  12/22 22:39   1/02 20:37   7/04 11:32
//    1912  3/20 23:20  6/21 19:08  9/23  9:49  12/22  4:29   1/03  2:51   7/03 17:46
//    1913  3/21  5:10  6/22  0:56  9/23 15:38  12/22 10:19   1/02  9:05   7/03 24:00
//    1914  3/21 10:59  6/22  6:44  9/23 21:27  12/22 16:09   1/02 15:19   7/04  6:14
//    1915  3/21 16:48  6/22 12:32  9/24  3:15  12/22 21:59   1/02 21:33   7/04 12:28
//    1916  3/20 22:37  6/21 18:20  9/23  9:04  12/22  3:48   1/03  3:47   7/03 18:42
//    1917  3/21  4:26  6/22  0:08  9/23 14:53  12/22  9:38   1/02 10:01   7/04  0:56
//    1918  3/21 10:16  6/22  5:57  9/23 20:42  12/22 15:28   1/02 16:15   7/04  7:10
//    1919  3/21 16:05  6/22 11:45  9/24  2:30  12/22 21:18   1/02 22:29   7/04 13:24
//    1920  3/20 21:54  6/21 17:33  9/23  8:19  12/22  3:07   1/03  4:43   7/03 19:38
//    1921  3/21  3:43  6/21 23:21  9/23 14:08  12/22  8:57   1/02 10:57   7/04  1:52
//    1922  3/21  9:32  6/22  5:09  9/23 19:57  12/22 14:47   1/02 17:11   7/04  8:06
//    1923  3/21 15:22  6/22 10:57  9/24  1:45  12/22 20:37   1/02 23:25   7/04 14:20
//    1924  3/20 21:11  6/21 16:45  9/23  7:34  12/22  2:26   1/03  5:39   7/03 20:34
//    1925  3/21  3:00  6/21 22:34  9/23 13:23  12/22  8:16   1/02 11:53   7/04  2:48
//    1926  3/21  8:49  6/22  4:22  9/23 19:12  12/22 14:06   1/02 18:07   7/04  9:02
//    1927  3/21 14:38  6/22 10:10  9/24  1:00  12/22 19:56   1/03  0:21   7/04 15:16
//    1928  3/20 20:28  6/21 15:58  9/23  6:49  12/22  1:45   1/03  6:35   7/03 21:30
//    1929  3/21  2:17  6/21 21:46  9/23 12:38  12/22  7:35   1/02 12:49   7/04  3:44
//    1930  3/21  8:06  6/22  3:34  9/23 18:26  12/22 13:25   1/02 19:03   7/04  9:58
//    1931  3/21 13:55  6/22  9:22  9/24  0:15  12/22 19:15   1/03  1:17   7/04 16:12
//    1932  3/20 19:44  6/21 15:11  9/23  6:04  12/22  1:04   1/03  7:31   7/03 22:25
//    1933  3/21  1:34  6/21 20:59  9/23 11:53  12/22  6:54   1/02 13:45   7/04  4:39
//    1934  3/21  7:23  6/22  2:47  9/23 17:41  12/22 12:44   1/02 19:59   7/04 10:53
//    1935  3/21 13:12  6/22  8:35  9/23 23:30  12/22 18:34   1/03  2:13   7/04 17:07
//    1936  3/20 19:01  6/21 14:23  9/23  5:19  12/22  0:23   1/03  8:27   7/03 23:21
//    1937  3/21  0:50  6/21 20:11  9/23 11:08  12/22  6:13   1/02 14:41   7/04  5:35
//    1938  3/21  6:40  6/22  1:59  9/23 16:56  12/22 12:03   1/02 20:55   7/04 11:49
//    1939  3/21 12:29  6/22  7:48  9/23 22:45  12/22 17:53   1/03  3:09   7/04 18:03
//    1940  3/20 18:18  6/21 13:36  9/23  4:34  12/21 23:42   1/03  9:23   7/04  0:17
//    1941  3/21  0:07  6/21 19:24  9/23 10:22  12/22  5:32   1/02 15:37   7/04  6:31
//    1942  3/21  5:56  6/22  1:12  9/23 16:11  12/22 11:22   1/02 21:51   7/04 12:45
//    1943  3/21 11:46  6/22  7:00  9/23 22:00  12/22 17:12   1/03  4:05   7/04 18:59
//    1944  3/20 17:35  6/21 12:48  9/23  3:49  12/21 23:01   1/03 10:19   7/04  1:13
//    1945  3/20 23:24  6/21 18:36  9/23  9:37  12/22  4:51   1/02 16:33   7/04  7:27
//    1946  3/21  5:13  6/22  0:25  9/23 15:26  12/22 10:41   1/02 22:46   7/04 13:41
//    1947  3/21 11:02  6/22  6:13  9/23 21:15  12/22 16:31   1/03  5:00   7/04 19:55
//    1948  3/20 16:52  6/21 12:01  9/23  3:03  12/21 22:20   1/03 11:14   7/04  2:09
//    1949  3/20 22:41  6/21 17:49  9/23  8:52  12/22  4:10   1/02 17:28   7/04  8:23
//    1950  3/21  4:30  6/21 23:37  9/23 14:41  12/22 10:00   1/02 23:42   7/04 14:37
//    1951  3/21 10:19  6/22  5:25  9/23 20:30  12/22 15:50   1/03  5:56   7/04 20:51
//    1952  3/20 16:08  6/21 11:13  9/23  2:18  12/21 21:39   1/03 12:10   7/04  3:05
//    1953  3/20 21:58  6/21 17:01  9/23  8:07  12/22  3:29   1/02 18:24   7/04  9:19
//    1954  3/21  3:47  6/21 22:50  9/23 13:56  12/22  9:19   1/03  0:38   7/04 15:33
//    1955  3/21  9:36  6/22  4:38  9/23 19:44  12/22 15:09   1/03  6:52   7/04 21:47
//    1956  3/20 15:25  6/21 10:26  9/23  1:33  12/21 20:58   1/03 13:06   7/04  4:01
//    1957  3/20 21:14  6/21 16:14  9/23  7:22  12/22  2:48   1/02 19:20   7/04 10:15
//    1958  3/21  3:04  6/21 22:02  9/23 13:11  12/22  8:38   1/03  1:34   7/04 16:29
//    1959  3/21  8:53  6/22  3:50  9/23 18:59  12/22 14:27   1/03  7:48   7/04 22:43
//    1960  3/20 14:42  6/21  9:38  9/23  0:48  12/21 20:17   1/03 14:02   7/04  4:57
//    1961  3/20 20:31  6/21 15:27  9/23  6:37  12/22  2:07   1/02 20:16   7/04 11:11
//    1962  3/21  2:20  6/21 21:15  9/23 12:25  12/22  7:57   1/03  2:30   7/04 17:25
//    1963  3/21  8:10  6/22  3:03  9/23 18:14  12/22 13:46   1/03  8:44   7/04 23:39
//    1964  3/20 13:59  6/21  8:51  9/23  0:03  12/21 19:36   1/03 14:58   7/04  5:53
//    1965  3/20 19:48  6/21 14:39  9/23  5:52  12/22  1:26   1/02 21:12   7/04 12:07
//    1966  3/21  1:37  6/21 20:27  9/23 11:40  12/22  7:16   1/03  3:26   7/04 18:21
//    1967  3/21  7:26  6/22  2:15  9/23 17:29  12/22 13:05   1/03  9:40   7/05  0:34
//    1968  3/20 13:16  6/21  8:03  9/22 23:18  12/21 18:55   1/03 15:54   7/04  6:48
//    1969  3/20 19:05  6/21 13:52  9/23  5:06  12/22  0:45   1/02 22:08   7/04 13:02
//    1970  3/21  0:54  6/21 19:40  9/23 10:55  12/22  6:35   1/03  4:22   7/04 19:16
//    1971  3/21  6:43  6/22  1:28  9/23 16:44  12/22 12:24   1/03 10:36   7/05  1:30
//    1972  3/20 12:32  6/21  7:16  9/22 22:33  12/21 18:14   1/03 16:50   7/04  7:44
//    1973  3/20 18:22  6/21 13:04  9/23  4:21  12/22  0:04   1/02 23:04   7/04 13:58
//    1974  3/21  0:11  6/21 18:52  9/23 10:10  12/22  5:54   1/03  5:18   7/04 20:12
//    1975  3/21  6:00  6/22  0:40  9/23 15:59  12/22 11:43   1/03 11:32   7/05  2:26
//    1976  3/20 11:49  6/21  6:29  9/22 21:47  12/21 17:33   1/03 17:46   7/04  8:40
//    1977  3/20 17:38  6/21 12:17  9/23  3:36  12/21 23:23   1/02 24:00   7/04 14:54
//    1978  3/20 23:28  6/21 18:05  9/23  9:25  12/22  5:13   1/03  6:14   7/04 21:08
//    1979  3/21  5:17  6/21 23:53  9/23 15:14  12/22 11:02   1/03 12:28   7/05  3:22
//    1980  3/20 11:06  6/21  5:41  9/22 21:02  12/21 16:52   1/03 18:41   7/04  9:36
//    1981  3/20 16:55  6/21 11:29  9/23  2:51  12/21 22:42   1/03  0:55   7/04 15:50
//    1982  3/20 22:44  6/21 17:17  9/23  8:40  12/22  4:31   1/03  7:09   7/04 22:04
//    1983  3/21  4:34  6/21 23:05  9/23 14:28  12/22 10:21   1/03 13:23   7/05  4:18
//    1984  3/20 10:23  6/21  4:54  9/22 20:17  12/21 16:11   1/03 19:37   7/04 10:32
//    1985  3/20 16:12  6/21 10:42  9/23  2:06  12/21 22:01   1/03  1:51   7/04 16:46
//    1986  3/20 22:01  6/21 16:30  9/23  7:54  12/22  3:50   1/03  8:05   7/04 23:00
//    1987  3/21  3:50  6/21 22:18  9/23 13:43  12/22  9:40   1/03 14:19   7/05  5:14
//    1988  3/20  9:40  6/21  4:06  9/22 19:32  12/21 15:30   1/03 20:33   7/04 11:28
//    1989  3/20 15:29  6/21  9:54  9/23  1:21  12/21 21:20   1/03  2:47   7/04 17:42
//    1990  3/20 21:18  6/21 15:42  9/23  7:09  12/22  3:09   1/03  9:01   7/04 23:56
//    1991  3/21  3:07  6/21 21:31  9/23 12:58  12/22  8:59   1/03 15:15   7/05  6:10
//    1992  3/20  8:56  6/21  3:19  9/22 18:47  12/21 14:49   1/03 21:29   7/04 12:24
//    1993  3/20 14:46  6/21  9:07  9/23  0:35  12/21 20:39   1/03  3:43   7/04 18:38
//    1994  3/20 20:35  6/21 14:55  9/23  6:24  12/22  2:28   1/03  9:57   7/05  0:52
//    1995  3/21  2:24  6/21 20:43  9/23 12:13  12/22  8:18   1/03 16:11   7/05  7:06
//    1996  3/20  8:13  6/21  2:31  9/22 18:01  12/21 14:08   1/03 22:25   7/04 13:20
//    1997  3/20 14:02  6/21  8:19  9/22 23:50  12/21 19:57   1/03  4:39   7/04 19:34
//    1998  3/20 19:52  6/21 14:07  9/23  5:39  12/22  1:47   1/03 10:53   7/05  1:47
//    1999  3/21  1:41  6/21 19:56  9/23 11:28  12/22  7:37   1/03 17:07   7/05  8:01
//    2000  3/20  7:30  6/21  1:44  9/22 17:16  12/21 13:27   1/03 23:21   7/04 14:15
//    2001  3/20 13:19  6/21  7:32  9/22 23:05  12/21 19:16   1/03  5:35   7/04 20:29
//    2002  3/20 19:08  6/21 13:20  9/23  4:54  12/22  1:06   1/03 11:49   7/05  2:43
//    2003  3/21  0:58  6/21 19:08  9/23 10:42  12/22  6:56   1/03 18:03   7/05  8:57
//    2004  3/20  6:47  6/21  0:56  9/22 16:31  12/21 12:46   1/04  0:17   7/04 15:11
//    2005  3/20 12:36  6/21  6:44  9/22 22:20  12/21 18:35   1/03  6:31   7/04 21:25
//    2006  3/20 18:25  6/21 12:32  9/23  4:08  12/22  0:25   1/03 12:45   7/05  3:39
//    2007  3/21  0:14  6/21 18:21  9/23  9:57  12/22  6:15   1/03 18:59   7/05  9:53
//    2008  3/20  6:04  6/21  0:09  9/22 15:46  12/21 12:04   1/04  1:13   7/04 16:07
//    2009  3/20 11:53  6/21  5:57  9/22 21:34  12/21 17:54   1/03  7:27   7/04 22:21
//    2010  3/20 17:42  6/21 11:45  9/23  3:23  12/21 23:44   1/03 13:40   7/05  4:35
//    2011  3/20 23:31  6/21 17:33  9/23  9:12  12/22  5:34   1/03 19:54   7/05 10:49
//    2012  3/20  5:20  6/20 23:21  9/22 15:01  12/21 11:23   1/04  2:08   7/04 17:03
//    2013  3/20 11:10  6/21  5:09  9/22 20:49  12/21 17:13   1/03  8:22   7/04 23:17
//    2014  3/20 16:59  6/21 10:57  9/23  2:38  12/21 23:03   1/03 14:36   7/05  5:31
//    2015  3/20 22:48  6/21 16:46  9/23  8:27  12/22  4:53   1/03 20:50   7/05 11:45
//    2016  3/20  4:37  6/20 22:34  9/22 14:15  12/21 10:42   1/04  3:04   7/04 17:59
//    2017  3/20 10:26  6/21  4:22  9/22 20:04  12/21 16:32   1/03  9:18   7/05  0:13
//    2018  3/20 16:16  6/21 10:10  9/23  1:53  12/21 22:22   1/03 15:32   7/05  6:27
//    2019  3/20 22:05  6/21 15:58  9/23  7:41  12/22  4:11   1/03 21:46   7/05 12:41
//    2020  3/20  3:54  6/20 21:46  9/22 13:30  12/21 10:01   1/04  4:00   7/04 18:55
//    2021  3/20  9:43  6/21  3:34  9/22 19:19  12/21 15:51   1/03 10:14   7/05  1:09
//    2022  3/20 15:32  6/21  9:23  9/23  1:07  12/21 21:41   1/03 16:28   7/05  7:23
//    2023  3/20 21:22  6/21 15:11  9/23  6:56  12/22  3:30   1/03 22:42   7/05 13:37
//    2024  3/20  3:11  6/20 20:59  9/22 12:45  12/21  9:20   1/04  4:56   7/04 19:51
//    2025  3/20  9:00  6/21  2:47  9/22 18:33  12/21 15:10   1/03 11:10   7/05  2:05
//    2026  3/20 14:49  6/21  8:35  9/23  0:22  12/21 20:59   1/03 17:24   7/05  8:19
//    2027  3/20 20:38  6/21 14:23  9/23  6:11  12/22  2:49   1/03 23:38   7/05 14:32
//    2028  3/20  2:28  6/20 20:11  9/22 11:59  12/21  8:39   1/04  5:52   7/04 20:46
//    2029  3/20  8:17  6/21  1:59  9/22 17:48  12/21 14:29   1/03 12:06   7/05  3:00
//    2030  3/20 14:06  6/21  7:48  9/22 23:37  12/21 20:18   1/03 18:20   7/05  9:14
//    2031  3/20 19:55  6/21 13:36  9/23  5:26  12/22  2:08   1/04  0:34   7/05 15:28
//    2032  3/20  1:44  6/20 19:24  9/22 11:14  12/21  7:58   1/04  6:48   7/04 21:42
//    2033  3/20  7:34  6/21  1:12  9/22 17:03  12/21 13:48   1/03 13:02   7/05  3:56
//    2034  3/20 13:23  6/21  7:00  9/22 22:52  12/21 19:37   1/03 19:16   7/05 10:10
//    2035  3/20 19:12  6/21 12:48  9/23  4:40  12/22  1:27   1/04  1:30   7/05 16:24
//    2036  3/20  1:01  6/20 18:36  9/22 10:29  12/21  7:17   1/04  7:44   7/04 22:38
//    2037  3/20  6:50  6/21  0:24  9/22 16:18  12/21 13:06   1/03 13:58   7/05  4:52
//    2038  3/20 12:40  6/21  6:13  9/22 22:06  12/21 18:56   1/03 20:11   7/05 11:06
//    2039  3/20 18:29  6/21 12:01  9/23  3:55  12/22  0:46   1/04  2:25   7/05 17:20
//    2040  3/20  0:18  6/20 17:49  9/22  9:44  12/21  6:36   1/04  8:39   7/04 23:34
//    2041  3/20  6:07  6/20 23:37  9/22 15:32  12/21 12:25   1/03 14:53   7/05  5:48
//    2042  3/20 11:56  6/21  5:25  9/22 21:21  12/21 18:15   1/03 21:07   7/05 12:02
//    2043  3/20 17:46  6/21 11:13  9/23  3:10  12/22  0:05   1/04  3:21   7/05 18:16
//    2044  3/19 23:35  6/20 17:01  9/22  8:58  12/21  5:54   1/04  9:35   7/05  0:30
//    2045  3/20  5:24  6/20 22:49  9/22 14:47  12/21 11:44   1/03 15:49   7/05  6:44
//    2046  3/20 11:13  6/21  4:37  9/22 20:36  12/21 17:34   1/03 22:03   7/05 12:58
//    2047  3/20 17:02  6/21 10:26  9/23  2:24  12/21 23:24   1/04  4:17   7/05 19:12
//    2048  3/19 22:52  6/20 16:14  9/22  8:13  12/21  5:13   1/04 10:31   7/05  1:26
//    2049  3/20  4:41  6/20 22:02  9/22 14:02  12/21 11:03   1/03 16:45   7/05  7:40
//    2050  3/20 10:30  6/21  3:50  9/22 19:50  12/21 16:53   1/03 22:59   7/05 13:54
//    2051  3/20 16:19  6/21  9:38  9/23  1:39  12/21 22:42   1/04  5:13   7/05 20:08
//    2052  3/19 22:08  6/20 15:26  9/22  7:28  12/21  4:32   1/04 11:27   7/05  2:22
//    2053  3/20  3:58  6/20 21:14  9/22 13:16  12/21 10:22   1/03 17:41   7/05  8:35
//    2054  3/20  9:47  6/21  3:02  9/22 19:05  12/21 16:12   1/03 23:55   7/05 14:49
//    2055  3/20 15:36  6/21  8:51  9/23  0:54  12/21 22:01   1/04  6:09   7/05 21:03
//    2056  3/19 21:25  6/20 14:39  9/22  6:42  12/21  3:51   1/04 12:23   7/05  3:17
//    2057  3/20  3:14  6/20 20:27  9/22 12:31  12/21  9:41   1/03 18:37   7/05  9:31
//    2058  3/20  9:04  6/21  2:15  9/22 18:20  12/21 15:30   1/04  0:51   7/05 15:45
//    2059  3/20 14:53  6/21  8:03  9/23  0:08  12/21 21:20   1/04  7:05   7/05 21:59
//    2060  3/19 20:42  6/20 13:51  9/22  5:57  12/21  3:10   1/04 13:19   7/05  4:13
//    2061  3/20  2:31  6/20 19:39  9/22 11:46  12/21  9:00   1/03 19:33   7/05 10:27
//    2062  3/20  8:20  6/21  1:27  9/22 17:34  12/21 14:49   1/04  1:47   7/05 16:41
//    2063  3/20 14:10  6/21  7:16  9/22 23:23  12/21 20:39   1/04  8:01   7/05 22:55
//    2064  3/19 19:59  6/20 13:04  9/22  5:12  12/21  2:29   1/04 14:14   7/05  5:09
//    2065  3/20  1:48  6/20 18:52  9/22 11:00  12/21  8:18   1/03 20:28   7/05 11:23
//    2066  3/20  7:37  6/21  0:40  9/22 16:49  12/21 14:08   1/04  2:42   7/05 17:37
//    2067  3/20 13:26  6/21  6:28  9/22 22:38  12/21 19:58   1/04  8:56   7/05 23:51
//    2068  3/19 19:16  6/20 12:16  9/22  4:26  12/21  1:47   1/04 15:10   7/05  6:05
//    2069  3/20  1:05  6/20 18:04  9/22 10:15  12/21  7:37   1/03 21:24   7/05 12:19
//    2070  3/20  6:54  6/20 23:52  9/22 16:04  12/21 13:27   1/04  3:38   7/05 18:33
//    2071  3/20 12:43  6/21  5:41  9/22 21:52  12/21 19:17   1/04  9:52   7/06  0:47
//    2072  3/19 18:32  6/20 11:29  9/22  3:41  12/21  1:06   1/04 16:06   7/05  7:01
//    2073  3/20  0:22  6/20 17:17  9/22  9:30  12/21  6:56   1/03 22:20   7/05 13:15
//    2074  3/20  6:11  6/20 23:05  9/22 15:18  12/21 12:46   1/04  4:34   7/05 19:29
//    2075  3/20 12:00  6/21  4:53  9/22 21:07  12/21 18:35   1/04 10:48   7/06  1:43
//    2076  3/19 17:49  6/20 10:41  9/22  2:56  12/21  0:25   1/04 17:02   7/05  7:57
//    2077  3/19 23:38  6/20 16:29  9/22  8:44  12/21  6:15   1/03 23:16   7/05 14:11
//    2078  3/20  5:28  6/20 22:17  9/22 14:33  12/21 12:05   1/04  5:30   7/05 20:24
//    2079  3/20 11:17  6/21  4:05  9/22 20:22  12/21 17:54   1/04 11:44   7/06  2:38
//    2080  3/19 17:06  6/20  9:54  9/22  2:10  12/20 23:44   1/04 17:58   7/05  8:52
//    2081  3/19 22:55  6/20 15:42  9/22  7:59  12/21  5:34   1/04  0:12   7/05 15:06
//    2082  3/20  4:44  6/20 21:30  9/22 13:48  12/21 11:23   1/04  6:26   7/05 21:20
//    2083  3/20 10:34  6/21  3:18  9/22 19:36  12/21 17:13   1/04 12:40   7/06  3:34
//    2084  3/19 16:23  6/20  9:06  9/22  1:25  12/20 23:03   1/04 18:54   7/05  9:48
//    2085  3/19 22:12  6/20 14:54  9/22  7:14  12/21  4:52   1/04  1:08   7/05 16:02
//    2086  3/20  4:01  6/20 20:42  9/22 13:02  12/21 10:42   1/04  7:22   7/05 22:16
//    2087  3/20  9:50  6/21  2:30  9/22 18:51  12/21 16:32   1/04 13:36   7/06  4:30
//    2088  3/19 15:40  6/20  8:19  9/22  0:39  12/20 22:22   1/04 19:49   7/05 10:44
//    2089  3/19 21:29  6/20 14:07  9/22  6:28  12/21  4:11   1/04  2:03   7/05 16:58
