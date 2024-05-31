use std::{
        future::Future, pin::Pin, sync::{mpsc::{sync_channel, Receiver, Sender, SyncSender}, Arc, Mutex}, task::{Context, Poll, Waker}, thread, time::Duration
};

use futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake}
};

// futures will not do anything unless driven to completion
// we need a futures executor
// executor takes top level futures and call poll on them when they can make progress
// futures indicate they can make progress by calling wake


struct SharedState {
        completed: bool,
        waker: Option<Waker>

}
pub struct TimerFuture {
        shared_state: Arc<Mutex<SharedState>>
}

impl Future for TimerFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut shared_state = self.shared_state.lock().unwrap();

                if shared_state.completed {
                        Poll::Ready(())
                } else {
                        shared_state.waker = Some(cx.waker().clone());
                        Poll::Pending
                }
        }
}

impl TimerFuture {
        pub fn new(duration: Duration) -> Self {
                let shared_state = Arc::new(Mutex::new(SharedState {completed: false, waker: None}));

                let thread_shared_state = shared_state.clone();

                thread::spawn(move || {
                        thread::sleep(duration);
                        let mut shared_state = thread_shared_state.lock().unwrap();
                        shared_state.completed = true;

                        if let Some(waker) = shared_state.waker.take() {
                                waker.wake();
                        }
                });

                TimerFuture { shared_state }
        }
}

// receives tasks of a channel and runs them
struct Executor {
        ready_queue: Receiver<Arc<Task>>
}

// spawn new futures onto the task channel
struct Spawner {
        task_sender: SyncSender<Arc<Task>>
}

impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
                let future = future.boxed();
                let task = Arc::new(Task {
                        future: Mutex::new(Some(future)),
                        task_sender: self.task_sender.clone(),
                });
                self.task_sender.send(task).expect("too many tasks");
        }
}

// a future that can reschedule itself
struct Task {
        future: Mutex<Option<BoxFuture<'static, ()>>>,
        task_sender: SyncSender<Arc<Task>>
}

fn new_executor_and_spanwer() -> (Executor, Spawner) {
        const MAX_QUEUED_TASKS: usize =  10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
}






fn main() {

}







