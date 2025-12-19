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

macro_rules! config_matrix_pins_stm32 {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new($p.$out_pin, embassy_stm32::gpio::Level::Low, embassy_stm32::gpio::Speed::VeryHigh)), +];
            let input_pins = [$(Input::new($p.$in_pin, embassy_stm32::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
