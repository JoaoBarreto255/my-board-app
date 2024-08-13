

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Frequence {
    OneTime,
    Daily,
    Weekly,
    Monthly,
    Yearly
}

impl Frequence {
    fn code(self) -> u8 {
        return match self {
            Frequence::OneTime => 1,
            Frequence::Daily => 2,
            Frequence::Weekly => 3,
            Frequence::Monthly => 4,
            Frequence::Yearly => 5,
        };
    }

    fn new(code: u8) -> Frequence {
        return match code {
            2 => Frequence::Daily,
            3 => Frequence::Weekly,
            4 => Frequence::Monthly,
            5 => Frequence::Yearly,
            _ => Frequence::OneTime,
        };
    }
}