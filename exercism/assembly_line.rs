pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = 221 as f64;
    let f_speed = speed as f64;
    
    match speed {
        1..=4 => f_speed * rate * 1.0,
        5..=8 => f_speed * rate * 0.9,
        _ => f_speed * rate * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate = production_rate_per_hour(speed) / 60.0;
    rate as u32
}
