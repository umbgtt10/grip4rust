pub struct Rect {
    w: f64,
    h: f64,
}

impl Rect {
    pub fn new(w: f64, h: f64) -> Self {
        Self { w, h }
    }

    pub fn area(&self) -> f64 {
        self.w * self.h
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.w + self.h)
    }

    pub fn center(&self) -> (f64, f64) {
        (self.w / 2.0, self.h / 2.0)
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect({}x{})", self.w, self.h)
    }
}
