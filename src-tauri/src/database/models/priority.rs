

#[derive(Debug,PartialEq,Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Now
}

impl Priority {
    fn code(self) -> u8 {
        return match self {
            Priority::Low => 1,
            Priority::Normal => 2,
            Priority::High => 3,
            Priority::Now => 4
        };
    }

    fn new(code: u8) -> Priority {
        return match code {
            2 => Priority::Normal,
            3 => Priority::High,
            4 => Priority::Now,
            _ => Priority::Low,
        };
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