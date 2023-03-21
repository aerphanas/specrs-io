#[cfg(test)]
mod test;

use gcd::Gcd;

pub struct Ressolution {
  width: u32,
  height: u32,
  gcd: u32
}

impl Ressolution {

  pub fn new(width: u32, height: u32) -> Ressolution {
      let gcd_calc: u32 = width.gcd_euclid(height);
      Ressolution {
          width,
          height,
          gcd: gcd_calc
      }
  }

  pub fn get_dar(&self) -> f32 {
      self.width as f32 / self.height as f32
  }

  pub fn get_ar(&self) -> (u32, u32) {
      let width = self.width / self.gcd;
      let height = self.height / self.gcd;
      (width, height)
  }

  pub fn get_p_count(&self) -> u32 {
      self.width * self.height
  }

}
