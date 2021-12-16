use crate::utils;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertice_ary_length: usize,
    rect_verice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &GL) -> Self {
        let program = utils::link_program(
            gl,
            crate::shaders::vertex::color_2d::SHADER,
            crate::shaders::fragment::color_2d::SHADER,
        )
        .unwrap();

        let vertices_rect: [f32; 12] = [0., 1., 0., 0., 1., 1., 1., 1., 0., 0., 1., 0.];

        // creating a memory buffer that JS can understand
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );

        // putting data from JS close to GPU
        let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        // STATIC_DRAW means we will not change the content of this buffer
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            rect_vertice_ary_length: vertices_rect.len(),
            rect_verice_buffer: buffer_rect,
            program,
        }
    }

	#[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        gl: &GL,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_verice_buffer));

        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0., 0.5, 0.5, 1.0);
		gl.uniform1f(Some(&self.u_opacity), 1.0);


		let translation_matrix = utils::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_matrix =
            utils::scaling_matrix(2. * (right - left) / canvas_width, 2. * (top - bottom) / canvas_height, 0.);

		let transform_matrix = utils::matrix_mul::<4, 16>(scale_matrix, translation_matrix);
		gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_matrix);

		gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_length / 2) as i32);
    }
}
