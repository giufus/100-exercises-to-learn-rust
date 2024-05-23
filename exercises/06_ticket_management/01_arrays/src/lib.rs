// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

#[derive(Default)]
pub struct WeekTemperatures {
    pub temperatures: [i32; 7],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
pub enum NumberError {
    InvalidNumber,
}

impl TryFrom<i32> for Weekday {
    type Error = NumberError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Weekday::Monday),
            1 => Ok(Weekday::Tuesday),
            2 => Ok(Weekday::Wednesday),
            3 => Ok(Weekday::Thursday),
            4 => Ok(Weekday::Friday),
            5 => Ok(Weekday::Saturday),
            6 => Ok(Weekday::Sunday),
            _ => Err(NumberError::InvalidNumber),
        }
    }
}

impl WeekTemperatures {
    pub fn new() -> Self {
        /*let temperatures = (0..7)
            .map(|d| Weekday::try_from(d).unwrap())
            .collect::<Vec<Weekday>>();*/
        WeekTemperatures {
            temperatures: [i32::MIN; 7],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        self.temperatures
            .get(day as usize)
            .map(|&d| if d != i32::MIN { Some(d) } else { None })
            .flatten()
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        self.temperatures[day as usize] = temperature;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
