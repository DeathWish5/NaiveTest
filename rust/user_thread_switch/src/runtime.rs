use super::{DEFAULT_STACK_SIZE, MAX_TASKS};
use spin::{Mutex, MutexGuard};
use std::collections::VecDeque;
pub struct Runtime {
    tasks: Vec<Task>,
    unused: VecDeque<usize>,
    ready: VecDeque<usize>,
    current: usize,
    ctx: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Available,
    Running,
    Ready,
}

pub struct Task {
    id: usize,
    stack: Vec<u8>,
    ctx: usize,
    state: State,
}

#[derive(Debug, Default)]
#[repr(C)] // not strictly needed but Rust ABI is not guaranteed to be stable
struct TaskContext {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub rbp: usize,
    pub rbx: usize,
    // pc
    pub rip: usize,
}

impl Task {
    pub fn new(id: usize) -> Self {
        Task {
            id,
            stack: vec![0u8; DEFAULT_STACK_SIZE],
            ctx: 0,
            state: State::Available,
        }
    }

    pub fn get_context(&self) -> usize {
        (&self.ctx) as *const usize as _
    }

    pub fn set_context(&mut self, ctx: usize) {
        self.ctx = ctx;
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new());
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            tasks: (0..MAX_TASKS).map(|i| Task::new(i)).collect(),
            unused: (0..MAX_TASKS).collect(),
            ready: VecDeque::with_capacity(MAX_TASKS),
            current: usize::MAX,
            ctx: 0,
        }
    }

    pub fn get_current_task(&mut self) -> Option<&mut Task> {
        if self.current < MAX_TASKS {
            Some(&mut self.tasks[self.current])
        } else {
            None
        }
    }

    pub fn get_context(&self) -> usize {
        (&self.ctx) as *const usize as _
    }

    pub fn spawn(&mut self, f: fn(arg: usize), arg: usize) {
        let loc = self.unused.pop_front().expect("no available task.");
        let proc = &mut self.tasks[loc];
        if proc.state != State::Available {
            panic!("proc isn't available");
        }
        let stack_bottom = proc.stack.as_mut_ptr() as usize;
        let stack_top = stack_bottom + DEFAULT_STACK_SIZE;
        let mut sp: usize = stack_top;
        sp = unsafe { push_stack(sp, f as usize) };
        sp = unsafe { push_stack(sp, arg) };
        let context_data = TaskContext {
            rip: entry as usize,
            ..TaskContext::default()
        };
        sp = unsafe { push_stack(sp, context_data) };
        proc.ctx = sp;
        // println!("task[{}].sp = 0x{:x}, entry = 0x{:x}", 0, sp, f as usize);
        proc.state = State::Ready;
        self.ready.push_back(proc.id);
    }
}

pub unsafe fn push_stack<T>(stack_top: usize, val: T) -> usize {
    let stack_top = (stack_top as *mut T).sub(1);
    *stack_top = val;
    stack_top as _
}

pub fn get_current_runtime() -> MutexGuard<'static, Runtime> {
    GLOBAL_RUNTIME.lock()
}

pub fn spawn(f: fn(arg: usize), arg: usize) {
    let mut runtime = get_current_runtime();
    runtime.spawn(f, arg);
}

/// switch to runtime, which would select an appropriate executor to run.
pub fn sched_yield() {
    let mut runtime = get_current_runtime();
    let runtime_cx = runtime.get_context();
    if let Some(task) = runtime.get_current_task() {
        let task_cx = task.get_context();
        drop(task);
        drop(runtime);
        // println!("      try to yield to runtime");
        switch(task_cx as _, runtime_cx as _);
        // println!("      yield return");
    }
}

pub fn exit() {
    let mut runtime = get_current_runtime();
    let mut task = runtime.get_current_task().unwrap();
    task.state = State::Available;
    let id = task.id;
    drop(task);
    runtime.unused.push_front(id);
    drop(runtime);
    sched_yield();
    unreachable!();
}

#[naked]
unsafe fn entry() {
    std::arch::asm!(
        "
    pop rdi
    ret
    ",
        options(noreturn, raw)
    )
}

#[naked]
unsafe fn switch_inner(old: usize, new: usize) {
    std::arch::asm!(
        "
    // push rip by caller

    // Save callee-save registers
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    
    mov [rdi], rsp      // rdi = from_rsp
    mov rsp, [rsi]      // rsi = to_rspget_current_runtime
    
    // Pop callee-save registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    // pop rip
    ret
    ",
        options(noreturn, raw)
    )
}

/// This is where we start running our runtime. If it is our base task, we call yield until
/// it returns false (which means that there are no tasks scheduled) and we are done.
pub fn run_until_idle() {
    COUNT.store(0, Ordering::Relaxed);
    while run_task() {}
    // println!("context switch number: {}", COUNT.load(Ordering::SeqCst));
}

fn run_task() -> bool {
    let mut runtime = get_current_runtime();
    if let Some(task) = runtime.get_current_task() {
        let id = task.id;
        if task.state == State::Running {
            task.state = State::Ready;
            runtime.ready.push_back(id);
        }
    }
    // let start = (current + 1) % MAX_TASKS;
    // let mut pos = start;
    // while runtime.tasks[pos].state != State::Ready {
    //     pos = (pos + 1) % MAX_TASKS;
    //     if pos == start {
    //         return false;
    //     }
    // }

    let pos = runtime.ready.pop_front();
    if pos.is_none() {
        return false;
    }
    let current = pos.unwrap();
    runtime.current = current;
    let runtime_cx = runtime.get_context();
    let task = &mut runtime.tasks[current];
    if task.state != State::Ready && task.state != State::Running {
        println!("task[{}].state = {:?}", task.id, task.state);
        panic!("ready not ready");
    } else {
        task.state = State::Running;
    }

    let task_cx = task.get_context();
    drop(runtime);
    switch(runtime_cx, task_cx);
    true
}

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn switch(old: usize, new: usize) {
    COUNT.fetch_add(1, Ordering::Relaxed);
    unsafe { switch_inner(old, new) };
}
