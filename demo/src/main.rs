#![no_std]
#![no_main]

#[macro_use] extern crate log;
extern crate alloc;

use esp_idf_svc::{hal::delay::Delay, sys::{esp_err_to_name, esp_lcd_new_rgb_panel, esp_lcd_panel_draw_bitmap, esp_lcd_panel_init, esp_lcd_panel_reset, esp_lcd_rgb_panel_get_frame_buffer}};
use embedded_graphics::{geometry::{Point, Size}, pixelcolor::{Rgb565, RgbColor}, primitives::{Primitive, PrimitiveStyle, Rectangle}, Drawable};
use embedded_graphics_framebuf::{backends::{EndianCorrectedBuffer, EndianCorrection}, FrameBuf};

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
  let panel = unsafe {
    let mut panel = core::ptr::null_mut();
    esp_lcd_new_rgb_panel(&lcd_config as *const _, &mut panel as *mut _);
    esp_lcd_panel_reset(panel);
    let err = esp_lcd_panel_init(panel);
    if err != 0 { error!("{:?}", core::ffi::CStr::from_ptr(esp_err_to_name(err))); loop {} }
    panel
  };

  let buffer_ptr = unsafe {
    let mut ptr = core::ptr::null_mut();
    let err = esp_lcd_rgb_panel_get_frame_buffer(panel, 1, &mut ptr as *mut _);
    if err != 0 { error!("{:?}", core::ffi::CStr::from_ptr(esp_err_to_name(err))); loop {} }
    ptr as *mut Rgb565
  };
  let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, 800*480) };
  let mut frame = FrameBuf::new(EndianCorrectedBuffer::new(buffer, EndianCorrection::ToLittleEndian), 800, 480);

  info!("Hello, world!");
  delay.delay_ms(1000);

  const COLORS: [Rgb565; 8] = [Rgb565::BLUE, Rgb565::CYAN, Rgb565::GREEN, Rgb565::BLACK, Rgb565::RED, Rgb565::WHITE, Rgb565::YELLOW, Rgb565::MAGENTA];
  for i in 0..48 {
    let color = if i <= 10 || i >= 40 {
      COLORS[i as usize % COLORS.len()]
    } else {
      COLORS[0]
    };
    info!("{} {:?}", i, color);
    Rectangle::new(Point::new(0, i*10), Size::new(800, 10))
      .into_styled(PrimitiveStyle::with_fill(color.into()))
      .draw(&mut frame)
      .unwrap();
  }
  unsafe {
    esp_lcd_panel_draw_bitmap(panel, 0, 0, 800, 480, buffer_ptr as *const _);
  }

  loop {
    info!("loop...");
    // unsafe {
    //   esp_lcd_rgb_panel_restart(panel);
    // }
    delay.delay_ms(1000);
  }
}
