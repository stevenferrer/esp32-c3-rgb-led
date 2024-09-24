use anyhow::Result;
use esp_idf_svc::hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_svc::sys::esp_random;
use log::info;

use esp32_c3_rgb_led::{RGB8, WS2812RMT};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0)?;
    let delay = 1000;
    loop {
        random_light(&mut led);
        FreeRtos::delay_ms(delay);
    }
}

#[allow(unused)]
fn random_light(led: &mut WS2812RMT) {
    let mut color = RGB8::new(0, 0, 0);
    unsafe {
        let r = esp_random() as u8;
        let g = esp_random() as u8;
        let b = esp_random() as u8;

        color = RGB8::new(r, g, b);
    }

    info!("({}, {}, {})", color.r, color.g, color.b);
    led.set_pixel(color).unwrap();
}
