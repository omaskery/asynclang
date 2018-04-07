
// replace everything with continuation passing?

static struct {
    Task *task_to_wake;
} globals;

typedef struct _TaskState_delay {
    Task common;
    struct {
        uint32_t period_ms;
    } locals;
} TaskState_delay;

typedef struct _TaskState_periodic {
    Task common;
    union {
        TaskState_delay delay;
        TaskState_println println;
    } nested_task;
    struct {
        uint32_t period_ms;
    } locals;
} TaskState_periodic;

void init() {
}

void idle() {
}

void init_task__delay(TaskState_delay *this, uint32_t period_ms) {
    init_task_common(&this->common, init_task__delay);
    this->locals.period_ms = period_ms;
}

TaskCondition task_delay(TaskState_delay *this) {
    switch(this->common.state) {
        case 0: {
            init_timerX(this->locals.period_ms);
            globals.task_to_wake = this;
            advance_task_common(&this->common);
            return eTaskCondition_Suspend;
        } break;
        case 1: {
            return eTaskCondition_Complete;
        } break;
    }
}

void init_task__periodic(TaskState_periodic *this, uint32_t period_ms) {
    init_task_common(&this->common, init_task__periodic);
    this->locals.period_ms = period_ms;
}

TaskCondition task_periodic(TaskState_periodic *this) {
    switch(this->common.state) {
        case 0: {
            init_task__delay(&this->nested_task.delay, this->locals.period);
            advance_task_common(&this->common);
        } // intentional fallthrough
        case 1: {
            return task_await((Task*) this, task_delay);
        } break;
        case 2: {
            init_task__println(&this->nested_task.println, "Hi!");
            advance_task_common(&this->common);
        } // intentional fallthrough
        case 3: {
            return task_await((Task*) this, task_println);
        } break;
        case 4: {
            return eTaskCondition_Complete;
        } break;
    }
}

INTERRUPT(timerx_overflow) {
    task_resume(globals.task_to_wake);
}

