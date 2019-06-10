use tokio::prelude::*;
use std::thread;
use std::time;

struct FutureTest {
    poll_count: i32,
}

impl Future for FutureTest {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("poll called!");
        self.poll_count += 1;

        let task = task::current();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(1000));
            task.notify();
        });

        if self.poll_count <= 1 {
            println!("not ready");
            Ok(Async::NotReady)
        }
        else {
            println!("ready!");
            Ok(Async::Ready(()))
        }
    }
}

fn main() {
    let f = FutureTest{poll_count:0};

    tokio::run(f);
}
