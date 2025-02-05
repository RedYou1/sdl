use sdl2::event::Event as SdlEvent;

use super::_enum::Event;

impl From<SdlEvent> for Event {
    #[allow(clippy::too_many_lines)]
    fn from(value: SdlEvent) -> Self {
        match value {
            SdlEvent::Quit { .. } => Self::Quit,
            SdlEvent::AppTerminating { .. } => Self::AppTerminating,
            SdlEvent::AppLowMemory { .. } => Self::AppLowMemory,
            SdlEvent::AppWillEnterBackground { .. } => Self::AppWillEnterBackground,
            SdlEvent::AppDidEnterBackground { .. } => Self::AppDidEnterBackground,
            SdlEvent::AppWillEnterForeground { .. } => Self::AppWillEnterForeground,
            SdlEvent::AppDidEnterForeground { .. } => Self::AppDidEnterForeground,
            SdlEvent::Display {
                display_index,
                display_event,
                ..
            } => Self::Display {
                display_index,
                display_event,
            },
            SdlEvent::Window { win_event, .. } => Self::Window { win_event },
            SdlEvent::KeyDown {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => Self::KeyDown {
                keycode,
                scancode,
                keymod,
                repeat,
            },
            SdlEvent::KeyUp {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => Self::KeyUp {
                keycode,
                scancode,
                keymod,
                repeat,
            },
            SdlEvent::TextEditing {
                text,
                start,
                length,
                ..
            } => Self::TextEditing {
                text,
                start,
                length,
            },
            SdlEvent::TextInput { text, .. } => Self::TextInput { text },
            SdlEvent::MouseMotion {
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
                ..
            } => Self::MouseMotion {
                which,
                mousestate,
                x: x as f32,
                y: y as f32,
                moved_x: xrel as f32,
                moved_y: yrel as f32,
            },
            SdlEvent::MouseButtonDown {
                which,
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => Self::MouseButtonDown {
                which,
                mouse_btn,
                clicks,
                x: x as f32,
                y: y as f32,
            },
            SdlEvent::MouseButtonUp {
                which,
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => Self::MouseButtonUp {
                which,
                mouse_btn,
                clicks,
                x: x as f32,
                y: y as f32,
            },
            SdlEvent::MouseWheel {
                which,
                direction,
                precise_x,
                precise_y,
                mouse_x,
                mouse_y,
                ..
            } => Self::MouseWheel {
                which,
                scroll_x: precise_x,
                scroll_y: precise_y,
                direction,
                mouse_x: mouse_x as f32,
                mouse_y: mouse_y as f32,
            },
            SdlEvent::JoyAxisMotion {
                which,
                axis_idx,
                value,
                ..
            } => Self::JoyAxisMotion {
                which,
                axis_idx,
                value,
            },
            SdlEvent::JoyBallMotion {
                which,
                ball_idx,
                xrel,
                yrel,
                ..
            } => Self::JoyBallMotion {
                which,
                ball_idx,
                moved_x: xrel as f32,
                moved_y: yrel as f32,
            },
            SdlEvent::JoyHatMotion {
                which,
                hat_idx,
                state,
                ..
            } => Self::JoyHatMotion {
                which,
                hat_idx,
                state,
            },
            SdlEvent::JoyButtonDown {
                which, button_idx, ..
            } => Self::JoyButtonDown { which, button_idx },
            SdlEvent::JoyButtonUp {
                which, button_idx, ..
            } => Self::JoyButtonUp { which, button_idx },
            SdlEvent::JoyDeviceAdded { which, .. } => Self::JoyDeviceAdded { which },
            SdlEvent::JoyDeviceRemoved { which, .. } => Self::JoyDeviceRemoved { which },
            SdlEvent::ControllerAxisMotion {
                which, axis, value, ..
            } => Self::ControllerAxisMotion { which, axis, value },
            SdlEvent::ControllerButtonDown { which, button, .. } => {
                Self::ControllerButtonDown { which, button }
            }
            SdlEvent::ControllerButtonUp { which, button, .. } => {
                Self::ControllerButtonUp { which, button }
            }
            SdlEvent::ControllerDeviceAdded { which, .. } => Self::ControllerDeviceAdded { which },
            SdlEvent::ControllerDeviceRemoved { which, .. } => {
                Self::ControllerDeviceRemoved { which }
            }
            SdlEvent::ControllerDeviceRemapped { which, .. } => {
                Self::ControllerDeviceRemapped { which }
            }
            SdlEvent::ControllerTouchpadDown {
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
                ..
            } => Self::ControllerTouchpadDown {
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
            },
            SdlEvent::ControllerTouchpadMotion {
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
                ..
            } => Self::ControllerTouchpadMotion {
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
            },
            SdlEvent::ControllerTouchpadUp {
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
                ..
            } => Self::ControllerTouchpadUp {
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
            },
            SdlEvent::FingerDown {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
                ..
            } => Self::FingerDown {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            SdlEvent::FingerUp {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
                ..
            } => Self::FingerUp {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            SdlEvent::FingerMotion {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
                ..
            } => Self::FingerMotion {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            SdlEvent::DollarGesture {
                touch_id,
                gesture_id,
                num_fingers,
                error,
                x,
                y,
                ..
            } => Self::DollarGesture {
                touch_id,
                gesture_id,
                num_fingers,
                error,
                x,
                y,
            },
            SdlEvent::DollarRecord {
                touch_id,
                gesture_id,
                num_fingers,
                error,
                x,
                y,
                ..
            } => Self::DollarRecord {
                touch_id,
                gesture_id,
                num_fingers,
                error,
                x,
                y,
            },
            SdlEvent::MultiGesture {
                touch_id,
                d_theta,
                d_dist,
                x,
                y,
                num_fingers,
                ..
            } => Self::MultiGesture {
                touch_id,
                d_theta,
                d_dist,
                x,
                y,
                num_fingers,
            },
            SdlEvent::ClipboardUpdate { .. } => Self::ClipboardUpdate,
            SdlEvent::DropFile { filename, .. } => Self::DropFile { filename },
            SdlEvent::DropText { filename, .. } => Self::DropText { filename },
            SdlEvent::DropBegin { .. } => Self::DropBegin,
            SdlEvent::DropComplete { .. } => Self::DropComplete,
            SdlEvent::AudioDeviceAdded {
                which, iscapture, ..
            } => Self::AudioDeviceAdded { which, iscapture },
            SdlEvent::AudioDeviceRemoved {
                which, iscapture, ..
            } => Self::AudioDeviceRemoved { which, iscapture },
            SdlEvent::RenderDeviceReset { .. } => Self::RenderDeviceReset,
            SdlEvent::RenderTargetsReset { .. } => Self::RenderTargetsReset,
            SdlEvent::Unknown { type_, .. } => Self::Unknown { type_ },
            SdlEvent::User {
                type_,
                code,
                data1,
                data2,
                ..
            } => Self::User {
                type_,
                code,
                data1,
                data2,
            },
        }
    }
}
