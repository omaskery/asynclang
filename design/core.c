
#define INTERRUPT(vector) void vector(void)

typedef struct void (*TaskFn)(void *this);

typedef struct _Continuation {
    TaskFn function;
    void *context;
} Continuation;

typedef struct _TaskStateCore {
    Continuation continuesWith;
} TaskStateCore;

static struct {
} system_state;

void Continuation_init(Continuation *this, TaskFn function, void *context) {
    this->function = function;
    this->context = context;
}

void Continuation_invoke(Continuation *this) {
    this->function(this->context);
}

int main() {
    init();

    for(;;) {
        idle();
    }
}

void abort(int _exit) {
    exit(_exit);
}

