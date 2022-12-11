#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(const_option)]

// use arduino_hal::{default_serial, delay_ms, pins, prelude::*, Adc, DefaultClock, Peripherals};
// use panic_halt as _;
// use ufmt::uwriteln;

// mod time;
use arduino_hal as hal;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

#[no_mangle]
fn main() -> ! {
    // let dp = Peripherals::take().unwrap();
    // let pins = pins!(dp);
    // let mut adc = Adc::new(dp.ADC, Default::default());

    // let mut serial = default_serial!(dp, pins, 115200);
    // let a0 = pins.a0.into_analog_input(&mut adc);
    // let a1 = pins.a1.into_analog_input(&mut adc);

    // // let clock = time::TimerClock::<DefaultClock>::new(dp.TC0, time::Resolution::_1_MS).unwrap();
    // // unsafe { avr_device::interrupt::enable() };

    // loop {
    //     let sweat_rate = a0.analog_read(&mut adc);
    //     // let sweat_rate = 0;
    //     let heart_rate = a1.analog_read(&mut adc);
    //     // let timestamp = clock.millis();

    //     // uwriteln!(&mut serial, "{},{},{}", sweat_rate, heart_rate, timestamp);
    //     uwriteln!(&mut serial, "{},{}", sweat_rate, heart_rate);
    //     delay_ms(100);
    // }
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut serial = hal::default_serial!(dp, pins, 115200);
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
