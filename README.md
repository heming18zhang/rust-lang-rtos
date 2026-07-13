# ARTOS

## Async Rust Language Real Time Operating System

ARTOS 是一个使用 Rust `async/await` 构建的轻量级嵌入式实时系统（RTOS）学习项目。

目标：

* 使用 Rust 实现一个简单 async runtime
* 学习 Future、Executor、Waker 工作原理
* 使用 `#![no_std]` 设计 MCU 友好的系统
* 在 Windows/Linux 无硬件环境测试
* 设计目标为 Cortex-M MCU（例如 STM32F3）

# Features

## v0.1

| Feature            | Status |
| ------------------ | ------ |
| Rust async/await   | ✅      |
| Future based task  | ✅      |
| no_std kernel      | ✅      |
| Timer abstraction  | ✅      |
| Host simulator     | ✅      |
| Windows/Linux test | ✅      |

# Architecture

```text
Application

    |
    |
    v

async task

    |
    |
    v

ARTOS Executor

    |
    |
    v

Future::poll()

    |
    |
    v

Hardware Abstraction

    |
    +----------------+
    |                |
    v                v

Windows Host      STM32 MCU

Instant           TIM/SysTick
```

ARTOS kernel 不直接访问硬件。

通过 trait 抽象：

```rust
trait Clock {
    fn now(&self) -> u64;
}
```

Windows:

```rust
struct HostClock;
```

STM32:

```rust
struct STM32Clock;
```

应用层代码无需改变。

# Project Structure

```text
rust-artos

├── Cargo.toml
│
├── artos-core
│   │
│   ├── lib.rs
│   ├── timer.rs
│   └── executor.rs
│
└── host
    │
    └── main.rs
```

# Why Async RTOS?

传统 RTOS：

```text
Task

 |
Thread Stack

 |
Context Switch

 |
Scheduler
```

每个任务需要：

* 独立 stack
* CPU context switch

Async RTOS：

```text
async task

      |

Future State Machine

      |

poll()

      |

Ready / Pending
```

优势：

* 更少 RAM 使用
* 不需要线程 stack
* 编译器生成状态机
* Rust 类型安全

适合：

* Cortex-M MCU
* IoT
* Sensor
* Embedded Control

# Example

Async LED Task：

```rust
async fn led_task(
    clock: &HostClock
)
{
    loop
    {
        println!("LED ON");

        Sleep::new(
            clock,
            1000
        )
        .await;


        println!("LED OFF");

        Sleep::new(
            clock,
            1000
        )
        .await;
    }
}
```

执行流程：

```text
LED ON

 |

Sleep Future

 |

Pending

 |

Timer expired

 |

LED OFF
```

# Async Internals

Rust：

```rust
async fn task()
{

}
```

编译器生成：

```text
async function

       |

       v

Future State Machine

       |

       v

poll()

       |

 +-----+------+

 |            |

Pending    Ready
```

Executor 不创建线程。

它只负责：

```text
poll Future
```

直到任务完成。

# no_std Design

ARTOS Core：

```rust
#![no_std]
```

不使用：

```text
std

 |
 +-- thread
 |
 +-- filesystem
 |
 +-- OS service
```

使用：

```text
core

 |
 +-- Future
 |
 +-- Pin
 |
 +-- Trait
 |
 +-- Result
```

因此目标平台：

```text
Windows Host

      |

      v

QEMU Cortex-M

      |

      v

STM32F3 MCU
```

# Run

安装 Rust：

https://rustup.rs/

检查：

```bash
rustc --version
cargo --version
```

运行 Host：

```bash
cd rust-artos

cargo run -p host
```

输出：

```text
ARTOS v0.1 start

LED ON
LED OFF
LED ON
LED OFF
```

# Useful Commands

检查 kernel：

```bash
cargo check -p artos-core
```

编译：

```bash
cargo build
```

清理：

```bash
cargo clean
```

# Learning Goals

## Rust Embedded

* no_std
* ownership
* lifetime
* trait abstraction
* async/await
* Future
* Pin
* Waker

## RTOS Design

* Executor
* Task scheduling
* Timer service
* Interrupt driven runtime

## MCU Development

* Cortex-M
* QEMU
* STM32 HAL
* Embedded Rust

# License

MIT License
