/// A cardinal direction (`UP`, `DOWN`, `LEFT`, or `RIGHT`) that a node
/// can be facing/looking in. Contains the unit-coordinates of the direction
/// relative to the node.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Direction(pub i16, pub i16);

impl Direction {
    pub const UP: Self = Self(-1, 0);
    pub const DOWN: Self = Self(1, 0);
    pub const LEFT: Self = Self(0, -1);
    pub const RIGHT: Self = Self(0, 1);
}
