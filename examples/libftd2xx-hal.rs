use eh0::digital::v2::OutputPin;
use ftdi_embedded_hal as hal;
use std::{thread::sleep, time::Duration};

const NUM_BLINK: usize = 10;
const SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() {
    // 7つのFTDIデバイスが接続されている
    let device0 = libftd2xx::Ft232h::with_serial_number("0").unwrap();
    let device1 = libftd2xx::Ft232h::with_serial_number("1").unwrap();
    let device2 = libftd2xx::Ft232h::with_serial_number("2").unwrap();
    let device3 = libftd2xx::Ft232h::with_serial_number("3").unwrap();
    let device4 = libftd2xx::Ft232h::with_serial_number("4").unwrap();
    let device5 = libftd2xx::Ft232h::with_serial_number("5").unwrap();
    let device6 = libftd2xx::Ft232h::with_serial_number("6").unwrap();

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
    for n in 0..NUM_BLINK {
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
