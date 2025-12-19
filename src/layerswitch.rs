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

use embedded_hal::digital::InputPin;
use rmk::{
    event::{Event, KeyboardEvent},
    input_device::InputDevice,
};

pub struct LayerSwitch<P> {
    pin: P,
    last_state: Option<bool>,
    pending_release: Option<(u8, u8)>,
}

impl<P: InputPin> LayerSwitch<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            last_state: None,
            pending_release: None,
        }
    }
}

impl<P: InputPin> InputDevice for LayerSwitch<P> {
    async fn read_event(&mut self) -> Event {
        loop {
            if let Some((row, col)) = self.pending_release.take() {
                return Event::Key(KeyboardEvent::key(row, col, false));
            }

            let is_high = self.pin.is_high().unwrap_or(false);

            match self.last_state {
                None => {
                    self.last_state = Some(is_high);
                    let (row, col) = if is_high { (5, 3) } else { (5, 4) };
                    self.pending_release = Some((row, col));
                    return Event::Key(KeyboardEvent::key(row, col, true));
                }
                Some(last) if last != is_high => {
                    self.last_state = Some(is_high);
                    let (row, col) = if is_high { (5, 3) } else { (5, 4) };
                    self.pending_release = Some((row, col));
                    return Event::Key(KeyboardEvent::key(row, col, true));
                }
                _ => {}
            }

            embassy_time::Timer::after_millis(20).await;
        }
    }
}
