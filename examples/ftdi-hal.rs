use eh0::digital::v2::OutputPin;
use ftdi_embedded_hal as hal;
use std::{thread::sleep, time::Duration};

const NUM_BLINK: usize = 10;
const SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() {
    // 7つのFTDIデバイスが接続されている
    let device0 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("0")
        .open()
        .unwrap();
    let device1 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("1")
        .open()
        .unwrap();
    let device2 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("2")
        .open()
        .unwrap();
    let device3 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("3")
        .open()
        .unwrap();
    let device4 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("4")
        .open()
        .unwrap();
    let device5 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("5")
        .open()
        .unwrap();
    let device6 = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .serial("6")
        .open()
        .unwrap();

    let hal0 = hal::FtHal::init_default(device0).unwrap();
    let hal1 = hal::FtHal::init_default(device1).unwrap();
    let hal2 = hal::FtHal::init_default(device2).unwrap();
    let hal3 = hal::FtHal::init_default(device3).unwrap();
    let hal4 = hal::FtHal::init_default(device4).unwrap();
    let hal5 = hal::FtHal::init_default(device5).unwrap();
    let hal6 = hal::FtHal::init_default(device6).unwrap();

    let mut esp32_0_pin3 = hal0.ad3().unwrap();
    let mut esp32_1_pin3 = hal1.ad3().unwrap();
    let mut esp32_2_pin3 = hal2.ad3().unwrap();
    let mut esp32_3_pin3 = hal3.ad3().unwrap();
    let mut esp32_4_pin3 = hal4.ad3().unwrap();
    let mut esp32_5_pin3 = hal5.ad3().unwrap();
    let mut esp32_6_pin3 = hal6.ad3().unwrap();

    println!("Starting blinky example");
    for _ in 0..NUM_BLINK {
        esp32_0_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_0_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_1_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_1_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_2_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_2_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_3_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_3_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_4_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_4_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_5_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_5_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_6_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        esp32_6_pin3.set_low().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);

        esp32_0_pin3.set_high().expect("failed to set GPIO");
        esp32_1_pin3.set_high().expect("failed to set GPIO");
        esp32_2_pin3.set_high().expect("failed to set GPIO");
        esp32_3_pin3.set_high().expect("failed to set GPIO");
        esp32_4_pin3.set_high().expect("failed to set GPIO");
        esp32_5_pin3.set_high().expect("failed to set GPIO");
        esp32_6_pin3.set_high().expect("failed to set GPIO");
        sleep(SLEEP_DURATION);
        sleep(SLEEP_DURATION);
        sleep(SLEEP_DURATION);
        esp32_0_pin3.set_low().expect("failed to set GPIO");
        esp32_1_pin3.set_low().expect("failed to set GPIO");
        esp32_2_pin3.set_low().expect("failed to set GPIO");
        esp32_3_pin3.set_low().expect("failed to set GPIO");
        esp32_4_pin3.set_low().expect("failed to set GPIO");
        esp32_5_pin3.set_low().expect("failed to set GPIO");
        esp32_6_pin3.set_low().expect("failed to set GPIO");
    }
}
