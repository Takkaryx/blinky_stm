#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod modules {
    pub mod heartbeat;
}

use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pin, Pull};
use modules::heartbeat::heartbeat_start;
use panic_halt as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());

    // Start up a blinking LED so we know we're running
    // PD14 is the RED led
    let _ = heartbeat_start(spawner, p.PD14.degrade());

    let button = Input::new(p.PA0, Pull::None);
    let mut button = ExtiInput::new(button, p.EXTI0);

    loop {
        // Check if button got pressed
        button.wait_for_rising_edge().await;
    }
}
