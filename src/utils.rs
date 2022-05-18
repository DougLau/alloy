use gelatin::glium::glutin::event::VirtualKeyCode;

pub fn virtual_keycode_is_char(vk: VirtualKeyCode) -> bool {
    #[allow(clippy::match_like_matches_macro)]
    match vk {
        VirtualKeyCode::Key1 => true,
        VirtualKeyCode::Key2 => true,
        VirtualKeyCode::Key3 => true,
        VirtualKeyCode::Key4 => true,
        VirtualKeyCode::Key5 => true,
        VirtualKeyCode::Key6 => true,
        VirtualKeyCode::Key7 => true,
        VirtualKeyCode::Key8 => true,
        VirtualKeyCode::Key9 => true,
        VirtualKeyCode::Key0 => true,
        VirtualKeyCode::A => true,
        VirtualKeyCode::B => true,
        VirtualKeyCode::C => true,
        VirtualKeyCode::D => true,
        VirtualKeyCode::E => true,
        VirtualKeyCode::F => true,
        VirtualKeyCode::G => true,
        VirtualKeyCode::H => true,
        VirtualKeyCode::I => true,
        VirtualKeyCode::J => true,
        VirtualKeyCode::K => true,
        VirtualKeyCode::L => true,
        VirtualKeyCode::M => true,
        VirtualKeyCode::N => true,
        VirtualKeyCode::O => true,
        VirtualKeyCode::P => true,
        VirtualKeyCode::Q => true,
        VirtualKeyCode::R => true,
        VirtualKeyCode::S => true,
        VirtualKeyCode::T => true,
        VirtualKeyCode::U => true,
        VirtualKeyCode::V => true,
        VirtualKeyCode::W => true,
        VirtualKeyCode::X => true,
        VirtualKeyCode::Y => true,
        VirtualKeyCode::Z => true,
        VirtualKeyCode::Space => true,
        VirtualKeyCode::Caret => true,
        VirtualKeyCode::Numpad0 => true,
        VirtualKeyCode::Numpad1 => true,
        VirtualKeyCode::Numpad2 => true,
        VirtualKeyCode::Numpad3 => true,
        VirtualKeyCode::Numpad4 => true,
        VirtualKeyCode::Numpad5 => true,
        VirtualKeyCode::Numpad6 => true,
        VirtualKeyCode::Numpad7 => true,
        VirtualKeyCode::Numpad8 => true,
        VirtualKeyCode::Numpad9 => true,
        VirtualKeyCode::AbntC1 => true,
        VirtualKeyCode::AbntC2 => true,
        VirtualKeyCode::Apostrophe => true,
        VirtualKeyCode::Asterisk => true,
        VirtualKeyCode::At => true,
        VirtualKeyCode::Backslash => true,
        VirtualKeyCode::Colon => true,
        VirtualKeyCode::Comma => true,
        VirtualKeyCode::Equals => true,
        VirtualKeyCode::Grave => true,
        VirtualKeyCode::LBracket => true,
        VirtualKeyCode::Minus => true,
        VirtualKeyCode::NumpadAdd => true,
        VirtualKeyCode::NumpadComma => true,
        VirtualKeyCode::NumpadDivide => true,
        VirtualKeyCode::NumpadEnter => true,
        VirtualKeyCode::NumpadEquals => true,
        VirtualKeyCode::NumpadMultiply => true,
        VirtualKeyCode::NumpadSubtract => true,
        VirtualKeyCode::Period => true,
        VirtualKeyCode::Plus => true,
        VirtualKeyCode::RBracket => true,
        VirtualKeyCode::Semicolon => true,
        VirtualKeyCode::Slash => true,
        VirtualKeyCode::Tab => true,
        VirtualKeyCode::Underline => true,
        VirtualKeyCode::Yen => true,
        _ => false,
    }
}

