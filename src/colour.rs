use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Colour {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Colour {
    pub fn convert(self) -> (u8, u8, u8, u8) {
        // Convert u8 RGB values to f32 in the range [0.0, 1.0]
        let r = f32::from(self.r) / 255.0;
        let g = f32::from(self.g) / 255.0;
        let b = f32::from(self.b) / 255.0;

        // Convert from linear RGB to sRGB
        let r_srgb = if r <= 0.0031308 {
            r * 12.92
        } else {
            1.055 * r.powf(1.0 / 2.4) - 0.055
        };
        let g_srgb = if g <= 0.0031308 {
            g * 12.92
        } else {
            1.055 * g.powf(1.0 / 2.4) - 0.055
        };
        let b_srgb = if b <= 0.0031308 {
            b * 12.92
        } else {
            1.055 * b.powf(1.0 / 2.4) - 0.055
        };

        // Convert to u8 values
        let r_u8 = (r_srgb * 255.0) as u8;
        let g_u8 = (g_srgb * 255.0) as u8;
        let b_u8 = (b_srgb * 255.0) as u8;

        // Set alpha channel to 255 (fully opaque)
        let a_u8 = 255;

        (r_u8, g_u8, b_u8, a_u8)
    }
}

impl Mul<f64> for Colour {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Colour {
            r: ((self.r as f64) * rhs) as u8,
            g: ((self.g as f64) * rhs) as u8,
            b: ((self.b as f64) * rhs) as u8,
        }
    }
}

impl Add for Colour {
    type Output = Self;
    fn add(self, rhs: Colour) -> Self {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
