pub struct MonagementInit {
    pub start: u64,
    pub maximum: u64,
}

impl MonagementInit {
    pub fn get_minimum(&self) -> u64 {
        1 << self.start
    }

    pub fn get_raw_minimum(&self) -> u64 {
        self.start
    }

    pub fn get_maximum(&self) -> u64 {
        self.maximum
    }
}
