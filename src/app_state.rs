use std::sync::Arc;
use std::sync::Mutex;

use lazy_static::lazy_static;

#[derive(Default)]
pub struct AppState {
	pub canvas_height: f32,
	pub canvas_width: f32,
	pub control_bottom: f32,
	pub control_top: f32,
	pub control_left: f32,
	pub control_right: f32,
	pub time: f32,
}

lazy_static! {
	static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::default()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
	let min_height_width = canvas_height.min(canvas_width);
	let display_size = 0.9 * min_height_width;
	let half_display_size = display_size / 2.;
	let half_canvas_height = canvas_height / 2.;
	let half_canvas_width = canvas_width / 2.;

	let mut data = APP_STATE.lock().unwrap();

	*data = Arc::new(AppState {
		canvas_height,
		canvas_width,

		control_bottom: half_canvas_height - half_display_size,
		control_top: half_canvas_height + half_display_size,
		control_left: half_canvas_width - half_display_size,
		control_right: half_canvas_width + half_display_size,

		time,
	})
}

pub fn get_curr_state() -> Arc<AppState> {
	APP_STATE.lock().unwrap().clone()
}