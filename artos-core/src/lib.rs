// =================================================
// ARTOS Kernel
//
// 运行目标:
//     STM32F3
//     Cortex-M
//
// 特点:
//     no_std
//     async runtime
//
// 这里不能使用:
//     println!
//     Vec
//     String
//     std::time
//
// 因为 MCU 没有操作系统。
// =================================================


#![no_std]


pub mod timer;

pub mod executor;