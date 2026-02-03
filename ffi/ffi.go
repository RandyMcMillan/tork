package main

/*
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    const char* id;
    const char* name;
    const char* description;
    const char* state;
    const char* image;
    const char* run;
    const char* queue;
    const char* error;
    const char* result;
    const char* var_name;
    const char* if_expr;
    const char* timeout;
    const char* gpus;
    const char* workdir;
    int         position;
    int         priority;
    double      progress;
} CTask;

typedef struct {
    const char* id;
    const char* parent_id;
    const char* name;
    const char* description;
    const char* state;
    const char* output;
    const char* result;
    const char* error;
    int         position;
    int         task_count;
} CJob;

typedef struct {
    const char* id;
    const char* name;
    const char* hostname;
    const char* queue;
    const char* status;
    const char* version;
    double      cpu_percent;
    int         port;
    int         task_count;
} CNode;

typedef struct {
    const char* id;
    const char* name;
    const char* username;
    int         disabled;
} CUser;

typedef struct {
    const char* id;
    const char* slug;
    const char* name;
} CRole;

typedef struct {
    const char* type;
    const char* source;
    const char* target;
} CMount;

typedef struct {
    int jobs_running;
    int tasks_running;
    int nodes_running;
    double nodes_cpu_percent;
} CMetrics;
*/
import "C"

import (
	"encoding/json"
	"unsafe"

	tork "github.com/runabol/tork"
)

// --- Version ---

//export tork_version
func tork_version() *C.char {
	return C.CString(tork.Version)
}

//export tork_git_commit
func tork_git_commit() *C.char {
	return C.CString(tork.GitCommit)
}

// --- Free ---

//export tork_free_string
func tork_free_string(s *C.char) {
	C.free(unsafe.Pointer(s))
}

// --- Job ---

//export tork_job_from_json
func tork_job_from_json(data *C.char) *C.CJob {
	j := &tork.Job{}
	if err := json.Unmarshal([]byte(C.GoString(data)), j); err != nil {
		return nil
	}
	return jobToC(j)
}

