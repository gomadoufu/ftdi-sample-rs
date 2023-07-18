use eh0::digital::v2::OutputPin;
use ftdi_embedded_hal as hal;
use std::{thread::sleep, time::Duration};

const NUM_BLINK: usize = 10;
const SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() {
    let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
        // .serial("0")
        .interface(ftdi::Interface::A)
        .open()
        .unwrap();

    let hal = hal::FtHal::init_default(device).unwrap();
    let mut output_pin = hal.ad3().unwrap();

    println!("Starting blinky example");
    for n in 0..NUM_BLINK {
        output_pin.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        output_pin.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        println!("Blinked {}/{} times", n + 1, NUM_BLINK);
    }
}
