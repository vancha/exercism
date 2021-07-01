//I'll assume this module is imported and the user has no access to it
pub mod foo {
    use std::fmt;
    use std::fmt::*;

    const HOURS_IN_DAY: i32 = 24;
    const SEC_IN_MINUTE: i32 = 60;

    #[derive(Debug, PartialEq)]
    pub struct Clock {
        pub hours: i32,
        pub minutes: i32,
        _secret: (),//this will disallow a user to use struct literal instantiation from outside of this module
    }

    impl Clock {
        ///this is the method I expect a user to be creating the clock with
        pub fn new(hours: i32, minutes: i32) -> Self {
            let hours = hours + (minutes.wrapping_div_euclid(60));
            Clock {
                hours: hours.rem_euclid(HOURS_IN_DAY),
                minutes: minutes.rem_euclid(SEC_IN_MINUTE),
                _secret: (),
            }
        }
        ///creates a clone of a clock object, with `minutes` added to it.
        pub fn add_minutes(&self, minutes: i32) -> Self {
            Clock::new(self.hours, self.minutes + minutes)
        }
    }
    ///implemented to automatically get the ToString implemented as well
    impl fmt::Display for Clock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:02}:{:02}", &self.hours, &self.minutes)
        }
    }
}
//this will allow the user to make use of the clock struct
pub use foo::Clock;
