use core::sync::atomic::{AtomicU32, Ordering};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::{Duration, Timer};

pub struct LedStruct {
    pin: AnyPin,
    del: AtomicU32,
}

impl LedStruct {
    pub fn new(pin: AnyPin, delay: u32) -> Self {
        let mut led_struct = LedStruct {
            pin,
            del: AtomicU32::new(0),
        };
        LedStruct::set_delay(&mut led_struct, delay);
        led_struct
    }

    pub fn set_delay(&mut self, new_delay: u32) {
        self.del.store(new_delay, Ordering::Relaxed);
    }
}

#[embassy_executor::task]
async fn heartbeat(led_struct: LedStruct) {
    let mut led = Output::new(led_struct.pin, Level::Low, Speed::Low);
    loop {
        let del = led_struct.del.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(del.into())).await;
        led.toggle();
    }
}

pub fn heartbeat_start(spawner: Spawner, pin: AnyPin) -> Result<(), embassy_executor::SpawnError> {
    let heart = LedStruct::new(pin, 1000);
    spawner.spawn(heartbeat(heart))
}
