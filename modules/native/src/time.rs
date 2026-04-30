use std::time::SystemTime;

pub fn get_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .expect("check clock settings as system time is before UNIX_EPOCH")
}
