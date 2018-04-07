
#define INTERRUPT(vector) void vector(void)

typedef TaskCondition (*TaskFunction)(void *this);

typedef enum _TaskCondition {
    eTaskCondition_Starting,
    eTaskCondition_Suspend,
    eTaskCondition_Running,
    eTaskCondition_Complete,
} TaskCondition;

typedef struct _Task {
    TaskCondition condition;
    Task *next_task;
    TaskFunction fn;
    uint32_t state;
} Task;

static struct {
    Task *ready_task_list;
    Task *suspend_task_list;
    Task *current_task;
} system_state;

void main() {
    init();

    for(;;) {
        if(system_state.current_task != NULL) {
            task_step_common(system_state.current_task);
        } else {
            idle();
        }
    }
}

void task_resume(Task *this) {
    if(this->condition != eTaskCondition_Suspend) {
        abort(-1);
    }
    this->condition = eTaskCondition_Running;
    
}

void init_task_common(Task *this, TaskFunction fn) {
    this->condition = eTaskCondition_Starting;
    this->state = 0;
    this->fn = fn;
}

TaskCondition step_task_common(Task *this) {
    this->condition = this->fn(this);
    return this->condition;
}

TaskCondition advance_task_common(Task *this) {
    this->state ++;
}

TaskCondition task_await(Task *this) {
    TaskCondition nested_result = step_task_common(this);
    if(nested_result == eTaskCondition_Complete) {
        advance_task_common(this);
        return eTaskCondition_Running;
    } else {
        return nested_result;
    }
}

void abort(int _exit) {
    exit(_exit);
}

