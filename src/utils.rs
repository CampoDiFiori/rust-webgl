use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn link_program(
	gl: &GL,
	vert_source: &str,
	frag_source: &str,
) -> Result<WebGlProgram, String> {
	let program = gl.create_program().ok_or_else(|| String::from("Error creating program"))?;

	let vert_shader = compile_shader(
		gl,
		GL::VERTEX_SHADER,
		vert_source
	)?;

	let frag_shader = compile_shader(
		gl,
		GL::FRAGMENT_SHADER,
		frag_source,
	)?;

	gl.attach_shader(&program, &vert_shader);
	gl.attach_shader(&program, &frag_shader);
	gl.link_program(&program);

	if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
		Ok(program)
	} else {
		Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
	}
}


fn compile_shader(
	gl: &GL,
	shader_type: u32,
	source: &str,
) -> Result<WebGlShader, String> {
	let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Error creating shader"))?;

	gl.shader_source(&shader, source);
	gl.compile_shader(&shader);

	if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
		Ok(shader)
	} else {
		Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unable to get shader info log")))
	}
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];

	return_var[0] = 1.;
	return_var[5] = 1.;
	return_var[10] = 1.;
	return_var[15] = 1.;

	return_var[12] = tx;
	return_var[13] = ty;
	return_var[14] = tz;

	return_var
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];
	return_var[0] = sx;
	return_var[5] = sy;
	return_var[10] = sz;
	return_var[15] = 1.;

	return_var
}

pub fn matrix_mul<const ROW_LEN: usize, const N: usize>(mat1: [f32; N], mat2: [f32; N]) -> [f32; N] {
	let mut res = [0.; N];

	for i in 0..ROW_LEN {
		for j in 0..ROW_LEN {
			let mut sum = 0.;
			for k in 0..ROW_LEN {
				sum += mat1[(i * ROW_LEN) + k] * mat2[j + (k * ROW_LEN)];
			}
			res[(i * ROW_LEN) + j] = sum;
		}
	}

	res
}