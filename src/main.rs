
use std::thread;
use std::time::Duration;

use esp_idf_hal::peripherals::Peripherals;

use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::RGB8;
use smart_leds::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

const LED_COUNT: usize = 5;

#[derive(Clone)]
struct LedPattern {
  time_step: u8,
  led_data: [RGB8;LED_COUNT],
}

struct LedAnimation {
  entries: Vec<LedPattern>,
  pointer: usize,
}

impl LedAnimation {
  fn new() -> Self{
    LedAnimation{entries: Vec::new(), pointer: 0}
  }
  fn next_pattern(&mut self) -> Option<LedPattern>{
    let ret = if let Some(pat) = self.entries.get(self.pointer%self.entries.len()){
      Some(pat.clone())
    }else{
      None
    };
    self.pointer += 1;
    ret
  }
  fn add_pattern(&mut self, pattern: LedPattern){
    self.entries.push(pattern);
  }
}

fn main(){
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  let peripherals = Peripherals::take().unwrap();
  
  log::info!("Hello, world!");

  let ws2812_pin = peripherals.pins.gpio10;
  let channel = peripherals.rmt.channel0;
  //let mut ws2812 = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(channel, ws2812_pin).unwrap();

  let mut ani = LedAnimation::new();
  let pat = LedPattern{
                            time_step: 10, 
                            led_data: [ RGB8{r:0xff,g:0,b:0},
                                        RGB8{r:0,g:0xff,b:0},
                                        RGB8{r:0,g:0,b:0xff},
                                        RGB8{r:0,g:0xff,b:0xff},
                                        RGB8{r:0xff,g:0,b:0xff}]};
  &ani.add_pattern(pat);
  // let pat = LedPattern{
  //                           time_step: 10, 
  //                           led_data: [ RGB8{r:0xff,g:0,b:0},
  //                                       RGB8{r:0,g:0xff,b:0},
  //                                       RGB8{r:0,g:0,b:0xff},
  //                                       RGB8{r:0,g:0xff,b:0xff},
  //                                       RGB8{r:0xff,g:0,b:0xff}]};
  // ani.add_pattern(pat);

  let mut ws2812 = Ws2812Esp32Rmt::new(channel, ws2812_pin).unwrap();

  let mut hue = 20;
  loop {
    let pixels = std::iter::repeat(hsv2rgb(Hsv {
        hue,
        sat: 255,
        val: 5,
    })).take(5);
    let pixels2 = std::iter::repeat(hsv2rgb(Hsv {
        hue: hue.wrapping_add(100),
        sat: 255,
        val: 50,
    })).take(5);
    if let Some(p) = ani.next_pattern() {
      let i = p.led_data.clone().iter();
      let o = [ RGB8{r:0xff,g:0,b:0},
              RGB8{r:0,g:0xff,b:0},
              RGB8{r:0,g:0,b:0xff},
              RGB8{r:0,g:0xff,b:0xff},
              RGB8{r:0xff,g:0,b:0xff}].to_vec();
      let ii = o.iter();
      ws2812.write(pixels).unwrap();
    
      thread::sleep(Duration::from_millis(50));
      ws2812.write(pixels2).unwrap();
      thread::sleep(Duration::from_millis(50));
      //ws2812.write_nocopy(ii).unwrap();
    }

      thread::sleep(Duration::from_millis(50));

      hue = hue.wrapping_add(3);
  }
}
