use std::thread;
use std::sync::mpsc::channel;
//use crossbeam_channel::unbounded as channel;

mod message {
    use std::fmt;

    const LEN: usize = 1;

    #[derive(Clone, Copy)]
    pub struct Message(pub [usize; LEN]);

    impl fmt::Debug for Message {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.pad("Message")
        }
    }

    #[inline]
    pub fn new(num: usize) -> Message {
        Message([num; LEN])
    }
}

const MESSAGES: usize = 5_000_000;

fn run_sync() {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let handle = thread::spawn(move || {
        unsafe {
            //let param = libc::sched_param {sched_priority: 1};
            let param = libc::sched_param {sched_priority: 2};
            assert_eq!(libc::sched_setscheduler(0, libc::SCHED_FIFO, &param), 0);
        }
        for i in 0..MESSAGES {
            tx1.send(message::new(i)).unwrap();
            rx2.recv().unwrap();
        }
    });
    unsafe {
        let param = libc::sched_param {sched_priority: 1};
        assert_eq!(libc::sched_setscheduler(0, libc::SCHED_FIFO, &param), 0);
    }
    for i in 0..MESSAGES {
        rx1.recv().unwrap();
        tx2.send(message::new(i)).unwrap();
    }
    handle.join().unwrap();
}

fn main() {
    let now = ::std::time::Instant::now();
    run_sync();
    let elapsed = now.elapsed();
    println!(
        "{:7.3} sec",
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9
    );
}
