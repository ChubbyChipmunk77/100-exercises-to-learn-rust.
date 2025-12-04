// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    valuearr: [Pair; 7],
}

pub struct Pair {
    week: Weekday,
    temp: Option<i32>,
}

#[derive(PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        Self {
            valuearr: [
                Pair {
                    week: Weekday::Monday,
                    temp: None,
                },
                Pair {
                    week: Weekday::Tuesday,
                    temp: None,
                },
                Pair {
                    week: Weekday::Wednesday,
                    temp: None,
                },
                Pair {
                    week: Weekday::Thursday,
                    temp: None,
                },
                Pair {
                    week: Weekday::Friday,
                    temp: None,
                },
                Pair {
                    week: Weekday::Saturday,
                    temp: None,
                },
                Pair {
                    week: Weekday::Sunday,
                    temp: None,
                },
            ],
        }
    }
    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        for i in 0..7 {
            if self.valuearr[i].week == day {
                return self.valuearr[i].temp;
            }
        }
        None
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        for i in 0..7 {
            if self.valuearr[i].week == day {
                self.valuearr[i].temp = Some(temperature);
                break;
            }
        }
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
