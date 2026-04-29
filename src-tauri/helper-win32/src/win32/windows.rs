#[cfg(windows)]
pub fn local_utc_offset_secs() -> i64 {
    use windows::Win32::System::Time::{
        DYNAMIC_TIME_ZONE_INFORMATION, GetDynamicTimeZoneInformation, GetTimeZoneInformation,
        TIME_ZONE_INFORMATION,
    };

    unsafe {
        let mut tz = DYNAMIC_TIME_ZONE_INFORMATION::default();
        let _ = GetDynamicTimeZoneInformation(&mut tz);

        if tz.DynamicDaylightTimeDisabled {
            return 0 as i64;
        }

        let mut _tz_info = TIME_ZONE_INFORMATION::default();
        let time_zone = GetTimeZoneInformation(&mut _tz_info);

        let is_daylight = time_zone == 2;

        let bias = tz.Bias;
        let offset_bias = if is_daylight {
            tz.DaylightBias
        } else {
            tz.StandardBias
        };

        -((offset_bias + bias) as i64 * 60)
    }
}
