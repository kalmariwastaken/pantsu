#[cfg(target_arch = "aarch64")]
pub mod nr {
    pub const EXIT: usize = 93;
    pub const MOUNT: usize = 40;
    pub const SETSID: usize = 157;
    pub const WRITE: usize = 64;
    pub const WRITEV: usize = 66;
}

#[cfg(target_arch = "x86_64")]
pub mod nr {
    pub const EXIT: usize = 60;
    pub const MOUNT: usize = 165;
    pub const SETSID: usize = 112;
    pub const WRITE: usize = 1;
    pub const WRITEV: usize = 20;
}

pub use nr::*;
