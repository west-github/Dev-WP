use futures::Future;
use pin_project::pin_project;
use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, ReadBuf},
    time::{sleep, Instant, Sleep},
};

#[pin_project]
struct SlowRead<R> {
    #[pin]
    sleep: Sleep,

    #[pin]
    reader: R,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self {
            sleep: sleep(Default::default()),
            reader,
        }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        // let (mut sleep, reader) = unsafe {
        //     let this = self.get_unchecked_mut();

        //     (
        //         Pin::new_unchecked(&mut this.sleep),
        //         Pin::new_unchecked(&mut this.reader),
        //     )
        // };

        // match sleep.as_mut().poll(cx) {
        //     Poll::Ready(_) => {
        //         sleep.reset(Instant::now() + Duration::from_secs(1));

        //         reader.poll_read(cx, buf)
        //     }
        //     Poll::Pending => Poll::Pending,
        // }

        let mut this = self.project();

        match this.sleep.as_mut().poll(cx) {
            Poll::Ready(_) => {
                this.sleep.reset(Instant::now() + Duration::from_secs(1));

                this.reader.poll_read(cx, buf)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod slow_reader_tests {

    use std::pin::Pin;

    use super::SlowRead;
    use anyhow::Result;
    use tokio::{fs::File, io::AsyncReadExt, time::Instant};

    #[tokio::test(flavor = "current_thread")]
    async fn slow_reader_test() -> Result<()> {
        let mut buf = vec![0u8; 128 * 1028];

        let mut f = SlowRead::new(File::open("./index.txt").await?);

        let before = Instant::now();

        let mut f = unsafe { Pin::new_unchecked(&mut f) };

        f.read_exact(&mut buf).await?;

        println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

        Ok(())
    }

    #[tokio::test(flavor = "current_thread")]
    async fn reader_without_slow_reader_tests() -> Result<()> {
        let mut buf = vec![0u8; 128 * 1028];
        let mut f = File::open("./index.txt").await?;
        let before = Instant::now();
        f.read_exact(&mut buf).await?;
        println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

        Ok(())
    }
}
