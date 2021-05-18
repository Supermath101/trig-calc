/// Contains the side lengths of a triangle.
#[derive(Debug)]
pub struct Triangle {
    pub leg1: f64, // always smallest side
    pub leg2: f64, // medium sized side
    pub hypotenuse: f64,
}

impl Triangle {
    /// Given the two leg lengths of a triangle, this will calculate all three sides using the Pythagorean Theorem.
    pub fn from_legs(side_a: f64, side_b: f64) -> Self {
        Triangle {
            leg1: f64::min(side_a, side_b),
            leg2: f64::max(side_a, side_b),
            hypotenuse: f64::sqrt(side_a.powi(2) + side_b.powi(2)),
        }
    }

    /// Given the length of the hypotenuse and a leg, this will calculate all three sides of a triangle using the Pythagorean Theorem.
    pub fn from_hypotenuse(side_a: f64, side_b: f64) -> Self {
        let hypotenuse = f64::max(side_a, side_b);
        let missing_leg = f64::sqrt(hypotenuse.powi(2) - f64::min(side_a, side_b).powi(2));
        todo!();
    }
}
