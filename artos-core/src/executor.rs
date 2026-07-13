// =====================================================
// ARTOS Executor
//
// v0.1:
//     只运行一个 async task
//
// 后续:
//     多 task scheduler
//
// =====================================================


use core::future::Future;

use core::pin::Pin;

use core::task::Context;



pub struct Executor;



impl Executor {


    pub const fn new()->Self
    {

        Executor

    }



    pub fn run<F>(
        &self,
        mut future:F

    )
    where

        F:Future<Output=()>

    {


        // Future 地址固定

        let mut future =
            unsafe
                {
                    Pin::new_unchecked(
                        &mut future
                    )
                };



        // 临时 waker

        let waker =
            futures::task::noop_waker();



        let mut cx =
            Context::from_waker(
                &waker
            );



        loop
        {


            // 调用 Future.poll()

            match future
                .as_mut()
                .poll(&mut cx)
            {

                core::task::Poll::Ready(_)=>
                    {

                        break;

                    }


                core::task::Poll::Pending=>
                    {

                        // 继续循环

                    }

            }


        }


    }

}