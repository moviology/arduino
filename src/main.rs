#![no_std]
#![no_main]

use arduino_hal as hal;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

/// TODO: Fix Heart Rate Reading (not accurate)

#[arduino_hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut serial = hal::default_serial!(dp, pins, 57600);
    let mut adc = hal::Adc::new(dp.ADC, Default::default());

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);

    loop {
        let mut gsr_sum = 0;
        let mut hr_sum = 0;

        for _ in 0..10 {
            let gsr_value = a0.analog_read(&mut adc);
            let hr_value = a1.analog_read(&mut adc);

            gsr_sum += gsr_value;
            hr_sum += hr_value;

            hal::delay_ms(200);
        }

        let gsr_average = gsr_sum / 10;
        let hr_average = hr_sum / 100;

        uwriteln!(&mut serial, "{}, {}\r", hr_average, gsr_average).void_unwrap();
    }
}
