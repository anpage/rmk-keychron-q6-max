// Copyright (C) 2025 Alex Page
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo};
pub(crate) const COL: usize = 20;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 4;

// TODO: Macros for macOS: screenshot, Siri
// TODO: Macros for Windows: Task View, File Explorer, Cortana/Copilot
// TODO: Bluetooth host, 2.4GHz, and battey level keybinds

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0: Base layer (Mac)
        layer!([
            [k!(Escape),      k!(BrightnessDown), k!(BrightnessUp),  k!(MissionControl), k!(Launchpad),      k!(RgbVad),      k!(RgbVai),        k!(MediaPrevTrack), k!(MediaPlayPause), k!(MediaNextTrack), k!(AudioMute),   k!(AudioVolDown), k!(AudioVolUp),   k!(AudioMute),   a!(No/*Screenshot*/), a!(No/*Siri*/),  k!(RgbModeForward), k!(F13),         k!(F14),         k!(F15)        ],
            [k!(Grave),       k!(Kc1),            k!(Kc2),           k!(Kc3),            k!(Kc4),            k!(Kc5),         k!(Kc6),           k!(Kc7),            k!(Kc8),            k!(Kc9),            k!(Kc0),         k!(Minus),        k!(Equal),        k!(Backspace),   k!(Insert),           k!(Home),        k!(PageUp),         k!(NumLock),     k!(KpSlash),     k!(KpAsterisk) ],
            [k!(Tab),         k!(Q),              k!(W),             k!(E),              k!(R),              k!(T),           k!(Y),             k!(U),              k!(I),              k!(O),              k!(P),           k!(LeftBracket),  k!(RightBracket), k!(Backslash),   k!(Delete),           k!(End),         k!(PageDown),       k!(Kp7),         k!(Kp8),         k!(Kp9)        ],
            [k!(CapsLock),    k!(A),              k!(S),             k!(D),              k!(F),              k!(G),           k!(H),             k!(J),              k!(K),              k!(L),              k!(Semicolon),   k!(Quote),        k!(Enter),        k!(F16),         k!(KpMinus),          k!(KpPlus),      k!(KpEnter),        k!(Kp4),         k!(Kp5),         k!(Kp6)        ],
            [k!(LShift),      a!(No),             k!(Z),             k!(X),              k!(C),              k!(V),           k!(B),             k!(N),              k!(M),              k!(Comma),          k!(Dot),         k!(Slash),        a!(No),           k!(RShift),      a!(No),               k!(Up),          a!(No),             k!(Kp1),         k!(Kp2),         k!(Kp3)        ],
            [k!(LCtrl),       k!(LAlt),           k!(LGui),          a!(No),             a!(No),             a!(No),          k!(Space),         a!(No),             a!(No),             a!(No),             k!(RGui),        k!(RAlt),         mo!(1),           k!(RCtrl),       k!(Left),             k!(Down),        k!(Right),          a!(No),          k!(Kp0),         k!(KpDot)      ]
        ]),

        // Layer 1: Fn layer (Mac)
        layer!([
            [a!(Transparent), k!(F1),             k!(F2),            k!(F3),             k!(F4),             k!(F5),          k!(F6),            k!(F7),             k!(F8),             k!(F9),             k!(F10),         k!(F11),          k!(F12),          k!(RgbTog),      a!(Transparent),      a!(Transparent), k!(RgbTog),         a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), k!(No/*BtHost1*/),  a!(No/*BtHost2*/), a!(No/*BtHost3*/),  a!(No/*2.4GHz*/),   a!(Transparent), a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent),  a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(RgbTog),      k!(RgbModeForward), k!(RgbVai),        k!(RgbVai),         k!(RgbHui),         k!(RgbSpi),      a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent),  a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), k!(RgbModeReverse), k!(RgbVad),        k!(RgbVad),         k!(RgbHud),         k!(RgbSpd),      a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent),  a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),             a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent), a!(No/*BattLvl*/), a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(No),           a!(Transparent), a!(No),               a!(Transparent), a!(No),             a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent),    a!(Transparent),   a!(No),             a!(No),             a!(No),          a!(Transparent),   a!(No),             a!(No),             a!(No),             a!(Transparent), a!(Transparent),  a!(No),           a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(No),          a!(Transparent), a!(Transparent)]
        ]),

        // Layer 2: Base layer (Windows)
        layer!([
            [k!(Escape),      k!(F1),             k!(F2),            k!(F3),             k!(F4),             k!(F5),          k!(F6),            k!(F7),             k!(F8),             k!(F9),             k!(F10),         k!(F11),          k!(F12),          k!(AudioMute),   k!(PrintScreen),      k!(ScrollLock),  k!(Pause),          a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(Grave),       k!(Kc1),            k!(Kc2),           k!(Kc3),            k!(Kc4),            k!(Kc5),         k!(Kc6),           k!(Kc7),            k!(Kc8),            k!(Kc9),            k!(Kc0),         k!(Minus),        k!(Equal),        k!(Backspace),   k!(Insert),           k!(Home),        k!(PageUp),         k!(NumLock),     k!(KpSlash),     k!(KpAsterisk) ],
            [k!(Tab),         k!(Q),              k!(W),             k!(E),              k!(R),              k!(T),           k!(Y),             k!(U),              k!(I),              k!(O),              k!(P),           k!(LeftBracket),  k!(RightBracket), k!(Backslash),   k!(Delete),           k!(End),         k!(PageDown),       k!(Kp7),         k!(Kp8),         k!(Kp9)        ],
            [k!(CapsLock),    k!(A),              k!(S),             k!(D),              k!(F),              k!(G),           k!(H),             k!(J),              k!(K),              k!(L),              k!(Semicolon),   k!(Quote),        k!(Enter),        a!(Transparent), k!(KpMinus),          k!(KpPlus),      k!(KpEnter),        k!(Kp4),         k!(Kp5),         k!(Kp6)        ],
            [k!(LShift),      a!(No),             k!(Z),             k!(X),              k!(C),              k!(V),           k!(B),             k!(N),              k!(M),              k!(Comma),          k!(Dot),         k!(Slash),        a!(No),           k!(RShift),      a!(No),               k!(Up),          a!(No),             k!(Kp1),         k!(Kp2),         k!(Kp3)        ],
            [k!(LCtrl),       k!(LGui),           k!(LAlt),          a!(No),             a!(No),             a!(No),          k!(Space),         a!(No),             a!(No),             a!(No),             k!(RAlt),        k!(RGui),         mo!(3),           k!(RCtrl),       k!(Left),             k!(Down),        k!(Right),          a!(No),          k!(Kp0),         k!(KpDot)      ]
        ]),

        // Layer 3: Fn layer (Windows)
        layer!([
            [a!(Transparent), k!(BrightnessDown), k!(BrightnessUp),  a!(No/*TaskView*/), a!(No/*Explorer*/), k!(RgbVad),      k!(RgbVai),        k!(MediaPrevTrack), k!(MediaPlayPause), k!(MediaNextTrack), k!(AudioMute),   k!(AudioVolDown), k!(AudioVolUp),   k!(RgbTog),      a!(Transparent),      a!(Transparent), k!(RgbTog),         a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), k!(No/*BtHost1*/),  a!(No/*BtHost2*/), a!(No/*BtHost3*/),  a!(No/*2.4GHz*/),   a!(Transparent), a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent),  a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(RgbTog),      k!(RgbModeForward), k!(RgbVai),        k!(RgbVai),         k!(RgbHui),         k!(RgbSpi),      a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent),  a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), k!(RgbModeReverse), k!(RgbVad),        k!(RgbVad),         k!(RgbHud),         k!(RgbSpd),      a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent),  a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),             a!(Transparent),   a!(Transparent),    a!(Transparent),    a!(Transparent), a!(No/*BattLvl*/), a!(Transparent),    a!(Transparent),    a!(Transparent),    a!(Transparent), a!(Transparent),  a!(No),           a!(Transparent), a!(No),               a!(Transparent), a!(No),             a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent),    a!(Transparent),   a!(No),             a!(No),             a!(No),          a!(Transparent),   a!(No),             a!(No),             a!(No),             a!(Transparent), a!(Transparent),  a!(No),           a!(Transparent), a!(Transparent),      a!(Transparent), a!(Transparent),    a!(No),          a!(Transparent), a!(Transparent)]
        ]),
    ]
}
