// Utility functions for the application

// use wasm_bindgen::prelude::*;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
//
// For more details see
// https://github.com/rustwasm/console_error_panic_hook#readme
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub(crate) use console_log;

// Timing utilities
pub struct Timer {
    start_time: f64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: js_sys::Date::now(),
        }
    }

    pub fn elapsed_ms(&self) -> f64 {
        js_sys::Date::now() - self.start_time
    }

    pub fn elapsed_s(&self) -> f64 {
        self.elapsed_ms() / 1000.0
    }

    pub fn reset(&mut self) {
        self.start_time = js_sys::Date::now();
    }
}

// Performance monitoring
pub struct PerformanceMonitor {
    frame_times: Vec<f64>,
    last_frame_time: f64,
    frame_count: u32,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            frame_times: Vec::with_capacity(100),
            last_frame_time: js_sys::Date::now(),
            frame_count: 0,
        }
    }

    pub fn frame_start(&mut self) {
        let current_time = js_sys::Date::now();
        let frame_time = current_time - self.last_frame_time;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > 100 {
            self.frame_times.remove(0);
        }

        self.last_frame_time = current_time;
        self.frame_count += 1;

        // Log performance every 300 frames (~5 seconds at 60fps)
        if self.frame_count % 300 == 0 {
            let avg_frame_time = self.average_frame_time();
            let fps = 1000.0 / avg_frame_time;
            console_log!(
                "Performance: {:.1} FPS, {:.2}ms frame time",
                fps,
                avg_frame_time
            );
        }
    }

    pub fn average_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.frame_times.iter().sum();
        sum / self.frame_times.len() as f64
    }

    pub fn fps(&self) -> f64 {
        let avg_time = self.average_frame_time();
        if avg_time > 0.0 {
            1000.0 / avg_time
        } else {
            0.0
        }
    }
}

// Coordinate system utilities
pub fn screen_to_ndc(
    screen_x: f32,
    screen_y: f32,
    screen_width: f32,
    screen_height: f32,
) -> (f32, f32) {
    let ndc_x = (screen_x / screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (screen_y / screen_height) * 2.0; // Flip Y axis
    (ndc_x, ndc_y)
}

pub fn ndc_to_screen(ndc_x: f32, ndc_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
    let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
    let screen_y = (1.0 - ndc_y) * 0.5 * screen_height;
    (screen_x, screen_y)
}

// Color utilities
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = (h * 360.0 / 60.0) % 6.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r_prime + m, g_prime + m, b_prime + m)
}

// Random number utilities for initialization
pub fn random_in_range(min: f32, max: f32) -> f32 {
    let random_value: f32 = js_sys::Math::random() as f32;
    min + random_value * (max - min)
}

pub fn random_unit_vector() -> (f32, f32) {
    let angle = random_in_range(0.0, 2.0 * std::f32::consts::PI);
    (angle.cos(), angle.sin())
}
