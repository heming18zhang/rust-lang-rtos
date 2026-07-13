// =====================================================
// Host Simulator
//
// Windows/Linux 测试 ARTOS
//
// 这里可以使用 std。
// 因为它不是 MCU kernel。
//
// =====================================================

use std::time::Instant;

use artos_core::{
    executor::Executor,
    timer::{Clock, Sleep},
};

// =====================================================
// 模拟 MCU Timer
//
// STM32:
//
//     TIM counter
//
// Windows:
//
//     Instant
//
// =====================================================

struct HostClock {
    start: Instant,
}

impl HostClock {
    fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Clock for HostClock {
    fn now(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

// =====================================================
// 第一个 async task
//
// 这就是未来 MCU:
//
// led_task()
// uart_task()
// sensor_task()
//
// =====================================================

async fn led_task(clock: &HostClock) {
    loop {
        println!("LED ON");

        Sleep::new(clock, 1000).await;

        println!("LED OFF");

        Sleep::new(clock, 1000).await;
    }
}

fn main() {
    println!("ARTOS v0.1 start");

    let clock = HostClock::new();

    let executor = Executor::new();

    executor.run(led_task(&clock));
}
