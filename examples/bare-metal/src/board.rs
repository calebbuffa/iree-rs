//! Minimal QEMU MPS2-AN386 BSP. A real board supplies its own equivalent.
use core::{
    alloc::{GlobalAlloc, Layout},
    arch::{asm, global_asm},
    ffi::c_void,
    fmt::Write,
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

global_asm!(
    ".section .vector_table,\"a\"",
    ".word Reset",
    ".rept 13",
    ".word Fault",
    ".endr",
    ".word SysTick",
);

unsafe extern "C" {
    static mut __sdata: u32;
    static mut __edata: u32;
    static __sidata: u32;
    static mut __sbss: u32;
    static mut __ebss: u32;
    static mut __heap_start: u8;
    static mut __heap_end: u8;
    fn memalign(alignment: usize, size: usize) -> *mut c_void;
    fn free(pointer: *mut c_void);
    fn __errno() -> *mut i32;
}

struct NativeHeap;
unsafe impl GlobalAlloc for NativeHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { memalign(layout.align().max(8), layout.size().max(1)).cast() }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) {
        unsafe { free(pointer.cast()) };
    }
}
#[global_allocator]
static ALLOCATOR: NativeHeap = NativeHeap;

// Allocations and IREE are main-context only; no interrupt enters newlib.
#[unsafe(no_mangle)]
unsafe extern "C" fn _sbrk(increment: isize) -> *mut c_void {
    static mut BRK: *mut u8 = ptr::null_mut();
    unsafe {
        let base = ptr::addr_of_mut!(__heap_start);
        let end = ptr::addr_of_mut!(__heap_end);
        let current = if BRK.is_null() { base } else { BRK };
        let next = current.wrapping_offset(increment);
        if next < base || next > end {
            *__errno() = 12; // newlib ENOMEM
            return usize::MAX as *mut c_void;
        }
        BRK = next;
        current.cast()
    }
}

static MILLIS_LOW: AtomicU32 = AtomicU32::new(0);
static MILLIS_HIGH: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
extern "C" fn SysTick() {
    if MILLIS_LOW.fetch_add(1, Ordering::Relaxed) == u32::MAX {
        MILLIS_HIGH.fetch_add(1, Ordering::Relaxed);
    }
}

#[unsafe(no_mangle)]
extern "C" fn iree_bare_metal_time_now_ns() -> i64 {
    let mask: u32;
    unsafe { asm!("mrs {mask}, PRIMASK", "cpsid i", mask = out(reg) mask, options(nostack)) };
    let low = MILLIS_LOW.load(Ordering::Relaxed);
    let high = MILLIS_HIGH.load(Ordering::Relaxed);
    unsafe { asm!("msr PRIMASK, {mask}", mask = in(reg) mask, options(nostack)) };
    (((high as u64) << 32) | low as u64)
        .saturating_mul(1_000_000)
        .min(i64::MAX as u64) as i64
}
#[unsafe(no_mangle)]
extern "C" fn iree_bare_metal_wait_until_ns(deadline_ns: i64) -> bool {
    while iree_bare_metal_time_now_ns() < deadline_ns {
        unsafe { asm!("wfi", options(nostack)) };
    }
    true
}

#[unsafe(no_mangle)]
unsafe extern "C" fn Reset() -> ! {
    unsafe {
        let data_len = (ptr::addr_of!(__edata) as usize - ptr::addr_of!(__sdata) as usize) / 4;
        ptr::copy_nonoverlapping(
            ptr::addr_of!(__sidata),
            ptr::addr_of_mut!(__sdata),
            data_len,
        );
        let bss_len = (ptr::addr_of!(__ebss) as usize - ptr::addr_of!(__sbss) as usize) / 4;
        ptr::write_bytes(ptr::addr_of_mut!(__sbss), 0, bss_len);
        // Cortex-M4F CPACR and MPS2's 25MHz SysTick (1ms).
        ptr::write_volatile(0xE000_ED88 as *mut u32, 0x00F0_0000);
        asm!("dsb", "isb", options(nostack));
        ptr::write_volatile(0xE000_E014 as *mut u32, 25_000 - 1);
        ptr::write_volatile(0xE000_E018 as *mut u32, 0);
        ptr::write_volatile(0xE000_E010 as *mut u32, 7);
    }
    match crate::run() {
        Ok(()) => {
            let _ = writeln!(
                Console,
                "IREE bare-metal: VM i32/f32 passed; repeated call passed; teardown passed"
            );
            exit(0)
        }
        Err(error) => {
            let _ = writeln!(Console, "IREE bare-metal: {error}");
            exit(error.code().as_i32() as u32)
        }
    }
}

struct Console;
impl Write for Console {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        unsafe { iree_native_diagnostic(text.as_ptr(), text.len()) };
        Ok(())
    }
}

/// Board-provided sink for destructor diagnostics from the owned crate.
#[unsafe(no_mangle)]
unsafe extern "C" fn iree_native_diagnostic(data: *const u8, length: usize) {
    let text = unsafe { core::slice::from_raw_parts(data, length) };
    for chunk in text.chunks(127) {
        let mut buffer = [0u8; 128];
        buffer[..chunk.len()].copy_from_slice(chunk);
        unsafe {
            asm!("bkpt 0xab", inout("r0") 4u32 => _, in("r1") buffer.as_ptr(), options(nostack));
        }
    }
}

fn exit(code: u32) -> ! {
    let reason = [0x20026u32, code];
    unsafe {
        asm!("bkpt 0xab", inout("r0") 0x20u32 => _, in("r1") reason.as_ptr(), options(nostack));
    }
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
extern "C" fn Fault() -> ! {
    let _ = writeln!(Console, "IREE bare-metal: processor fault");
    exit(127)
}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    let _ = writeln!(Console, "IREE bare-metal: {info}");
    exit(126)
}
