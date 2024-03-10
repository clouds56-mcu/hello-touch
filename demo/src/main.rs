#![no_std]
#![no_main]

#[macro_use] extern crate log;
extern crate alloc;

use alloc::vec::Vec;
use esp_idf_svc::{hal::delay::Delay, sys::{esp_err_to_name, esp_lcd_new_rgb_panel, esp_lcd_panel_draw_bitmap, esp_lcd_panel_init, esp_lcd_panel_reset}};

macro_rules! pin {
  (none) => { -1 };
  ($expr:tt) => {
    $expr
  };
}

mod consts;

#[no_mangle]
fn main() {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  let delay = Delay::new_default();
  let lcd_config = consts::lcd_config();
  let mut panel = core::ptr::null_mut();
  unsafe {
    esp_lcd_new_rgb_panel(&lcd_config as *const _, &mut panel as *mut _);
    esp_lcd_panel_reset(panel);
    let err = esp_lcd_panel_init(panel);
    if err != 0 { error!("{:?}", core::ffi::CStr::from_ptr(esp_err_to_name(err))); loop {} }
  };

  info!("Hello, world!");
  for i in 0..40 {
    let color_data = (0..8000).map(|_| (i as u16).wrapping_mul(38259)).collect::<Vec<_>>();
    // let color_data = [(i as u16).saturating_mul(38259); 800*10];
    info!("{} {}", color_data.len(), color_data[color_data.len() -1]);
    unsafe {
      esp_lcd_panel_draw_bitmap(panel, 0, i*10, 800, (i+1)*10, color_data.as_ptr() as *const _);
    }
  }

  loop {
    info!("loop...");
    delay.delay_ms(1000);
  }
}
