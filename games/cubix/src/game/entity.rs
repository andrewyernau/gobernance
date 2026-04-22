use cgmath::{Point3, Vector3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub value: Vector3<f32>,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub half_extents: Vector3<f32>,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            half_extents: Vector3::new(0.5, 0.5, 0.5),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Item,
    Mob,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: EntityId,
    pub transform: Transform,
    pub velocity: Velocity,
    pub collider: Aabb,
    pub kind: EntityKind,
}
