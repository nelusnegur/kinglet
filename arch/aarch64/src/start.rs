use core::arch::global_asm;

global_asm!(include_str!("./entry.s"));

#[no_mangle]
pub(crate) fn start() -> ! {
    panic!("start function is not implemented yet");
}
