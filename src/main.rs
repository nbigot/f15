extern crate winapi;
use std::{thread, time};
use winapi::um::winuser::{keybd_event, VK_NUMLOCK, KEYEVENTF_KEYUP};

fn simulate_keypress() {
    unsafe {
        // By simulate 2 consecutive keypresses of the F15 key (Verr Num)
        // the state of the NumLock should not change its state and
        // the LED of the NumLock should not blink (it's too fast)

        // Simulate a keypress of the F15 key (Verr Num)
        keybd_event(VK_NUMLOCK as u8, 0, 0, 0);
        thread::sleep(time::Duration::from_millis(1));
        keybd_event(VK_NUMLOCK as u8, 0, KEYEVENTF_KEYUP, 0);
        thread::sleep(time::Duration::from_millis(1));

        // Simulate a keypress of the F15 key (Verr Num)
        keybd_event(VK_NUMLOCK as u8, 0, 0, 0);
        thread::sleep(time::Duration::from_millis(1));
        keybd_event(VK_NUMLOCK as u8, 0, KEYEVENTF_KEYUP, 0);
    }
}

fn main() {
    // Note: this program is intended to be run on Windows only.
    // Get the duration from an environment variable or use 60 seconds as the default
    const DEFAULT_DURATION: u64 = 60;
    let duration: u64 = option_env!("SLEEP_DURATION")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DURATION);
    let sleep_duration = time::Duration::from_secs(duration);

    #[cfg(feature = "verbose")]
    {
        println!("Frequency: {} seconds", duration);
    }

    loop {
        #[cfg(feature = "verbose")]
        {
            println!("Simulating keypress F15");
        }
        simulate_keypress();
        thread::sleep(sleep_duration);
    }
}
