pub fn get_api_rate_ms(calls_per_second: f32) -> u64 {
    (1.0 / calls_per_second * 1000.0) as u64
}
