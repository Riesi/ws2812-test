use std::thread;
use std::time::Duration;

use esp_idf_hal::peripherals::Peripherals;

// use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::RGB8;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

const LED_COUNT: usize = 5;

#[derive(Clone)]
struct LedPattern {
    time_step: u8,
    led_data: [RGB8; LED_COUNT],
}

impl LedPattern {
    fn new(time: u64, led_data: [RGB8; LED_COUNT]) -> Self {
        LedPattern {
            time_step: Self::convert_ms_to_time_step(time),
            led_data,
        }
    }
    /*
     * time step is biased starting from 10ms in 10ms steps
     */
    fn time_step_ms(&self) -> u64 {
        self.time_step as u64 * 10 + 10
    }

    fn convert_ms_to_time_step(time: u64) -> u8 {
        ((time - 10) / 10) as u8
    }
}

struct LedAnimation {
    entries: Vec<LedPattern>,
    index: usize,
    offset: u8,
}

impl LedAnimation {
    fn new() -> Self {
        LedAnimation {
            entries: Vec::new(),
            index: 0,
        }
    }
    fn next_pattern(&mut self) -> Option<LedPattern> {
        let ret = if let Some(pat) = self.entries.get(self.index) {
            Some(pat.clone())
        } else {
            None
        };
        self.index = (self.index + 1) % self.entries.len();
        ret
    }
    fn add_pattern(&mut self, pattern: LedPattern) {
        self.entries.push(pattern);
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    log::info!("Hello, world!");

    let ws2812_pin = peripherals.pins.gpio10;
    let channel = peripherals.rmt.channel0;
    let mut ani = LedAnimation::new();
    ani.add_pattern(LedPattern::new(
        100,
        [
            RGB8 {
                r: 0xff, g: 0, b: 0,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0,
            },
            RGB8 {
                r: 0, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0xff,
            },
        ],
    ));
    ani.add_pattern(LedPattern::new(
        100,
        [
            RGB8 {
                r: 0xff, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0,
            },
            RGB8 {
                r: 0, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0xff,
            },
        ],
    ));
    ani.add_pattern(LedPattern::new(
        100,
        [
            RGB8 {
                r: 0, g: 0xff, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0,
            },
            RGB8 {
                r: 0, g: 0, b: 0xff,
            },
        ],
    ));
    ani.add_pattern(LedPattern::new(
        100,
        [
            RGB8 {
                r: 0, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0,
            },
        ],
    ));
    ani.add_pattern(LedPattern::new(
        100,
        [
            RGB8 {
                r: 0, g: 0xff, b: 0,
            },
            RGB8 {
                r: 0, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0, g: 0xff, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0xff,
            },
            RGB8 {
                r: 0xff, g: 0, b: 0,
            },
        ],
    ));

    let mut ws2812 = Ws2812Esp32Rmt::new(channel, ws2812_pin).unwrap();

    let thread_b = thread::spawn(move || {
        loop{
            ani.next_pattern().map(|p| {
            ws2812.write_nocopy(p.led_data.iter().copied()).unwrap();
            thread::sleep(Duration::from_millis(p.time_step_ms()));
        });
        }
    });

    loop {

    }
}
