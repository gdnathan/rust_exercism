const CARS_PER_HOURS: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let factory_default = CARS_PER_HOURS * speed as u32;

    factory_default as f64
        * match speed {
            0     => 0.0,
            1..=4 => 1.0,
            5..=8 => 0.9,
            9 | 10 => 0.77,
            _ => panic!("speed out of range"),
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
