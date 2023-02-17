#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    pub fn modulus(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}
