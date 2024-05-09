use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::Sleep;

mod slow_reader;

#[pin_project]
pub struct Timer {
    #[pin]
    sleep: Sleep,
}

impl Timer {
    pub fn new() -> Self {
        let sleep = tokio::time::sleep(Duration::from_secs(5));

        Self { sleep }
    }
}

impl Future for Timer {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("My future polled ");

        let this = self.project();

        match this.sleep.poll(cx) {
            Poll::Ready(_) => Poll::Ready("Yes we got it sir"),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod future_tests {
    use super::Timer;
    use anyhow::Result;

    #[tokio::test(flavor = "current_thread")]
    async fn future_test() -> Result<()> {
        let result = Timer::new().await;

        println!("{}", result);

        crate::Ok()
    }
}
// // You can poll the sleep like this
// if let Poll::Ready(_) = self.sleep.poll_unpin(cx) {
//     return Poll::Ready("Yes we got it");
// }

// // or
// if let Poll::Ready(_) = Pin::new(&mut self.sleep).poll(cx) {
//     return Poll::Ready("Yes we got it");
// }

// // or
// if let Poll::Ready(_) = self.sleep.as_mut().poll(cx) {
//     return Poll::Ready("Yes we got it");
// }

// // or
// if let Poll::Ready(_) = Sleep::poll(self.sleep.as_mut(), cx) {
//     return Poll::Ready("Yes we got it");
// }

// let pin = unsafe {
//     let this = self.get_unchecked_mut();

//     Pin::new_unchecked(&mut this.sleep)
// };

// if let Poll::Ready(_) = pin.poll(cx) {
//     return Poll::Ready("Yes we got it");
// }
