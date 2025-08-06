use crate::utils::console_log;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent};

pub struct InputState {
    pub mouse_pos: (f32, f32),
    pub last_mouse_pos: (f32, f32),
    pub is_dragging: bool,
    pub zoom_delta: f32,
    pub pause_pressed: bool,
    pub reset_pressed: bool,
    pub running: bool,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub is_rotating: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos: (0.0, 0.0),
            last_mouse_pos: (0.0, 0.0),
            is_dragging: false,
            zoom_delta: 0.0,
            pause_pressed: false,
            reset_pressed: false,
            running: true,
            rotation_x: 0.0,
            rotation_y: 0.0,
            is_rotating: false,
        }
    }
}

pub struct InputHandler {
    state: Rc<RefCell<InputState>>,
    _closures: Vec<Closure<dyn FnMut(web_sys::Event)>>, // Keep closures alive
}

impl InputHandler {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self {
            state: Rc::new(RefCell::new(InputState::new())),
            _closures: Vec::new(),
        })
    }

    pub fn setup_event_listeners(&mut self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Mouse down event
        {
            let state = self.state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                let mut state = state.borrow_mut();

                // Left click for rotation (default), right click for panning
                if mouse_event.button() == 0 {
                    // Left click
                    state.is_rotating = true;
                } else if mouse_event.button() == 2 {
                    // Right click
                    state.is_dragging = true;
                }

                state.last_mouse_pos =
                    (mouse_event.client_x() as f32, mouse_event.client_y() as f32);
                state.mouse_pos = state.last_mouse_pos;
            }) as Box<dyn FnMut(web_sys::Event)>);

            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            self._closures.push(closure);
        }

        // Mouse move event
        {
            let state = self.state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                let mut state = state.borrow_mut();
                state.mouse_pos = (mouse_event.client_x() as f32, mouse_event.client_y() as f32);
            }) as Box<dyn FnMut(web_sys::Event)>);

            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            self._closures.push(closure);
        }

        // Mouse up event
        {
            let state = self.state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let mut state = state.borrow_mut();
                state.is_dragging = false;
                state.is_rotating = false;
            }) as Box<dyn FnMut(web_sys::Event)>);

            document
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            self._closures.push(closure);
        }

        // Prevent context menu on right click
        {
            let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                _event.prevent_default();
            }) as Box<dyn FnMut(web_sys::Event)>);

            canvas.add_event_listener_with_callback(
                "contextmenu",
                closure.as_ref().unchecked_ref(),
            )?;
            self._closures.push(closure);
        }

        // Wheel event for zooming
        {
            let state = self.state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let wheel_event = event.dyn_into::<WheelEvent>().unwrap();
                wheel_event.prevent_default();

                let mut state = state.borrow_mut();
                state.zoom_delta = -wheel_event.delta_y() as f32;
            }) as Box<dyn FnMut(web_sys::Event)>);

            canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
            self._closures.push(closure);
        }

        // Keyboard events
        {
            let state = self.state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let keyboard_event = event.dyn_into::<KeyboardEvent>().unwrap();
                let mut state = state.borrow_mut();

                match keyboard_event.code().as_str() {
                    "Space" => {
                        keyboard_event.prevent_default();
                        state.pause_pressed = true;
                    }
                    "KeyR" => {
                        state.reset_pressed = true;
                    }
                    _ => {}
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            self._closures.push(closure);
        }

        console_log!("Input event listeners set up");
        Ok(())
    }

    pub fn update_camera(&self, camera: &mut crate::camera::Camera) {
        let mut state = self.state.borrow_mut();

        // Handle 3D rotation (default behavior)
        if state.is_rotating {
            let delta_x = state.mouse_pos.0 - state.last_mouse_pos.0;
            let delta_y = state.mouse_pos.1 - state.last_mouse_pos.1;

            if delta_x.abs() > 0.1 || delta_y.abs() > 0.1 {
                camera.rotate(delta_x * 0.01, delta_y * 0.01);
                state.last_mouse_pos = state.mouse_pos;
            }
        }

        // Handle dragging (panning)
        if state.is_dragging {
            let delta_x = state.mouse_pos.0 - state.last_mouse_pos.0;
            let delta_y = state.mouse_pos.1 - state.last_mouse_pos.1;

            if delta_x.abs() > 0.1 || delta_y.abs() > 0.1 {
                camera.pan(delta_x, delta_y);
                state.last_mouse_pos = state.mouse_pos;
            }
        }

        // Handle zooming
        if state.zoom_delta.abs() > 0.1 {
            camera.zoom(state.zoom_delta);
            state.zoom_delta = 0.0;
        }

        // Handle reset
        if state.reset_pressed {
            camera.reset();
            state.reset_pressed = false;
            console_log!("Camera reset");
        }
    }

    pub fn is_running(&self) -> bool {
        self.state.borrow().running
    }

    pub fn pause_toggled(&self) -> bool {
        let mut state = self.state.borrow_mut();
        if state.pause_pressed {
            state.pause_pressed = false;
            state.running = !state.running;
            true
        } else {
            false
        }
    }
}