pub fn virtual_keycode_to_string(vk: VirtualKeyCode) -> String {
    match vk {
        VirtualKeyCode::Key1 => "Key1".into(),
        VirtualKeyCode::Key2 => "Key2".into(),
        VirtualKeyCode::Key3 => "Key3".into(),
        VirtualKeyCode::Key4 => "Key4".into(),
        VirtualKeyCode::Key5 => "Key5".into(),
        VirtualKeyCode::Key6 => "Key6".into(),
        VirtualKeyCode::Key7 => "Key7".into(),
        VirtualKeyCode::Key8 => "Key8".into(),
        VirtualKeyCode::Key9 => "Key9".into(),
        VirtualKeyCode::Key0 => "Key0".into(),
        VirtualKeyCode::A => "A".into(),
        VirtualKeyCode::B => "B".into(),
        VirtualKeyCode::C => "C".into(),
        VirtualKeyCode::D => "D".into(),
        VirtualKeyCode::E => "E".into(),
        VirtualKeyCode::F => "F".into(),
        VirtualKeyCode::G => "G".into(),
        VirtualKeyCode::H => "H".into(),
        VirtualKeyCode::I => "I".into(),
        VirtualKeyCode::J => "J".into(),
        VirtualKeyCode::K => "K".into(),
        VirtualKeyCode::L => "L".into(),
        VirtualKeyCode::M => "M".into(),
        VirtualKeyCode::N => "N".into(),
        VirtualKeyCode::O => "O".into(),
        VirtualKeyCode::P => "P".into(),
        VirtualKeyCode::Q => "Q".into(),
        VirtualKeyCode::R => "R".into(),
        VirtualKeyCode::S => "S".into(),
        VirtualKeyCode::T => "T".into(),
        VirtualKeyCode::U => "U".into(),
        VirtualKeyCode::V => "V".into(),
        VirtualKeyCode::W => "W".into(),
        VirtualKeyCode::X => "X".into(),
        VirtualKeyCode::Y => "Y".into(),
        VirtualKeyCode::Z => "Z".into(),
        VirtualKeyCode::Escape => "Escape".into(),
        VirtualKeyCode::F1 => "F1".into(),
        VirtualKeyCode::F2 => "F2".into(),
        VirtualKeyCode::F3 => "F3".into(),
        VirtualKeyCode::F4 => "F4".into(),
        VirtualKeyCode::F5 => "F5".into(),
        VirtualKeyCode::F6 => "F6".into(),
        VirtualKeyCode::F7 => "F7".into(),
        VirtualKeyCode::F8 => "F8".into(),
        VirtualKeyCode::F9 => "F9".into(),
        VirtualKeyCode::F10 => "F10".into(),
        VirtualKeyCode::F11 => "F11".into(),
        VirtualKeyCode::F12 => "F12".into(),
        VirtualKeyCode::F13 => "F13".into(),
        VirtualKeyCode::F14 => "F14".into(),
        VirtualKeyCode::F15 => "F15".into(),
        VirtualKeyCode::F16 => "F16".into(),
        VirtualKeyCode::F17 => "F17".into(),
        VirtualKeyCode::F18 => "F18".into(),
        VirtualKeyCode::F19 => "F19".into(),
        VirtualKeyCode::F20 => "F20".into(),
        VirtualKeyCode::F21 => "F21".into(),
        VirtualKeyCode::F22 => "F22".into(),
        VirtualKeyCode::F23 => "F23".into(),
        VirtualKeyCode::F24 => "F24".into(),
        VirtualKeyCode::Snapshot => "Snapshot".into(),
        VirtualKeyCode::Scroll => "Scroll".into(),
        VirtualKeyCode::Pause => "Pause".into(),
        VirtualKeyCode::Insert => "Insert".into(),
        VirtualKeyCode::Home => "Home".into(),
        VirtualKeyCode::Delete => "Delete".into(),
        VirtualKeyCode::End => "End".into(),
        VirtualKeyCode::PageDown => "PageDown".into(),
        VirtualKeyCode::PageUp => "PageUp".into(),
        VirtualKeyCode::Left => "Left".into(),
        VirtualKeyCode::Up => "Up".into(),
        VirtualKeyCode::Right => "Right".into(),
        VirtualKeyCode::Down => "Down".into(),
        VirtualKeyCode::Back => "Back".into(),
        VirtualKeyCode::Return => "Return".into(),
        VirtualKeyCode::Space => "Space".into(),
        VirtualKeyCode::Compose => "Compose".into(),
        VirtualKeyCode::Caret => "Caret".into(),
        VirtualKeyCode::Numlock => "Numlock".into(),
        VirtualKeyCode::Numpad0 => "Numpad0".into(),
        VirtualKeyCode::Numpad1 => "Numpad1".into(),
        VirtualKeyCode::Numpad2 => "Numpad2".into(),
        VirtualKeyCode::Numpad3 => "Numpad3".into(),
        VirtualKeyCode::Numpad4 => "Numpad4".into(),
        VirtualKeyCode::Numpad5 => "Numpad5".into(),
        VirtualKeyCode::Numpad6 => "Numpad6".into(),
        VirtualKeyCode::Numpad7 => "Numpad7".into(),
        VirtualKeyCode::Numpad8 => "Numpad8".into(),
        VirtualKeyCode::Numpad9 => "Numpad9".into(),
        VirtualKeyCode::AbntC1 => "AbntC1".into(),
        VirtualKeyCode::AbntC2 => "AbntC2".into(),
        VirtualKeyCode::Apostrophe => "Apostrophe".into(),
        VirtualKeyCode::Apps => "Apps".into(),
        VirtualKeyCode::Asterisk => "Asterisk".into(),
        VirtualKeyCode::At => "At".into(),
        VirtualKeyCode::Ax => "Ax".into(),
        VirtualKeyCode::Backslash => "Backslash".into(),
        VirtualKeyCode::Calculator => "Calculator".into(),
        VirtualKeyCode::Capital => "Capital".into(),
        VirtualKeyCode::Colon => "Colon".into(),
        VirtualKeyCode::Comma => "Comma".into(),
        VirtualKeyCode::Convert => "Convert".into(),
        VirtualKeyCode::Equals => "Equals".into(),
        VirtualKeyCode::Grave => "Grave".into(),
        VirtualKeyCode::Kana => "Kana".into(),
        VirtualKeyCode::Kanji => "Kanji".into(),
        VirtualKeyCode::LAlt => "LAlt".into(),
        VirtualKeyCode::LBracket => "LBracket".into(),
        VirtualKeyCode::LControl => "LControl".into(),
        VirtualKeyCode::LShift => "LShift".into(),
        VirtualKeyCode::LWin => "LWin".into(),
        VirtualKeyCode::Mail => "Mail".into(),
        VirtualKeyCode::MediaSelect => "MediaSelect".into(),
        VirtualKeyCode::MediaStop => "MediaStop".into(),

        // Handling the alphanumeric and the numpad version of `-`
        // as the same key.
        // Alphanumeric is `Minus`, the numapd is `NumpadSubtract`
        VirtualKeyCode::Minus => "Subtract".into(),

        VirtualKeyCode::Mute => "Mute".into(),
        VirtualKeyCode::MyComputer => "MyComputer".into(),
        VirtualKeyCode::NavigateBackward => "NavigateBackward".into(),
        VirtualKeyCode::NavigateForward => "NavigateForward".into(),
        VirtualKeyCode::NextTrack => "NextTrack".into(),
        VirtualKeyCode::NoConvert => "NoConvert".into(),
        VirtualKeyCode::NumpadAdd => "Add".into(),
        VirtualKeyCode::NumpadComma => "NumpadComma".into(),
        VirtualKeyCode::NumpadDecimal => "Decimal".into(),
        VirtualKeyCode::NumpadDivide => "Divide".into(),
        VirtualKeyCode::NumpadEnter => "NumpadEnter".into(),
        VirtualKeyCode::NumpadEquals => "NumpadEquals".into(),
        VirtualKeyCode::NumpadMultiply => "Multiply".into(),
        VirtualKeyCode::NumpadSubtract => "Subtract".into(),
        VirtualKeyCode::OEM102 => "OEM102".into(),
        VirtualKeyCode::Period => "Period".into(),
        VirtualKeyCode::PlayPause => "PlayPause".into(),

        // Handling the alphanumeric and the numpad version of `+`
        // as the same key.
        // Alphanumeric is `Plus`, the numapd is `NumpadAdd`
        VirtualKeyCode::Plus => "Add".into(),

        VirtualKeyCode::Power => "Power".into(),
        VirtualKeyCode::PrevTrack => "PrevTrack".into(),
        VirtualKeyCode::RAlt => "RAlt".into(),
        VirtualKeyCode::RBracket => "RBracket".into(),
        VirtualKeyCode::RControl => "RControl".into(),
        VirtualKeyCode::RShift => "RShift".into(),
        VirtualKeyCode::RWin => "RWin".into(),
        VirtualKeyCode::Semicolon => "Semicolon".into(),
        VirtualKeyCode::Slash => "Slash".into(),
        VirtualKeyCode::Sleep => "Sleep".into(),
        VirtualKeyCode::Stop => "Stop".into(),
        VirtualKeyCode::Sysrq => "Sysrq".into(),
        VirtualKeyCode::Tab => "Tab".into(),
        VirtualKeyCode::Underline => "Underline".into(),
        VirtualKeyCode::Unlabeled => "Unlabeled".into(),
        VirtualKeyCode::VolumeDown => "VolumeDown".into(),
        VirtualKeyCode::VolumeUp => "VolumeUp".into(),
        VirtualKeyCode::Wake => "Wake".into(),
        VirtualKeyCode::WebBack => "WebBack".into(),
        VirtualKeyCode::WebFavorites => "WebFavorites".into(),
        VirtualKeyCode::WebForward => "WebForward".into(),
        VirtualKeyCode::WebHome => "WebHome".into(),
        VirtualKeyCode::WebRefresh => "WebRefresh".into(),
        VirtualKeyCode::WebSearch => "WebSearch".into(),
        VirtualKeyCode::WebStop => "WebStop".into(),
        VirtualKeyCode::Yen => "Yen".into(),
        VirtualKeyCode::Copy => "Copy".into(),
        VirtualKeyCode::Paste => "Paste".into(),
        VirtualKeyCode::Cut => "Cut".into(),
    }
}
