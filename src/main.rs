use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{delay::FreeRtos, gpio, prelude::Peripherals};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    #[allow(unused)]
    let peripherals = Peripherals::take().unwrap();
    #[allow(unused)]
    let pins = peripherals.pins;
    let mut led = gpio::PinDriver::output(pins.gpio2)?;

    println!("Hello, world1!1");

    loop {
        println!("set high");
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(2000);
        println!("set low");
        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}