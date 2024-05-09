// 移除标准库(std)依赖
#![no_std]
// 移除main标准main函数
#![no_main]
// 为了在 lang_items.rs 中通过 PanicInfo::message 获取报错信息
#![feature(panic_info_message)]
// 在导入模块或 crate 时同时导入该模块或 crate 中定义的宏
#[macro_use]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
// global_asm!宏：嵌入全局汇编代码;include_str!宏：文件的内容作为一个字符串嵌入到程序中;
global_asm!(include_str!("entry.asm"));


// 不要对该函数名进行修饰，以便其他语言就可以通过该原始名称来链接和调用 Rust 函数或访问 Rust 变量。
#[no_mangle]
pub fn main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}


// 对 .bss 段清零
fn clear_bss() {
    extern "C" {
        // sbss 和 ebss 是链接器脚本中定义的符号，它们代表 .bss 段的边界地址。
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
