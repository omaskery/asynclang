
typedef struct _TaskState_delay {
    TaskStateCore core;
    struct {
        uint32_t period_ms;
    } locals;
} TaskState_delay;

typedef struct _TaskState_periodic {
    TaskStateCore core;
    union {
        TaskState_delay delay;
        TaskState_println println;
    } nested_tasks;
    struct {
        uint32_t period_ms;
    } locals;
} TaskState_periodic;

static struct {
    Continuation timerx_continuation;
} globals;

void init() {}

void idle() {}

void task_delay0(TaskState_delay *this) {
    globals.timerx_continuation = this->core.continuesWith;
    init_timerX(period_ms);
}

void task_periodic0(TaskState_periodic *this) {
    Continuation_init(
        &this->nested_tasks.delay.core.continuesWith,
        task_periodic1,
        this
    );
    this->nested_tasks.delay.locals.period_ms = this->locals.period_ms;
    task_delay0(&this->nested_tasks.delay);
}

void task_periodic1(TaskState_periodic *this) {
    Continuation_init(
        &this->nested_tasks.println.core.continuesWith,
        task_periodic0,
        this
    );
    this->nested_tasks.println.locals.text = "Hi!";
    task_println0(&this->nested_tasks.println);
}

INTERRUPT(timerx_overflow) {
    Continuation_invoke(&globals.timerx_continuation);
}

