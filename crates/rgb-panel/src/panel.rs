use alloc::vec::Vec;
use alloc::vec;

use crate::spec::PanelSpec;

pub trait Panel<Color> {
  fn byte_len() -> usize;
  fn current_fb(&self) -> &[u8];
  fn current_fb_mut(&mut self) -> &mut [u8];
}

pub struct Panel1<const N: usize, OP, DATA, Color> {
  pub fb: Vec<u8>,
  pub spec: PanelSpec<N, OP, DATA>,
  pub _color: core::marker::PhantomData<Color>,
}

impl<const N: usize, OP, DATA, Color> Panel1<N, OP, DATA, Color> {
  pub fn new(spec: PanelSpec<N, OP, DATA>) -> Self {
    let fb = vec![0; (spec.config.stride * spec.config.height) as usize * Self::byte_len()];
    Self {
      fb,
      spec,
      _color: core::marker::PhantomData,
    }
  }

  pub const fn byte_len() -> usize {
    (N - 1) / 8 + 1
  }
}

impl<const N: usize, OP, DATA, Color> Panel<Color> for Panel1<N, OP, DATA, Color> {
  fn byte_len() -> usize {
    (N - 1) / 8 + 1
  }

  fn current_fb(&self) -> &[u8] {
    &self.fb
  }

  fn current_fb_mut(&mut self) -> &mut [u8] {
    &mut self.fb
  }
}

pub struct Panel2<const N: usize, OP, DATA, Color> {
  pub fb1: Vec<u8>,
  pub fb2: Vec<u8>,
  pub current: usize,
  pub spec: PanelSpec<N, OP, DATA>,
  pub _color: core::marker::PhantomData<Color>,
}

impl<const N: usize, OP, DATA, Color> Panel2<N, OP, DATA, Color> {
  pub fn new(spec: PanelSpec<N, OP, DATA>) -> Self {
    let fb1 = vec![0; (spec.config.stride * spec.config.height) as usize * Self::byte_len()];
    let fb2 = vec![0; (spec.config.stride * spec.config.height) as usize * Self::byte_len()];
    Self {
      fb1,
      fb2,
      current: 1,
      spec,
      _color: core::marker::PhantomData,
    }
  }

  pub const fn byte_len() -> usize {
    (N - 1) / 8 + 1
  }
}

impl<const N: usize, OP, DATA, Color> Panel<Color> for Panel2<N, OP, DATA, Color> {
  fn byte_len() -> usize {
    (N - 1) / 8 + 1
  }

  fn current_fb(&self) -> &[u8] {
    if self.current == 1 {
      &self.fb1
    } else {
      &self.fb2
    }
  }

  fn current_fb_mut(&mut self) -> &mut [u8] {
    if self.current == 1 {
      &mut self.fb1
    } else {
      &mut self.fb2
    }
  }
}
