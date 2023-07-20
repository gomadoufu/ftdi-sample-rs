use eh0::digital::v2::OutputPin;
use ftdi_embedded_hal as hal;
use std::{thread::sleep, time::Duration};

const SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() {
    // 7つのFTDIデバイスが接続されている

    let devices = (0..7).map(|i| {
        ftdi::find_by_vid_pid(0x0403, 0x6014)
            .serial(&i.to_string())
            .open()
            .unwrap()
    });

    let hals = devices.map(|device| hal::FtHal::init_default(device).unwrap());

    let each_gpio = hals.map(|hal| hal.ad3().unwrap()).collect::<Vec<_>>();

    println!("Starting blinky example");
    for mut pinout in each_gpio.into_iter() {
        pinout.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        pinout.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
    }
}
