#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r.clamp(0, 255),
            g: g.clamp(0, 255),
            b: b.clamp(0, 255),
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, color: Color) -> Color {
        Color::new(
            ((color.r as f32) * self) as u8,
            ((color.g as f32) * self) as u8,
            ((color.b as f32) * self) as u8,
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Self::Output {
        Self::Output::new(
            ((self.r as f32) * scalar) as u8,
            ((self.g as f32) * scalar) as u8,
            ((self.b as f32) * scalar) as u8,
        )
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.r, self.g, self.b)
    }
}

pub static BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub static RED: Color = Color { r: 255, g: 0, b: 0 };
pub static GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub static YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
};
pub static BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub static CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
};
pub static WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub static GRAY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
};
pub static PURPLE: Color = Color {
    r: 123,
    g: 50,
    b: 220,
};