//export tork_job_to_json
func tork_job_to_json(cj *C.CJob) *C.char {
	j := jobFromC(cj)
	b, err := json.Marshal(j)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_job_free
func tork_job_free(cj *C.CJob) {
	if cj == nil {
		return
	}
	C.free(unsafe.Pointer(cj.id))
	C.free(unsafe.Pointer(cj.parent_id))
	C.free(unsafe.Pointer(cj.name))
	C.free(unsafe.Pointer(cj.description))
	C.free(unsafe.Pointer(cj.state))
	C.free(unsafe.Pointer(cj.output))
	C.free(unsafe.Pointer(cj.result))
	C.free(unsafe.Pointer(cj.error))
	C.free(unsafe.Pointer(cj))
}

// --- Task ---

//export tork_task_from_json
func tork_task_from_json(data *C.char) *C.CTask {
	t := &tork.Task{}
	if err := json.Unmarshal([]byte(C.GoString(data)), t); err != nil {
		return nil
	}
	return taskToC(t)
}

//export tork_task_to_json
func tork_task_to_json(ct *C.CTask) *C.char {
	t := taskFromC(ct)
	b, err := json.Marshal(t)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_task_is_active
func tork_task_is_active(state *C.char) C.int {
	s := C.GoString(state)
	switch s {
	case tork.TaskStateCreated, tork.TaskStatePending,
		tork.TaskStateScheduled, tork.TaskStateRunning:
		return 1
	}
	return 0
}

//export tork_task_free
func tork_task_free(ct *C.CTask) {
	if ct == nil {
		return
	}
	C.free(unsafe.Pointer(ct.id))
	C.free(unsafe.Pointer(ct.name))
	C.free(unsafe.Pointer(ct.description))
	C.free(unsafe.Pointer(ct.state))
	C.free(unsafe.Pointer(ct.image))
	C.free(unsafe.Pointer(ct.run))
	C.free(unsafe.Pointer(ct.queue))
	C.free(unsafe.Pointer(ct.error))
	C.free(unsafe.Pointer(ct.result))
	C.free(unsafe.Pointer(ct.var_name))
	C.free(unsafe.Pointer(ct.if_expr))
	C.free(unsafe.Pointer(ct.timeout))
	C.free(unsafe.Pointer(ct.gpus))
	C.free(unsafe.Pointer(ct.workdir))
	C.free(unsafe.Pointer(ct))
}

// --- Node ---

//export tork_node_from_json
func tork_node_from_json(data *C.char) *C.CNode {
	n := &tork.Node{}
	if err := json.Unmarshal([]byte(C.GoString(data)), n); err != nil {
		return nil
	}
	return nodeToC(n)
}

//export tork_node_to_json
func tork_node_to_json(cn *C.CNode) *C.char {
	n := nodeFromC(cn)
	b, err := json.Marshal(n)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_node_free
func tork_node_free(cn *C.CNode) {
	if cn == nil {
		return
	}
	C.free(unsafe.Pointer(cn.id))
	C.free(unsafe.Pointer(cn.name))
	C.free(unsafe.Pointer(cn.hostname))
	C.free(unsafe.Pointer(cn.queue))
	C.free(unsafe.Pointer(cn.status))
	C.free(unsafe.Pointer(cn.version))
	C.free(unsafe.Pointer(cn))
}

// --- Metrics ---

//export tork_metrics_from_json
func tork_metrics_from_json(data *C.char) *C.CMetrics {
	m := &tork.Metrics{}
	if err := json.Unmarshal([]byte(C.GoString(data)), m); err != nil {
		return nil
	}
	cm := (*C.CMetrics)(C.malloc(C.size_t(unsafe.Sizeof(C.CMetrics{}))))
	cm.jobs_running = C.int(m.Jobs.Running)
	cm.tasks_running = C.int(m.Tasks.Running)
	cm.nodes_running = C.int(m.Nodes.Running)
	cm.nodes_cpu_percent = C.double(m.Nodes.CPUPercent)
	return cm
}

//export tork_metrics_free
func tork_metrics_free(cm *C.CMetrics) {
	if cm != nil {
		C.free(unsafe.Pointer(cm))
	}
}

// --- Helpers ---

func cstr(s string) *C.char {
	if s == "" {
		return C.CString("")
	}
	return C.CString(s)
}

func jobToC(j *tork.Job) *C.CJob {
	cj := (*C.CJob)(C.malloc(C.size_t(unsafe.Sizeof(C.CJob{}))))
	cj.id = cstr(j.ID)
	cj.parent_id = cstr(j.ParentID)
	cj.name = cstr(j.Name)
	cj.description = cstr(j.Description)
	cj.state = cstr(j.State)
	cj.output = cstr(j.Output)
	cj.result = cstr(j.Result)
	cj.error = cstr(j.Error)
	cj.position = C.int(j.Position)
	cj.task_count = C.int(j.TaskCount)
	return cj
}

func jobFromC(cj *C.CJob) *tork.Job {
	return &tork.Job{
		ID:          C.GoString(cj.id),
		ParentID:    C.GoString(cj.parent_id),
		Name:        C.GoString(cj.name),
		Description: C.GoString(cj.description),
		State:       C.GoString(cj.state),
		Output:      C.GoString(cj.output),
		Result:      C.GoString(cj.result),
		Error:       C.GoString(cj.error),
		Position:    int(cj.position),
		TaskCount:   int(cj.task_count),
	}
}

func taskToC(t *tork.Task) *C.CTask {
	ct := (*C.CTask)(C.malloc(C.size_t(unsafe.Sizeof(C.CTask{}))))
	ct.id = cstr(t.ID)
	ct.name = cstr(t.Name)
	ct.description = cstr(t.Description)
	ct.state = cstr(t.State)
	ct.image = cstr(t.Image)
	ct.run = cstr(t.Run)
	ct.queue = cstr(t.Queue)
	ct.error = cstr(t.Error)
	ct.result = cstr(t.Result)
	ct.var_name = cstr(t.Var)
	ct.if_expr = cstr(t.If)
	ct.timeout = cstr(t.Timeout)
	ct.gpus = cstr(t.GPUs)
	ct.workdir = cstr(t.Workdir)
	ct.position = C.int(t.Position)
	ct.priority = C.int(t.Priority)
	ct.progress = C.double(t.Progress)
	return ct
}

func taskFromC(ct *C.CTask) *tork.Task {
	return &tork.Task{
		ID:          C.GoString(ct.id),
		Name:        C.GoString(ct.name),
		Description: C.GoString(ct.description),
		State:       C.GoString(ct.state),
		Image:       C.GoString(ct.image),
		Run:         C.GoString(ct.run),
		Queue:       C.GoString(ct.queue),
		Error:       C.GoString(ct.error),
		Result:      C.GoString(ct.result),
		Var:         C.GoString(ct.var_name),
		If:          C.GoString(ct.if_expr),
		Timeout:     C.GoString(ct.timeout),
		GPUs:        C.GoString(ct.gpus),
		Workdir:     C.GoString(ct.workdir),
		Position:    int(ct.position),
		Priority:    int(ct.priority),
		Progress:    float64(ct.progress),
	}
}

func nodeToC(n *tork.Node) *C.CNode {
	cn := (*C.CNode)(C.malloc(C.size_t(unsafe.Sizeof(C.CNode{}))))
	cn.id = cstr(n.ID)
	cn.name = cstr(n.Name)
	cn.hostname = cstr(n.Hostname)
	cn.queue = cstr(n.Queue)
	cn.status = cstr(string(n.Status))
	cn.version = cstr(n.Version)
	cn.cpu_percent = C.double(n.CPUPercent)
	cn.port = C.int(n.Port)
	cn.task_count = C.int(n.TaskCount)
	return cn
}

func nodeFromC(cn *C.CNode) *tork.Node {
	return &tork.Node{
		ID:         C.GoString(cn.id),
		Name:       C.GoString(cn.name),
		Hostname:   C.GoString(cn.hostname),
		Queue:      C.GoString(cn.queue),
		Status:     tork.NodeStatus(C.GoString(cn.status)),
		Version:    C.GoString(cn.version),
		CPUPercent: float64(cn.cpu_percent),
		Port:       int(cn.port),
		TaskCount:  int(cn.task_count),
	}
}

// --- Job State Constants ---

//export tork_job_state_pending
func tork_job_state_pending() *C.char { return C.CString(tork.JobStatePending) }

//export tork_job_state_scheduled
func tork_job_state_scheduled() *C.char { return C.CString(tork.JobStateScheduled) }

//export tork_job_state_running
func tork_job_state_running() *C.char { return C.CString(tork.JobStateRunning) }

//export tork_job_state_cancelled
func tork_job_state_cancelled() *C.char { return C.CString(tork.JobStateCancelled) }

//export tork_job_state_completed
func tork_job_state_completed() *C.char { return C.CString(tork.JobStateCompleted) }

//export tork_job_state_failed
func tork_job_state_failed() *C.char { return C.CString(tork.JobStateFailed) }

//export tork_job_state_restart
func tork_job_state_restart() *C.char { return C.CString(tork.JobStateRestart) }

// --- Task State Constants ---

//export tork_task_state_created
func tork_task_state_created() *C.char { return C.CString(tork.TaskStateCreated) }

//export tork_task_state_pending
func tork_task_state_pending() *C.char { return C.CString(tork.TaskStatePending) }

//export tork_task_state_scheduled
func tork_task_state_scheduled() *C.char { return C.CString(tork.TaskStateScheduled) }

//export tork_task_state_running
func tork_task_state_running() *C.char { return C.CString(tork.TaskStateRunning) }

//export tork_task_state_cancelled
func tork_task_state_cancelled() *C.char { return C.CString(tork.TaskStateCancelled) }

//export tork_task_state_stopped
func tork_task_state_stopped() *C.char { return C.CString(tork.TaskStateStopped) }

//export tork_task_state_completed
func tork_task_state_completed() *C.char { return C.CString(tork.TaskStateCompleted) }

//export tork_task_state_failed
func tork_task_state_failed() *C.char { return C.CString(tork.TaskStateFailed) }

//export tork_task_state_skipped
func tork_task_state_skipped() *C.char { return C.CString(tork.TaskStateSkipped) }

func main() {}
