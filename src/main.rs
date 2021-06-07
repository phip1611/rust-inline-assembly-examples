// new modern asm macro
#![feature(asm)]
// old legacy asm macro
#![feature(llvm_asm)]

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "arm")]
mod arm;

fn main() {
    #[cfg(target_arch = "x86_64")]
    x86_64::run_demos();
    #[cfg(target_arch = "arm")]
    arm::run_demos();
}
