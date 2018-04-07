
global task_to_wake: &Task;

fn init() {
}

fn idle() {
}

task delay(task: &Task, period_ms: u32) {
    init_timerX(period_ms);
    task_to_wake = task;
    suspend;
}

task periodic(task: &Task, period_ms: u32) {
    loop {
        await delay(period_ms);
        await println("Hi!");
    }
}

interrupt timerx_overflow {
    task_resume(task_to_wake);
}

