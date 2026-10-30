ENTRY(Reset)
MEMORY {
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 4M
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 4M
}
SECTIONS {
  .vector_table ORIGIN(FLASH) : {
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    KEEP(*(.vector_table));
  } > FLASH
  .text : {
    *(.text .text.*);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH
  .ARM.exidx : {
    __exidx_start = .;
    *(.ARM.exidx*);
    __exidx_end = .;
  } > FLASH
  .data : {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    __edata = .;
  } > RAM AT> FLASH
  __sidata = LOADADDR(.data);
  .bss (NOLOAD) : {
    . = ALIGN(8);
    __sbss = .;
    *(.bss .bss.* COMMON);
    . = ALIGN(8);
    __ebss = .;
  } > RAM
  .heap (NOLOAD) : {
    . = ALIGN(16);
    __heap_start = .;
    . += 2M;
    __heap_end = .;
  } > RAM
  /DISCARD/ : { *(.comment); }
}
ASSERT(__heap_end < ORIGIN(RAM) + LENGTH(RAM) - 64K, "reserve firmware stack")
