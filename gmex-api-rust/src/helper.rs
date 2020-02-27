#[inline]
pub fn time_now_msec() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

#[inline]
pub fn time_now_sec() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

#[inline]
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
