use vecto_rs::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct VerletPhysicsProperties {
    pub friction: f32,
    pub gravity : Vec2,
    pub collisions_on : bool,
}

impl Default for VerletPhysicsProperties {
    fn default() -> Self {
        Self {
            friction: 0.97, // 3% energy is lost
            gravity: Vec2::ZERO,
            collisions_on: true
        }
    }
}
