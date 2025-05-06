use core::fmt::Write;
use heapless::String;

pub const TIMEOUT: u32 = 5000;

// Format message for sending
pub fn make_msg(start: u64, stop: u64, value: f64) -> String<128> {
    const DEVICE_ID: &str = env!("DEVICE_ID");

    let mut msg = String::<128>::new();
    write!(
        &mut msg,
        r#"{{"start": {}, "stop": {}, "value": {}, "key": "{}"}}"#,
        start,
        stop - TIMEOUT as u64,
        value,
        DEVICE_ID
    )
    .unwrap();
    msg
}
