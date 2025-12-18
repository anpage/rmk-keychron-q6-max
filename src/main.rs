#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;
mod vial;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32::rcc;
use embassy_stm32::rcc::mux;
use embassy_stm32::usb::{Driver, InterruptHandler};
use embassy_stm32::{Config, bind_interrupts, time::Hertz};
use keymap::{COL, ROW};
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{BehaviorConfig, PositionalConfig, RmkConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join3;
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::{initialize_keymap, run_devices, run_rmk};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    OTG_FS => InterruptHandler<USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    // RCC config
    let mut config = Config::default();

    config.rcc.hse = Some(rcc::Hse {
        freq: Hertz(16_000_000),
        mode: rcc::HseMode::Oscillator,
    });
    config.rcc.pll_src = rcc::PllSource::HSE;
    config.rcc.pll = Some(rcc::Pll {
        prediv: rcc::PllPreDiv::DIV8,
        mul: rcc::PllMul::MUL96,
        divp: Some(rcc::PllPDiv::DIV4),
        divq: Some(rcc::PllQDiv::DIV4),
        divr: None,
    });
    config.rcc.sys = rcc::Sysclk::PLL1_P;
    config.rcc.ahb_pre = rcc::AHBPrescaler::DIV1;
    config.rcc.apb1_pre = rcc::APBPrescaler::DIV4;
    config.rcc.apb2_pre = rcc::APBPrescaler::DIV2;
    config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;

    // Initialize peripherals
    let p = embassy_stm32::init(config);

    // Usb config
    static EP_OUT_BUFFER: StaticCell<[u8; 1024]> = StaticCell::new();
    let mut usb_config = embassy_stm32::usb::Config::default();
    usb_config.vbus_detection = false;
    let driver = Driver::new_fs(
        p.USB_OTG_FS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut EP_OUT_BUFFER.init([0; 1024])[..],
        usb_config,
    );

    // Pin config
    let (col_pins, row_pins) = config_matrix_pins_stm32!(
        peripherals: p,
        input: [PC6, PC7, PC8, PA13, PA14, PA15, PC10, PC11, PC13, PC14, PC15, PC0, PC1, PC2, PC3, PA0, PA1, PA2, PA3, PB10],
        output: [PC12, PD2, PB3, PB4, PB5, PB6]
    );

    // Keyboard config
    let rmk_config = RmkConfig {
        vial_config: VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (1, 1)]),
        ..Default::default()
    };

    // Initialize the keymap
    let mut default_keymap = keymap::get_default_keymap();
    let mut behavior_config = BehaviorConfig::default();
    let mut per_key_config = PositionalConfig::default();
    let keymap = initialize_keymap(
        &mut default_keymap,
        &mut behavior_config,
        &mut per_key_config,
    )
    .await;

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::<_, _, _, ROW, COL, false>::new(row_pins, col_pins, debouncer);
    let mut keyboard = Keyboard::new(&keymap);

    // Start
    join3(
        run_devices! (
            (matrix) => EVENT_CHANNEL,
        ),
        keyboard.run(),
        run_rmk(&keymap, driver, rmk_config),
    )
    .await;
}
