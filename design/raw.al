
global timerx_continuation: Continuation;

fn init() {
}

fn idle() {
}

async delay(period_ms: u32) {
    timerx_continuation = task_current().continuesWith;
    init_timerX(period_ms);
}

async periodic(period_ms: u32) {
    loop {
        await delay(period_ms);
        await println("Hi!");
    }
}

interrupt timerx_overflow {
    await timerx_continuation;
}

