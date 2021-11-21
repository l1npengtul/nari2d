// haha fuck you github copilot
pub trait CalculationProvider {
    fn add_floats(&self, f1: f32, f2: f32, f3: f32, f4: f32, by: f32) -> [f32; 4];

    fn sub_floats(&self, f1: f32, f2: f32, f3: f32, f4: f32, by: f32) -> [f32; 4];

    fn div_floats(&self, f1: f32, f2: f32, f3: f32, f4: f32, by: f32) -> [f32; 4];

    fn mul_floats(&self, f1: f32, f2: f32, f3: f32, f4: f32, by: f32) -> [f32; 4];

    fn rem_floats(&self, f1: f32, f2: f32, f3: f32, f4: f32, by: f32) -> [f32; 4];

    fn pow_floats(&self, f1: f32, f2: f32, f3: f32, f4: f32, by: f32) -> [f32; 4];

    fn inside_circumcircle(
        &self,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        chk_pt: [f32; 2],
    ) -> bool;
}
