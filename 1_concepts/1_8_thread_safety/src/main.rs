use crate::sync_send::{OnlySend, OnlySync, SyncAndSend};
use std::sync::atomic::Ordering::SeqCst;
use std::{thread, time};

mod sync_send {
    use std::cell::Cell;
    use std::marker::PhantomPinned;
    use std::rc::Rc;
    use std::sync::atomic::AtomicU32;

    pub struct OnlySync {
        pub global_read_only: i32,
        _pin: PhantomPinned,
    }

    impl OnlySync {
        /// ## Safety
        /// Do not move this type before drop
        pub unsafe fn new(val: i32) -> OnlySync {
            OnlySync {
                global_read_only: val,
                _pin: PhantomPinned,
            }
        }
    }

    #[derive(Default)]
    pub struct OnlySend {
        cell: Cell<i32>,
    }

    #[derive(Default)]
    pub struct SyncAndSend {
        pub counter: AtomicU32,
    }

    #[derive(Default)]
    pub struct NotSyncNotSend {
        ptr: Rc<i32>,
    }
}

fn main() {
    let only_send = OnlySend::default();
    let sync_and_send = SyncAndSend::default();
    let do_not_move = unsafe { OnlySync::new(3) };

    println!("{}", sync_and_send.counter.load(SeqCst));

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            let reciever = only_send;
            let counter = &sync_and_send;
            println!("Peek inside -> {}", &do_not_move.global_read_only);

            counter.counter.fetch_add(1, SeqCst);
        });
    })
    .unwrap();

    // Can`t do that
    // let only_send = only_send;

    thread::sleep(time::Duration::from_millis(10));
    println!("{}", sync_and_send.counter.load(SeqCst));
}
