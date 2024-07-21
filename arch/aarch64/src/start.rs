use core::arch::global_asm;

use kinglet_drivers::console::Console;
use kinglet_drivers::gpio::raspi::Gpio;
use kinglet_drivers::serial::raspi::MiniUart;

use crate::cpu;

global_asm!(include_str!("./entry.s"));

#[no_mangle]
pub(crate) fn start() -> ! {
    let mut gpio = Gpio::new(cpu::wait);
    let uart = MiniUart::init(&mut gpio);
    let mut console = Console::new(uart);

    console.print_str("Hello World!");

    panic!("start function is not implemented yet");
}
