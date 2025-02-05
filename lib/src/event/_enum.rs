use std::os::raw::c_void;

use sdl2::{
    controller::{Axis, Button},
    event::{DisplayEvent, WindowEvent},
    joystick::HatState,
    keyboard::{Keycode, Mod, Scancode},
    mouse::{MouseButton, MouseState, MouseWheelDirection},
};

#[derive(Debug, Clone)]
pub enum Event {
    ElementMove {
        x: f32,
        y: f32,
    },
    ElementResize {
        width: f32,
        height: f32,
    },
    //--------------------
    Quit,
    AppTerminating,
    AppLowMemory,
    AppWillEnterBackground,
    AppDidEnterBackground,
    AppWillEnterForeground,
    AppDidEnterForeground,
    Display {
        display_index: i32,
        display_event: DisplayEvent,
    },
    Window {
        win_event: WindowEvent,
    },
    KeyDown {
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    },
    KeyUp {
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    },
    TextEditing {
        text: String,
        start: i32,
        length: i32,
    },
    TextInput {
        text: String,
    },
    MouseMotion {
        which: u32,
        mousestate: MouseState,
        x: f32,
        y: f32,
        moved_x: f32,
        moved_y: f32,
    },
    MouseButtonDown {
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: f32,
        y: f32,
    },
    MouseButtonUp {
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: f32,
        y: f32,
    },
    MouseWheel {
        which: u32,
        scroll_x: f32,
        scroll_y: f32,
        direction: MouseWheelDirection,
        mouse_x: f32,
        mouse_y: f32,
    },
    JoyAxisMotion {
        which: u32,
        axis_idx: u8,
        value: i16,
    },
    JoyBallMotion {
        which: u32,
        ball_idx: u8,
        moved_x: f32,
        moved_y: f32,
    },
    JoyHatMotion {
        which: u32,
        hat_idx: u8,
        state: HatState,
    },
    JoyButtonDown {
        which: u32,
        button_idx: u8,
    },
    JoyButtonUp {
        which: u32,
        button_idx: u8,
    },
    JoyDeviceAdded {
        which: u32,
    },
    JoyDeviceRemoved {
        which: u32,
    },
    ControllerAxisMotion {
        which: u32,
        axis: Axis,
        value: i16,
    },
    ControllerButtonDown {
        which: u32,
        button: Button,
    },
    ControllerButtonUp {
        which: u32,
        button: Button,
    },
    ControllerDeviceAdded {
        which: u32,
    },
    ControllerDeviceRemoved {
        which: u32,
    },
    ControllerDeviceRemapped {
        which: u32,
    },
    ControllerTouchpadDown {
        which: u32,
        touchpad: u32,
        finger: u32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    ControllerTouchpadMotion {
        which: u32,
        touchpad: u32,
        finger: u32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    ControllerTouchpadUp {
        which: u32,
        touchpad: u32,
        finger: u32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    FingerDown {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerUp {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerMotion {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    DollarGesture {
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32,
    },
    DollarRecord {
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32,
    },
    MultiGesture {
        touch_id: i64,
        d_theta: f32,
        d_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16,
    },
    ClipboardUpdate,
    DropFile {
        filename: String,
    },
    DropText {
        filename: String,
    },
    DropBegin,
    DropComplete,
    AudioDeviceAdded {
        which: u32,
        iscapture: bool,
    },
    AudioDeviceRemoved {
        which: u32,
        iscapture: bool,
    },
    RenderDeviceReset,
    RenderTargetsReset,
    Unknown {
        type_: u32,
    },
    User {
        type_: u32,
        code: i32,
        data1: *mut c_void,
        data2: *mut c_void,
    },
}
