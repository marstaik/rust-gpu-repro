use nalgebra::Vector4;
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(output: &mut Vector4<f32>) {
	*output = Vector4::new(1.0, 0.0, 0.0, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
	#[spirv(vertex_index)] vert_id: i32,
	#[spirv(position, invariant)] out_pos: &mut Vector4<f32>,
) {
	*out_pos = Vector4::new(
		(vert_id - 1) as f32,
		((vert_id & 1) * 2 - 1) as f32,
		0.0,
		1.0,
	);
}
