use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo};
pub(crate) const COL: usize = 20;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 4;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0: Base layer (Mac)
        layer!([
            [k!(Escape),   k!(BrightnessDown), k!(BrightnessUp), a!(No),    a!(No),    a!(No),    a!(No),      k!(MediaPrevTrack), k!(MediaStop), k!(MediaNextTrack), k!(AudioMute), k!(AudioVolDown), k!(AudioVolUp),   k!(AudioMute), a!(No),      a!(No),     a!(No),       k!(F13),     k!(F14),       k!(F15)       ],
            [k!(Grave),    k!(Kc0),            k!(Kc2),          k!(Kc3),   k!(Kc4),   k!(Kc5),   k!(Kc6),     k!(Kc7),            k!(Kc8),       k!(Kc9),            k!(Kc0),       k!(Minus),        k!(Equal),        k!(Backspace), k!(Insert),  k!(Home),   k!(PageUp),   k!(NumLock), k!(KpSlash),   k!(KpAsterisk)],
            [k!(Tab),      k!(Q),              k!(W),            k!(E),     k!(R),     k!(T),     k!(Y),       k!(U),              k!(I),         k!(O),              k!(P),         k!(LeftBracket),  k!(RightBracket), k!(Backslash), k!(Delete),  k!(End),    k!(PageDown), k!(Kp7),     k!(Kp8),       k!(Kp9)       ],
            [k!(CapsLock), k!(A),              k!(S),            k!(D),     k!(F),     k!(G),     k!(H),       k!(J),              k!(K),         k!(L),              k!(Semicolon), k!(Quote),        k!(Enter),        k!(KpEqual),   k!(KpMinus), k!(KpPlus), k!(KpEnter),  k!(Kp4),     k!(Kp5),       k!(Kp6)       ],
            [k!(LShift),   a!(No),             k!(Z),            k!(X),     k!(C),     k!(V),     k!(B),       k!(N),              k!(M),         k!(Comma),          k!(Dot),       k!(Slash),        a!(No),           k!(RShift),    a!(No),      k!(Up),     a!(No),       k!(Kp1),     k!(Kp2),       k!(Kp3)       ],
            [k!(LCtrl),    mo!(1),             k!(LAlt),         a!(No),    a!(No),    a!(No),    k!(Space),   a!(No),             a!(No),        a!(No),             k!(RAlt),      mo!(1),           mo!(1),           k!(RCtrl),     k!(Left),    k!(Down),   k!(Right),    a!(No),      k!(Kp0),       k!(KpDot)     ]
        ]),

        // Layer 1: Fn layer (Mac)
        layer!([
            [a!(Transparent), k!(F1),            k!(F2),            k!(F3),            k!(F4),            k!(F5),            k!(F6),            k!(F7),            k!(F8),            k!(F9),            k!(F10),           k!(F11),           k!(F12),           a!(No),            a!(Transparent), a!(Transparent), a!(No),          a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),            a!(No),            a!(No),            a!(No),            a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(No),          a!(No),            a!(No),            a!(No),            a!(No),            a!(No),            a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),            a!(No),            a!(No),            a!(No),            a!(No),            a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),            a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(No),            a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(No),            a!(Transparent),   a!(No),          a!(Transparent), a!(No),          a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent),   a!(Transparent),   a!(No),            a!(No),            a!(No),            a!(Transparent),   a!(No),            a!(No),            a!(No),            a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent), a!(Transparent), a!(Transparent), a!(No),          a!(Transparent), a!(Transparent)]
        ]),

        // Layer 2: Base layer (Windows)
        layer!([
            [k!(Escape),   k!(F1),   k!(F2),   k!(F3),   k!(F4),   k!(F5),   k!(F6),    k!(F7),   k!(F8),   k!(F9),    k!(F10),       k!(F11),         k!(F12),          k!(AudioMute),     k!(PrintScreen), a!(No),   a!(No),       a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(Grave),    k!(Kc1),  k!(Kc2),  k!(Kc3),  k!(Kc4),  k!(Kc5),  k!(Kc6),   k!(Kc7),  k!(Kc8),  k!(Kc9),   k!(Kc0),       k!(Minus),       k!(Equal),        k!(Backspace),     k!(Insert),      k!(Home), k!(PageUp),   k!(NumLock),     k!(KpSlash),     k!(KpAsterisk)  ],
            [k!(Tab),      k!(Q),    k!(W),    k!(E),    k!(R),    k!(T),    k!(Y),     k!(U),    k!(I),    k!(O),     k!(P),         k!(LeftBracket), k!(RightBracket), k!(Backslash),     k!(Delete),      k!(End),  k!(PageDown), k!(Kp7),         k!(Kp8),         k!(Kp9)         ],
            [k!(CapsLock), k!(A),    k!(S),    k!(D),    k!(F),    k!(G),    k!(H),     k!(J),    k!(K),    k!(L),     k!(Semicolon), k!(Quote),       k!(Enter),        a!(Transparent),   k!(KpMinus),     k!(KpPlus), k!(KpEnter), k!(Kp4),         k!(Kp5),         k!(Kp6)         ],
            [k!(LShift),   a!(No),   k!(Z),    k!(X),    k!(C),    k!(V),    k!(B),     k!(N),    k!(M),    k!(Comma), k!(Dot),       k!(Slash),       a!(No),           k!(RShift),        a!(No),          k!(Up),   a!(No),       k!(Kp1),         k!(Kp2),         k!(Kp3)         ],
            [k!(LCtrl),    k!(LGui), k!(LAlt), a!(No),   a!(No),   a!(No),   k!(Space), a!(No),   a!(No),   a!(No),    k!(RAlt),      k!(RGui),        mo!(3),           k!(RCtrl),         k!(Left),        k!(Down), k!(Right),    a!(No),          k!(Kp0),         k!(KpDot)       ]
        ]),

        // Layer 3: Fn layer (Windows)
        layer!([
            [a!(Transparent), k!(BrightnessDown), k!(BrightnessUp), a!(No),  a!(No),  a!(No),  a!(No),          k!(MediaPrevTrack), k!(MediaStop), k!(MediaNextTrack), k!(AudioMute),   k!(AudioVolDown), k!(AudioVolUp),  a!(No),          a!(Transparent), a!(Transparent), a!(No),          a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),             a!(No),           a!(No),  a!(No),  a!(Transparent), a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(No),          a!(No),             a!(No),           a!(No),  a!(No),  a!(No),          a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),             a!(No),           a!(No),  a!(No),  a!(No),          a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),  a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(No),             a!(Transparent),  a!(Transparent), a!(Transparent), a!(Transparent), a!(No),          a!(Transparent),    a!(Transparent), a!(Transparent),    a!(Transparent), a!(Transparent),  a!(No),          a!(Transparent), a!(No),          a!(Transparent), a!(No),          a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent),    a!(Transparent),  a!(No),  a!(No),  a!(No),          a!(Transparent), a!(No),             a!(No),          a!(No),             a!(Transparent), a!(Transparent),  a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(No),          a!(Transparent), a!(Transparent)]
        ]),
    ]
}
