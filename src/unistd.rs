use crate::{nr, Result};

pub fn setsid() -> Result<u32> {
    let result: usize;

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::asm!(
            "svc 0",
            in("x8") nr::SETSID,
            lateout("x0") result,
            options(nostack),
        );
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::asm!(
            "syscall",
            inlateout("rax") nr::SETSID => result,
            lateout("r11") _,
            lateout("rcx") _,
            options(nostack),
        );
    }

    crate::to_result(result).map(|pid| pid as u32)
}

pub fn exit(code: u32) -> ! {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::asm!(
            "svc 0",
            in("x8") nr::EXIT,
            in("x0") code,
            options(noreturn, nostack),
        );
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::asm!(
            "syscall",
            in("ax") nr::EXIT,
            in("di") code,
            options(noreturn, nostack),
        );
    }
}

pub fn write(fd: u64, buf: &[u8]) -> Result<usize> {
    let result: usize;

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::asm!(
            "svc 0",
            in("x8") nr::WRITE,
            inlateout("x0") fd => result,
            in("x1") buf.as_ptr(),
            in("x2") buf.len(),
            options(nostack),
        );
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::asm!(
            "syscall",
            inlateout("rax") nr::WRITE => result,
            in("rdi") fd as usize,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            lateout("r11") _,
            lateout("rcx") _,
            options(nostack),
        );
    }

    crate::to_result(result)
}

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
