use crate::{nr, Result};
use core::ptr;

pub fn mount(
    source: Option<&[u8]>,
    target: Option<&[u8]>,
    kind: Option<&[u8]>,
    flags: usize,
    extra: Option<&[u8]>,
) -> Result<usize> {
    fn as_ptr(opt: Option<&[u8]>) -> *const u8 {
        opt.map(|s| s.as_ptr()).unwrap_or(ptr::null())
    }

    let result: usize;

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::asm!(
            "svc 0",
            in("x8") nr::MOUNT,
            inlateout("x0") as_ptr(source) => result,
            in("x1") as_ptr(target),
            in("x2") as_ptr(kind),
            in("x3") flags,
            in("x4") as_ptr(extra),
            options(nostack),
        );
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::asm!(
            "syscall",
            inlateout("rax") nr::MOUNT => result,
            in("rdi") as_ptr(source),
            in("rsi") as_ptr(target),
            in("rdx") as_ptr(kind),
            in("r10") flags,
            in("r8") as_ptr(extra),
            lateout("r11") _,
            lateout("rcx") _,
            options(nostack),
        );
    }

    crate::to_result(result)
}
