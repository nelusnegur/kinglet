OUTPUT_ARCH(aarch64)
ENTRY(_entry)

KIMAGE_ADDRESS = 0x80000;

SECTIONS
{
    . = KIMAGE_ADDRESS;

    .text.entry : {
      *(.text.entry)
    }

    .text : {
      *(.text);
      *(.text.*)
    }

    .rodata : {
      *(.rodata);
      *(.rodata.*)
    }

    .data : {
      *(.data);
      *(.data.*)
    }

    .bss : {
      . = ALIGN(8);
      __bss_start = .;

      *(.bss);
      *(.bss.*)
    }

    . = ALIGN(8);
    __bss_end = .;
}
