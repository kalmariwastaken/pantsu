#![feature(asm)]
#![feature(crate_visibility_modifier)]
#![no_std]

pub type Error = usize;
pub type Result<T> = core::result::Result<T, Error>;

crate fn to_result(result: usize) -> Result<usize> {
    if (result as isize).is_positive() {
        Ok(result as isize as usize)
    } else {
        Err((-(result as isize)) as usize)
    }
}

pub mod mount;
pub mod nr;
pub mod sys;
pub mod unistd;
