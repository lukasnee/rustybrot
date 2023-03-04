// TODO: rename offset to postion
// TODO: rename real to x
// TODO: rename imag to y

#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {
    pub real_offset: f64,
    pub imag_offset: f64,
    pub real_span: f64,
    pub imag_span: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            real_offset: 0.0,
            imag_offset: 0.0,
            real_span: 1.0,
            imag_span: 1.0,
        }
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.imag_span = self.real_span / aspect_ratio;
    }

    pub fn set_scale(&mut self, scale: f64) {
        let aspect_ratio = self.real_span / self.imag_span;
        self.real_span = scale;
        self.update_aspect_ratio(aspect_ratio);
    }

    pub fn adjust_scale(&mut self, factor: f64) {
        self.set_scale(self.real_span * factor);
    }

    pub fn reset(&mut self, real_offset: f64, imag_offset: f64, scale: f64) {
        self.real_offset = real_offset;
        self.imag_offset = imag_offset;
        self.set_scale(scale);
    }
    pub fn reset_to_origin(&mut self, scale: f64) {
        *self = Self::new();
        self.set_scale(scale);
    }
}
