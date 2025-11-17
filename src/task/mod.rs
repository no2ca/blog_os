pub mod simple_executor;

use alloc::boxed::Box;
use core::{future::Future, pin::Pin, task::{Context, Poll}};

pub struct Task {
    // dynはトレイトオブジェクトをBoxに格納することを示す
    // 今回使用しているTaskは戻り値が無い
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        // この "+" は複数の条件を持っているときに使う
        // 今回はトレイトとライフタイムの両方
        Task {
            future: Box::pin(future),
        }
    }
    
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}