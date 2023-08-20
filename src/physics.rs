use vecto_rs::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct VerletPhysicsProperties {
    pub friction: f32,
    pub ground_friction: f32,
    pub gravity : Vec2,
    pub floor_height : f32,
    pub collisions_on : bool,
}

impl Default for VerletPhysicsProperties {
    fn default() -> Self {
        Self {
            friction: 0.97, // 3% energy is lost
            ground_friction: 0.7, // 30% energy is lost
            gravity: Vec2::UP,
            floor_height : 720.0,
            collisions_on: true
        }
    }
}
