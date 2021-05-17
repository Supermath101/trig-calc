/// Contains the side lengths of a triangle.
pub struct Triangle {
    pub leg1: f64, // always smallest side
    pub leg2: f64, // medium sized side
    pub hypotenuse: f64,
}

impl Triangle {
    /// Given the two leg lengths of a triangle, this will calculate all three sides using the Pythagorean Theorem.
    pub fn from_legs(side_a: f64, side_b: f64) -> Self {
        todo!();
    }

    /// Given the length of the hypotenuse and a leg, this will calculate all three sides of a triangle using the Pythagorean Theorem.
    pub fn from_hypotenuse(side_a: f64, side_b: f64) -> Self {
        todo!();
    }
}
