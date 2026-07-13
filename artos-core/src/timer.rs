// =====================================================
// Timer abstraction
//
// ARTOS 不直接访问硬件。
//
// Windows:
//     HostClock
//
// STM32:
//     TIM2 / SysTick
//
// 都实现 Clock trait.
//
// =====================================================


use core::future::Future;

use core::pin::Pin;

use core::task::{
    Context,
    Poll,
};



// =====================================================
// 所有平台的 Timer 接口
// =====================================================

pub trait Clock {

    // 返回当前时间(ms)

    fn now(&self) -> u64;

}



// =====================================================
// Sleep Future
//
// 使用:
//
// Sleep::new(clock,1000).await
//
// async 会暂停这里。
// =====================================================


pub struct Sleep<'a,C>
where
    C: Clock,
{

    // Timer 来源

    clock: &'a C,


    // 什么时候结束

    deadline: u64,

}



// 创建 Sleep

impl<'a,C> Sleep<'a,C>
where
    C:Clock,
{

    pub fn new(
        clock:&'a C,
        ms:u64
    )->Self
    {

        Self{

            clock,


            deadline:
            clock.now()+ms,

        }

    }

}



// =====================================================
// 关键:
//
// 让 Sleep 成为 Future
//
// 这样才能:
//
//     Sleep.await
//
// Rust async 要求:
// await 的对象必须实现 Future。
// =====================================================


impl<'a,C> Future for Sleep<'a,C>
where
    C:Clock,
{

    type Output = ();



    fn poll(
        self:Pin<&mut Self>,

        _cx:&mut Context<'_>

    )
        ->Poll<Self::Output>
    {


        // 时间到了

        if self.clock.now()
            >= self.deadline
        {

            Poll::Ready(())

        }


        // 还没有到

        else
        {

            Poll::Pending

        }


    }

}