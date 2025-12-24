// Utility functions for the application

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[allow(unused_macros)]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub(crate) use console_log;
