// --- 1. THE BINARY BASE (Taijitu) ---
pub type Taijitu = bool;

// --- 2. THE LOCOMOTION LAYER ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locomotion {
    Passive = 0,
    Active = 1,
}

// --- 3. THE ATOM (Ravel) ---
#[derive(Debug, Clone, PartialEq)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Convert longitude/latitude to Mercator projection coordinates (normalized -PI..PI)
    pub fn to_mercator(&self) -> (f32, f32) {
        let x = self.x.to_radians();
        let y = (self.y.to_radians().sin()).atanh();
        (x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ravel {
    pub base: Taijitu,
    pub locomotion: Locomotion,
    pub resolution: u8,
    pub pos: Pos,
    pub chord: [i32; 64],
}

impl Ravel {
    pub fn new(base: Taijitu, locomotion: Locomotion, resolution: u8, pos: Pos, chord: [i32; 64]) -> Self {
        Self { base, locomotion, resolution, pos, chord }
    }
}
