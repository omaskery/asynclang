
global timerx_continuation: Continuation;

fn init() {}

fn idle() {}

fn delay(continuation: Continuation, period_ms: u32) {
    timerx_continuation = continuation;
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

