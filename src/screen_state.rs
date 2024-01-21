use std::sync::Mutex;

struct ScreenStateInner {
    scale_factor: f64,
    width: u64,
    height: u64,
    mouse: Option<(u64, u64)>,
    prev_mouse: Option<(u64, u64)>,
}

pub struct ScreenState(Mutex<ScreenStateInner>);

impl ScreenState {
    pub fn new(scale_factor: f64, width: u64, height: u64) -> Self {
        ScreenState(Mutex::new(ScreenStateInner {
            scale_factor,
            width,
            height,
            mouse: None,
            prev_mouse: None,
        }))
    }

    pub fn scale_factor(&self) -> f64 {
        self.0.lock().unwrap().scale_factor
    }

    pub fn set_scale_factor(&self, scale_factor: f64) {
        self.0.lock().unwrap().scale_factor = scale_factor;
    }

    pub fn size(&self) -> (u64, u64) {
        let guard = self.0.lock().unwrap();
        (guard.width, guard.height)
    }

    pub fn set_size(&self, width: u64, height: u64) {
        let mut guard = self.0.lock().unwrap();
        guard.width = width;
        guard.height = height;
    }

    pub fn mouse(&self) -> Option<(u64, u64)> {
        self.0.lock().unwrap().mouse
    }

    pub fn prev_mouse(&self) -> Option<(u64, u64)> {
        self.0.lock().unwrap().prev_mouse
    }

    pub fn set_mouse(&self, mouse_x: u64, mouse_y: u64) {
        let mut guard = self.0.lock().unwrap();
        guard.prev_mouse = guard.mouse;
        guard.mouse = Some((mouse_x, mouse_y));
    }

    pub fn cursor_left(&self) {
        let mut guard = self.0.lock().unwrap();
        guard.prev_mouse = None;
        guard.mouse = None;
    }
}
