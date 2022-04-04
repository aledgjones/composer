pub struct MM(pub f32);

pub struct Spaces(pub f32);

pub struct BoundingBox {
    pub width: Spaces,
    pub height: Spaces,
    pub padding: Padding<Spaces>,
}

pub struct Padding<T>(pub T, pub T, pub T, pub T);
