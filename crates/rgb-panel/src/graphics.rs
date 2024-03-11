use core::{convert::Infallible, usize};

use embedded_graphics_core::{draw_target::DrawTarget, geometry::{OriginDimensions, Size}, pixelcolor::{raw::ToBytes, PixelColor}, Pixel};

use crate::{panel::Panel1, spec::{CheckColor, FitsBpp}};

impl<const N: usize, OP, DATA, Color: PixelColor> DrawTarget for Panel1<N, OP, DATA, Color>
where
  Color: PixelColor,
  CheckColor<Color>: FitsBpp<N>,
  Color: ToBytes,
  Color::Bytes: AsRef<[u8]>,
{
  type Color = Color;
  type Error = Infallible;

  fn draw_iter<I: IntoIterator<Item = Pixel<Self::Color>>>(&mut self, pixels: I) -> Result<(), Self::Error> {
    for Pixel(coord, color) in pixels {
      let x = coord.x as usize;
      let y = coord.y as usize;
      let offset = (y * self.spec.config.stride as usize + x) * Self::byte_len();
      let bytes = color.to_be_bytes();
      self.fb[offset..offset + Self::byte_len()].copy_from_slice(bytes.as_ref());
    }
    Ok(())
  }
}

impl<const N: usize, OP, DATA, Color> OriginDimensions for Panel1<N, OP, DATA, Color> {
  fn size(&self) -> Size {
    Size::new(self.spec.config.width, self.spec.config.height)
  }
}

#[cfg(test)]
mod test {
  use crate::spec::{Config, PanelSpec};

use super::*;
  use embedded_graphics_core::{geometry::Point, pixelcolor::{Rgb565, RgbColor}};

fn spec<const N: usize>() -> PanelSpec<N, i32, i32> {
  PanelSpec {
    config: Config {
      width: 800,
      height: 480,
      stride: 800,
      bits_per_pixel: N as _,
      fb_count: 2,
    },
    hsync_gpio: 1,
    vsync_gpio: 2,
    de_gpio: Some(3),
    pclk_gpio: 4,
    disp_gpio: None,
    data_gpios: [0; N],
  }
}

#[test]
fn test_panel() {
  let mut panel = Panel1::<16, _, _, Rgb565>::new(spec());
  panel.draw_iter([Pixel(Point::new(0, 0), Rgb565::BLACK)]);
}
}
