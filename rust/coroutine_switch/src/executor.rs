extern crate alloc;

use lazy_static::*;

use {
    alloc::{boxed::Box, sync::Arc},
    core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    },
    spin::Mutex,
    woke::{waker_ref, Woke},
};

const NUM: usize = 384;

/// Executor holds a list of tasks to be processed
pub struct Executor {
    tasks: [Option<Arc<Task>>; NUM],
    current: usize,
}

/// Task is our unit of execution and holds a future are waiting on
pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    state: Mutex<bool>,
}

/// Implement what we would like to do when a task gets woken up
impl Woke for Task {
    fn wake_by_ref(task: &Arc<Self>) {
        task.mark_ready();
    }
}

impl Task {
    fn mark_ready(&self) {
        let mut value = self.state.lock();
        *value = true;
    }

    pub fn is_sleeping(&self) -> bool {
        let value = self.state.lock();
        !(*value)
    }

    pub fn mark_sleep(&self) {
        let mut value = self.state.lock();
        *value = false;
    }
}

impl Executor {
    /// Add task for a future to the list of tasks
    fn add_task(&mut self, future: Pin<Box<dyn Future<Output = ()> + 'static + Send>>) {
        let mut tasks = &mut self.tasks;
        let mut task = tasks
            .iter_mut()
            .find(|t| t.is_none())
            .expect("no available task.");
        *task = Some(Arc::new(Task {
            future: Mutex::new(future),
            state: Mutex::new(true),
        }));
    }

    /// Give future to be polled and executed
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static + Send) {
        self.add_task(Box::pin(future));
    }

    /// Run futures until there is no runnable task.
    ///
    /// If all tasks (including sleeping tasks) are finished, returns `false`,
    /// otherwise returns `true`.
    pub fn run_until_idle(&mut self) {
        while self.run_task() {}
    }

    fn run_task(&mut self) -> bool {
        let current = self.current;
        let start = (current + 1) % NUM;
        let mut cur = start;
        let tasks = &self.tasks;
        while tasks[cur].is_none() || tasks[cur].as_ref().unwrap().is_sleeping() {
            cur = (cur + 1) % NUM;
            if cur == start {
                return false;
            }
        }
        let task = &tasks[cur].as_ref().unwrap().clone();
        drop(tasks);
        self.current = cur;
        // println!("run task {}", cur);
        // task.mark_sleep();
        // make a waker for our task
        let waker = waker_ref(task);
        // poll our future and give it a waker
        let mut context = Context::from_waker(&*waker);
        let ret = task.future.lock().as_mut().poll(&mut context);
        drop(task);
        if let Poll::Ready(_) = ret {
            self.tasks[self.current] = None;
        }
        true
    }
}

const NONE_TASK: Option<Arc<Task>> = None;

lazy_static! {
    static ref GLOBAL_EXECUTOR: Mutex<Executor> = Mutex::new(Executor {
        tasks: [NONE_TASK; NUM],
        current: NUM - 1,
    });
}

/// Give future to global executor to be polled and executed.
pub fn spawn(future: impl Future<Output = ()> + 'static + Send) {
    GLOBAL_EXECUTOR.lock().spawn(future);
}

/// Run futures in global executor until there is no runnable task.
///
/// If all tasks (including sleeping tasks) are finished, returns `false`,
/// otherwise returns `true`.
pub fn run_until_idle() {
    GLOBAL_EXECUTOR.lock().run_until_idle()
}
