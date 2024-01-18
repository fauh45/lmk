use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// LMK uses custom timestamp starting from 2024-01-17T15:00:00.000+00:00
pub const LMK_EPOCH_OFFSET: u64 = 1_705_503_600;

/// Get epoch that starts at 2024-01-17T15:00:00.000+00:00
pub fn get_lmk_epoch() -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(LMK_EPOCH_OFFSET)
}

/// Generate a LMK epoch based timestamp in secs
///
/// Might be problematic, but if duration is before LMK EPOCH it will result in the function returning 0.
pub fn generate_now() -> u64 {
    let lmk_epoch = get_lmk_epoch();

    SystemTime::now()
        .duration_since(lmk_epoch)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}
