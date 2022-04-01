use lazy_static::*;

extern crate alloc;

use {
    alloc::{boxed::Box, sync::Arc},
    core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    },
    spin::Mutex,
    std::collections::VecDeque,
    woke::{waker_ref, Woke},
};

use super::MAX_TASKS;

/// Executor holds a list of tasks to be processed
pub struct Executor {
    tasks: Vec<Option<Arc<Task>>>,
    unused: VecDeque<usize>,
    ready: Arc<Mutex<VecDeque<usize>>>,
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
    pub fn new() -> Self {
        Self {
            tasks: (0..MAX_TASKS).map(|_| NONE_TASK).collect(),
            unused: (0..MAX_TASKS).collect(),
            ready: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_TASKS))),
            current: usize::MAX,
        }
    }
    /// Add task for a future to the list of tasks
    fn add_task(&mut self, future: Pin<Box<dyn Future<Output = ()> + 'static + Send>>) {
        let loc = self.unused.pop_front().expect("no available task.");
        let task = &mut self.tasks[loc];
        if task.is_some() {
            panic!("task isn't available");
        }
        *task = Some(Arc::new(Task {
            future: Mutex::new(future),
            state: Mutex::new(true),
        }));
        self.ready.lock().push_back(loc);
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
        let next = self.ready.lock().pop_front();
        if next.is_none() {
            return false;
        }
        let cur = next.unwrap();
        let task = &self.tasks[cur].as_ref().unwrap().clone();
        self.current = cur;
        let waker = waker_ref(task);
        // poll our future and give it a waker
        let mut context = Context::from_waker(&*waker);
        let ret = task.future.lock().as_mut().poll(&mut context);
        drop(task);
        if let Poll::Ready(_) = ret {
            self.tasks[self.current] = None;
            self.unused.push_front(self.current);
        } else {
            let task = &self.tasks[cur].as_ref().unwrap();
            if *task.state.lock() == true {
                drop(task);
                self.ready.lock().push_back(self.current);
            } else {
                panic!("QAQ, not supported");
            }
        }
        true
    }
}

const NONE_TASK: Option<Arc<Task>> = None;

lazy_static! {
    static ref GLOBAL_EXECUTOR: Mutex<Executor> = Mutex::new(Executor::new());
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
