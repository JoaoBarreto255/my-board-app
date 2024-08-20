use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Now,
}

impl Priority {
    pub fn code(&self) -> i32 {
        return match self {
            Priority::Low => 1,
            Priority::Normal => 2,
            Priority::High => 3,
            Priority::Now => 4,
        };
    }

    pub fn new(code: i32) -> Priority {
        return match code {
            2 => Priority::Normal,
            3 => Priority::High,
            4 => Priority::Now,
            _ => Priority::Low,
        };
    }
}

impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_i32(self.code())
    }
}

impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = i32::deserialize(deserializer)?;
        if 1 <= value && value < 5 {
            return Err(Error::custom(format_args!("Error! Value {} must be between 1 and 4.", value)));
        }

        Ok(Self::new(value))
    }
}

#[cfg(test)]
mod priority_tests {
    use super::Priority;

    #[test]
    fn test_code() {
        assert_eq!(1, Priority::Low.code());
        assert_eq!(2, Priority::Normal.code());
        assert_eq!(3, Priority::High.code());
        assert_eq!(4, Priority::Now.code());
    }

    #[test]
    fn test_new() {
        assert_eq!(Priority::Low, Priority::new(1));
        assert_eq!(Priority::Normal, Priority::new(2));
        assert_eq!(Priority::High, Priority::new(3));
        assert_eq!(Priority::Now, Priority::new(4));
        assert_eq!(Priority::Low, Priority::new(5));
        assert_eq!(Priority::Low, Priority::new(10));
    }
}
