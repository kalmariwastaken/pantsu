use crate::{nr, Result};

pub fn writev(fd: u64, bufs: &[&[u8]]) -> Result<usize> {
    let result: usize;

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::asm!(
            "svc 0",
            in("x8") nr::WRITEV,
            inlateout("x0") fd => result,
            in("x1") bufs.as_ptr(),
            in("x2") bufs.len(),
            options(nostack),
        );
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::asm!(
            "syscall",
            inlateout("rax") nr::WRITEV => result,
            in("rdi") fd as usize,
            in("rsi") bufs.as_ptr(),
            in("rdx") bufs.len(),
            lateout("r11") _,
            lateout("rcx") _,
            options(nostack),
        );
    }

    crate::to_result(result)
}
