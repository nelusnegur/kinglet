use core::arch::asm;

#[inline(always)]
pub(crate) fn wait(cpu_cycles: u32) {
    unsafe {
        asm!(
            "1:",
            "subs {0:x}, {0:x}, #1",
            "bne 1b",
            inout(reg) cpu_cycles => _,
            options(nomem, nostack),
        )
    }
}
