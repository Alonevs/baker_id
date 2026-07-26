/// Vector 2D para el SDK de juegos
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Crea un nuevo vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Crea un vector cero
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Suma dos vectores
    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Resta dos vectores
    pub fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Multiplica por un escalar
    pub fn mul(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Divide por un escalar
    pub fn div(&self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    /// Longitud del vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normaliza el vector
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self.div(len)
        } else {
            Self::zero()
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}
