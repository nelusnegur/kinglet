.section ".text.entry"

.global _entry

_entry:
  mrs x5, mpidr_el1
  and x5, x5, #0xff
  cbz x5, primary_cpu_entry
  b secondary_cpu_entry

primary_cpu_entry:
  // Clear BSS section
  ldr x0, =__bss_start
  ldr x1, =__bss_end
  sub x1, x1, x0
  bl .memzero

  // Set up stack pointer
  ldr x5, =__stack
  mov sp, x5

  bl start
  // The start function should never return!
  // If it does return, then it's a bug.
  b .bug

secondary_cpu_entry:
  // TODO: Implement the setup of the secondary CPU cores
  b .park_cpu

.park_cpu:
  wfe
  b .park_cpu

.bug:
  b .park_cpu

// Parameters:
//  x0 - start address
//  x1 - size in bytes
//
// The provided region of memory must be 8-byte aligned.
.memzero:
	str xzr, [x0], #8
	subs x1, x1, #8
	b.gt .memzero
	ret
