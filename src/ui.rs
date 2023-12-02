pub struct UI {
    key_code: iced::keyboard::KeyCode,
}

#[derive(Debug, Clone, Copy)]
pub enum UiMessage {
    KeyboardKeyPressed(iced::keyboard::KeyCode),
}

impl iced::Application for UI {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = UiMessage;
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (UI, iced::Command<Self::Message>) {
        (
            UI {
                key_code: iced::keyboard::KeyCode::Pause,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        return String::from("Steam Deck File Manager");
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        return iced::subscription::events_with(|event, _status| match event {
            iced::Event::Keyboard(keyboard_event) => match keyboard_event {
                iced::keyboard::Event::KeyPressed { key_code, .. } => {
                    return Some(UiMessage::KeyboardKeyPressed(key_code));
                }
                _ => None,
            },
            _ => None,
        });
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            UiMessage::KeyboardKeyPressed(code) => self.key_code = code,
        }

        return iced::Command::none();
    }

    fn view(&self) -> iced::Element<Self::Message> {
        // Head panel.
        let head_panel = iced::widget::row![iced::widget::text("header"),].height(32);

        // Body panel.
        let body_panel = iced::widget::text(keycode_to_string(self.key_code))
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        // Tail panel.
        let tail_panel = iced::widget::row![
            iced::widget::text("-"),
            iced::widget::text("Change View"),
            iced::widget::text("B"),
            iced::widget::text("Back"),
            iced::widget::text("A"),
            iced::widget::text("Enter"),
        ]
        .align_items(iced::Alignment::Center)
        .spacing(10);
        let tail_panel = iced::widget::container(tail_panel)
            .width(iced::Length::Fill)
            .height(32)
            .align_x(iced::alignment::Horizontal::Right);

        // Let's box them together.
        let content = iced::widget::column![
            head_panel,
            iced::widget::horizontal_rule(38),
            body_panel,
            iced::widget::horizontal_rule(38),
            tail_panel,
        ]
        .width(iced::Length::Fill);

        //return iced::widget::container(content)
        //    .width(iced::Length::Fill)
        //    .into();
        return content.into();
    }
}

fn keycode_to_string(key_code: iced::keyboard::KeyCode) -> &'static str {
    match key_code {
        iced::keyboard::KeyCode::Key1 => return "Key1",
        iced::keyboard::KeyCode::Key2 => return "Key2",
        iced::keyboard::KeyCode::Key3 => return "Key3",
        iced::keyboard::KeyCode::Key4 => return "Key4",
        iced::keyboard::KeyCode::Key5 => return "Key5",
        iced::keyboard::KeyCode::Key6 => return "Key6",
        iced::keyboard::KeyCode::Key7 => return "Key7",
        iced::keyboard::KeyCode::Key8 => return "Key8",
        iced::keyboard::KeyCode::Key9 => return "Key9",
        iced::keyboard::KeyCode::Key0 => return "Key0",
        iced::keyboard::KeyCode::A => return "A",
        iced::keyboard::KeyCode::B => return "B",
        iced::keyboard::KeyCode::C => return "C",
        iced::keyboard::KeyCode::D => return "D",
        iced::keyboard::KeyCode::E => return "E",
        iced::keyboard::KeyCode::F => return "F",
        iced::keyboard::KeyCode::G => return "G",
        iced::keyboard::KeyCode::H => return "H",
        iced::keyboard::KeyCode::I => return "I",
        iced::keyboard::KeyCode::J => return "J",
        iced::keyboard::KeyCode::K => return "K",
        iced::keyboard::KeyCode::L => return "L",
        iced::keyboard::KeyCode::M => return "M",
        iced::keyboard::KeyCode::N => return "N",
        iced::keyboard::KeyCode::O => return "O",
        iced::keyboard::KeyCode::P => return "P",
        iced::keyboard::KeyCode::Q => return "Q",
        iced::keyboard::KeyCode::R => return "R",
        iced::keyboard::KeyCode::S => return "S",
        iced::keyboard::KeyCode::T => return "T",
        iced::keyboard::KeyCode::U => return "U",
        iced::keyboard::KeyCode::V => return "V",
        iced::keyboard::KeyCode::W => return "W",
        iced::keyboard::KeyCode::X => return "X",
        iced::keyboard::KeyCode::Y => return "Y",
        iced::keyboard::KeyCode::Z => return "Z",
        iced::keyboard::KeyCode::Escape => return "Escape",
        iced::keyboard::KeyCode::F1 => return "F1",
        iced::keyboard::KeyCode::F2 => return "F2",
        iced::keyboard::KeyCode::F3 => return "F3",
        iced::keyboard::KeyCode::F4 => return "F4",
        iced::keyboard::KeyCode::F5 => return "F5",
        iced::keyboard::KeyCode::F6 => return "F6",
        iced::keyboard::KeyCode::F7 => return "F7",
        iced::keyboard::KeyCode::F8 => return "F8",
        iced::keyboard::KeyCode::F9 => return "F9",
        iced::keyboard::KeyCode::F10 => return "F10",
        iced::keyboard::KeyCode::F11 => return "F11",
        iced::keyboard::KeyCode::F12 => return "F12",
        iced::keyboard::KeyCode::F13 => return "F13",
        iced::keyboard::KeyCode::F14 => return "F14",
        iced::keyboard::KeyCode::F15 => return "F15",
        iced::keyboard::KeyCode::F16 => return "F16",
        iced::keyboard::KeyCode::F17 => return "F17",
        iced::keyboard::KeyCode::F18 => return "F18",
        iced::keyboard::KeyCode::F19 => return "F19",
        iced::keyboard::KeyCode::F20 => return "F20",
        iced::keyboard::KeyCode::F21 => return "F21",
        iced::keyboard::KeyCode::F22 => return "F22",
        iced::keyboard::KeyCode::F23 => return "F23",
        iced::keyboard::KeyCode::F24 => return "F24",
        iced::keyboard::KeyCode::Snapshot => return "Snapshot",
        iced::keyboard::KeyCode::Scroll => return "Scroll",
        iced::keyboard::KeyCode::Pause => return "Pause",
        iced::keyboard::KeyCode::Insert => return "Insert",
        iced::keyboard::KeyCode::Home => return "Home",
        iced::keyboard::KeyCode::Delete => return "Delete",
        iced::keyboard::KeyCode::End => return "End",
        iced::keyboard::KeyCode::PageDown => return "PageDown",
        iced::keyboard::KeyCode::PageUp => return "PageUp",
        iced::keyboard::KeyCode::Left => return "Left",
        iced::keyboard::KeyCode::Up => return "Up",
        iced::keyboard::KeyCode::Right => return "Right",
        iced::keyboard::KeyCode::Down => return "Down",
        iced::keyboard::KeyCode::Backspace => return "Backspace",
        iced::keyboard::KeyCode::Enter => return "Enter",
        iced::keyboard::KeyCode::Space => return "Space",
        iced::keyboard::KeyCode::Compose => return "Compose",
        iced::keyboard::KeyCode::Caret => return "Caret",
        iced::keyboard::KeyCode::Numlock => return "Numlock",
        iced::keyboard::KeyCode::Numpad0 => return "Numpad0",
        iced::keyboard::KeyCode::Numpad1 => return "Numpad1",
        iced::keyboard::KeyCode::Numpad2 => return "Numpad2",
        iced::keyboard::KeyCode::Numpad3 => return "Numpad3",
        iced::keyboard::KeyCode::Numpad4 => return "Numpad4",
        iced::keyboard::KeyCode::Numpad5 => return "Numpad5",
        iced::keyboard::KeyCode::Numpad6 => return "Numpad6",
        iced::keyboard::KeyCode::Numpad7 => return "Numpad7",
        iced::keyboard::KeyCode::Numpad8 => return "Numpad8",
        iced::keyboard::KeyCode::Numpad9 => return "Numpad9",
        iced::keyboard::KeyCode::NumpadAdd => return "NumpadAdd",
        iced::keyboard::KeyCode::NumpadDivide => return "NumpadDivide",
        iced::keyboard::KeyCode::NumpadDecimal => return "NumpadDecimal",
        iced::keyboard::KeyCode::NumpadComma => return "NumpadComma",
        iced::keyboard::KeyCode::NumpadEnter => return "NumpadEnter",
        iced::keyboard::KeyCode::NumpadEquals => return "NumpadEquals",
        iced::keyboard::KeyCode::NumpadMultiply => return "NumpadMultiply",
        iced::keyboard::KeyCode::NumpadSubtract => return "NumpadSubtract",
        iced::keyboard::KeyCode::AbntC1 => return "AbntC1",
        iced::keyboard::KeyCode::AbntC2 => return "AbntC2",
        iced::keyboard::KeyCode::Apostrophe => return "Apostrophe",
        iced::keyboard::KeyCode::Apps => return "Apps",
        iced::keyboard::KeyCode::Asterisk => return "Asterisk",
        iced::keyboard::KeyCode::At => return "At",
        iced::keyboard::KeyCode::Ax => return "Ax",
        iced::keyboard::KeyCode::Backslash => return "Backslash",
        iced::keyboard::KeyCode::Calculator => return "Calculator",
        iced::keyboard::KeyCode::Capital => return "Capital",
        iced::keyboard::KeyCode::Colon => return "Colon",
        iced::keyboard::KeyCode::Comma => return "Comma",
        iced::keyboard::KeyCode::Convert => return "Convert",
        iced::keyboard::KeyCode::Equals => return "Equals",
        iced::keyboard::KeyCode::Grave => return "Grave",
        iced::keyboard::KeyCode::Kana => return "Kana",
        iced::keyboard::KeyCode::Kanji => return "Kanji",
        iced::keyboard::KeyCode::LAlt => return "LAlt",
        iced::keyboard::KeyCode::LBracket => return "LBracket",
        iced::keyboard::KeyCode::LControl => return "LControl",
        iced::keyboard::KeyCode::LShift => return "LShift",
        iced::keyboard::KeyCode::LWin => return "LWin",
        iced::keyboard::KeyCode::Mail => return "Mail",
        iced::keyboard::KeyCode::MediaSelect => return "MediaSelect",
        iced::keyboard::KeyCode::MediaStop => return "MediaStop",
        iced::keyboard::KeyCode::Minus => return "Minus",
        iced::keyboard::KeyCode::Mute => return "Mute",
        iced::keyboard::KeyCode::MyComputer => return "MyComputer",
        iced::keyboard::KeyCode::NavigateForward => return "NavigateForward",
        iced::keyboard::KeyCode::NavigateBackward => return "NavigateBackward",
        iced::keyboard::KeyCode::NextTrack => return "NextTrack",
        iced::keyboard::KeyCode::NoConvert => return "NoConvert",
        iced::keyboard::KeyCode::OEM102 => return "OEM102",
        iced::keyboard::KeyCode::Period => return "Period",
        iced::keyboard::KeyCode::PlayPause => return "PlayPause",
        iced::keyboard::KeyCode::Plus => return "Plus",
        iced::keyboard::KeyCode::Power => return "Power",
        iced::keyboard::KeyCode::PrevTrack => return "PrevTrack",
        iced::keyboard::KeyCode::RAlt => return "RAlt",
        iced::keyboard::KeyCode::RBracket => return "RBracket",
        iced::keyboard::KeyCode::RControl => return "RControl",
        iced::keyboard::KeyCode::RShift => return "RShift",
        iced::keyboard::KeyCode::RWin => return "RWin",
        iced::keyboard::KeyCode::Semicolon => return "Semicolon",
        iced::keyboard::KeyCode::Slash => return "Slash",
        iced::keyboard::KeyCode::Sleep => return "Sleep",
        iced::keyboard::KeyCode::Stop => return "Stop",
        iced::keyboard::KeyCode::Sysrq => return "Sysrq",
        iced::keyboard::KeyCode::Tab => return "Tab",
        iced::keyboard::KeyCode::Underline => return "Underline",
        iced::keyboard::KeyCode::Unlabeled => return "Unlabeled",
        iced::keyboard::KeyCode::VolumeDown => return "VolumeDown",
        iced::keyboard::KeyCode::VolumeUp => return "VolumeUp",
        iced::keyboard::KeyCode::Wake => return "Wake",
        iced::keyboard::KeyCode::WebBack => return "WebBack",
        iced::keyboard::KeyCode::WebFavorites => return "WebFavorites",
        iced::keyboard::KeyCode::WebForward => return "WebForward",
        iced::keyboard::KeyCode::WebHome => return "WebHome",
        iced::keyboard::KeyCode::WebRefresh => return "WebRefresh",
        iced::keyboard::KeyCode::WebSearch => return "WebSearch",
        iced::keyboard::KeyCode::WebStop => return "WebStop",
        iced::keyboard::KeyCode::Yen => return "Yen",
        iced::keyboard::KeyCode::Copy => return "Copy",
        iced::keyboard::KeyCode::Paste => return "Paste",
        iced::keyboard::KeyCode::Cut => return "Cut",
    }
}
