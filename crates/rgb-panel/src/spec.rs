
use core::usize;

use embedded_graphics_core::pixelcolor;

pub struct PanelSpec<const N: usize, OP, DATA> {
  /// Configs like width and height
  pub config: Config,
  /// GPIO used for HSYNC signal
  pub hsync_gpio: OP,
  /// GPIO used for VSYNC signal
  pub vsync_gpio: OP,
  /// GPIO used for DE signal, set to -1 if it's not used
  pub de_gpio: Option<OP>,
  /// GPIO used for PCLK signal
  pub pclk_gpio: OP,
  /// GPIO used for display control signal, set to -1 if it's not used
  pub disp_gpio: Option<OP>,
  /// GPIOs used for data lines
  pub data_gpios: [DATA; N],
}

pub struct Config {
  pub width: u32,
  pub height: u32,
  pub stride: u32,
  pub bits_per_pixel: u32,
  pub fb_count: u32,
  // pub bounce_buffer_size_px: u32,
}

pub trait FitsBpp<const N: usize> {}
pub struct CheckColor<T>(core::marker::PhantomData<T>);

macro_rules! impl_fits_bpp {
  ($n:expr => $($ty:ty),*) => {
    $(impl FitsBpp<$n> for CheckColor<$ty> {})*
  };
}
impl_fits_bpp!(2 => pixelcolor::Gray2);
impl_fits_bpp!(4 => pixelcolor::Gray4);
impl_fits_bpp!(8 => pixelcolor::Gray8);
impl_fits_bpp!(15 => pixelcolor::Rgb555, pixelcolor::Bgr555);
impl_fits_bpp!(16 => pixelcolor::Rgb565, pixelcolor::Bgr565);
impl_fits_bpp!(18 => pixelcolor::Rgb666, pixelcolor::Bgr666);
impl_fits_bpp!(24 => pixelcolor::Rgb888, pixelcolor::Bgr888);
