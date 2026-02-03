package main

/*
#include <stdlib.h>
#include <stdint.h>

// --- Web API Types ---

typedef struct {
    const char* id;
    const char* parent_id;
    const char* name;
    const char* description;
    const char* state;
    const char* result;
    const char* error;
    const char* created_at;
    const char* started_at;
    const char* completed_at;
    const char* failed_at;
    int         position;
    int         task_count;
    double      progress;
} CJobSummary;

typedef struct {
    const char* id;
    const char* name;
    const char* description;
    const char* state;
    const char* cron;
    const char* created_at;
} CScheduledJobSummary;

typedef struct {
    const char* name;
    int         size;
    int         subscribers;
    int         unacked;
} CQueueInfo;

typedef struct {
    const char* status;
    const char* version;
} CHealthCheckResult;

typedef struct {
    const char* id;
    const char* task_id;
    const char* number;
    const char* contents;
} CTaskLogPart;

typedef struct {
    const char* json_items;
    int         number;
    int         size;
    int         total_pages;
    int         total_items;
} CPage;

typedef struct {
    const char* id;
    const char* name;
    const char* description;
    const char* state;
    const char* cron;
    const char* output;
    const char* created_at;
    const char* tasks_json;
    const char* inputs_json;
} CScheduledJob;
*/
import "C"

import (
	"encoding/json"
	"fmt"
	"time"
	"unsafe"

	tork "github.com/runabol/tork"
	"github.com/runabol/tork/broker"
	"github.com/runabol/tork/datastore"
	"github.com/runabol/tork/health"
)

// ==================== JobSummary ====================

//export tork_web_job_summary_from_json
func tork_web_job_summary_from_json(data *C.char) *C.CJobSummary {
	js := &tork.JobSummary{}
	if err := json.Unmarshal([]byte(C.GoString(data)), js); err != nil {
		return nil
	}
	return jobSummaryToC(js)
}

//export tork_web_job_summary_to_json
func tork_web_job_summary_to_json(cs *C.CJobSummary) *C.char {
	js := jobSummaryFromC(cs)
	b, err := json.Marshal(js)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_job_summary_free
func tork_web_job_summary_free(cs *C.CJobSummary) {
	if cs == nil {
		return
	}
	C.free(unsafe.Pointer(cs.id))
	C.free(unsafe.Pointer(cs.parent_id))
	C.free(unsafe.Pointer(cs.name))
	C.free(unsafe.Pointer(cs.description))
	C.free(unsafe.Pointer(cs.state))
	C.free(unsafe.Pointer(cs.result))
	C.free(unsafe.Pointer(cs.error))
	C.free(unsafe.Pointer(cs.created_at))
	C.free(unsafe.Pointer(cs.started_at))
	C.free(unsafe.Pointer(cs.completed_at))
	C.free(unsafe.Pointer(cs.failed_at))
	C.free(unsafe.Pointer(cs))
}

func jobSummaryToC(js *tork.JobSummary) *C.CJobSummary {
	cs := (*C.CJobSummary)(C.malloc(C.size_t(unsafe.Sizeof(C.CJobSummary{}))))
	cs.id = cstr(js.ID)
	cs.parent_id = cstr(js.ParentID)
	cs.name = cstr(js.Name)
	cs.description = cstr(js.Description)
	cs.state = cstr(js.State)
	cs.result = cstr(js.Result)
	cs.error = cstr(js.Error)
	cs.created_at = cstr(js.CreatedAt.Format(time.RFC3339))
	if js.StartedAt != nil {
		cs.started_at = cstr(js.StartedAt.Format(time.RFC3339))
	} else {
		cs.started_at = cstr("")
	}
	if js.CompletedAt != nil {
		cs.completed_at = cstr(js.CompletedAt.Format(time.RFC3339))
	} else {
		cs.completed_at = cstr("")
	}
	if js.FailedAt != nil {
		cs.failed_at = cstr(js.FailedAt.Format(time.RFC3339))
	} else {
		cs.failed_at = cstr("")
	}
	cs.position = C.int(js.Position)
	cs.task_count = C.int(js.TaskCount)
	cs.progress = C.double(js.Progress)
	return cs
}

func jobSummaryFromC(cs *C.CJobSummary) *tork.JobSummary {
	js := &tork.JobSummary{
		ID:          C.GoString(cs.id),
		ParentID:    C.GoString(cs.parent_id),
		Name:        C.GoString(cs.name),
		Description: C.GoString(cs.description),
		State:       C.GoString(cs.state),
		Result:      C.GoString(cs.result),
		Error:       C.GoString(cs.error),
		Position:    int(cs.position),
		TaskCount:   int(cs.task_count),
		Progress:    float64(cs.progress),
	}
	if t, err := time.Parse(time.RFC3339, C.GoString(cs.created_at)); err == nil {
		js.CreatedAt = t
	}
	if s := C.GoString(cs.started_at); s != "" {
		if t, err := time.Parse(time.RFC3339, s); err == nil {
			js.StartedAt = &t
		}
	}
	if s := C.GoString(cs.completed_at); s != "" {
		if t, err := time.Parse(time.RFC3339, s); err == nil {
			js.CompletedAt = &t
		}
	}
	if s := C.GoString(cs.failed_at); s != "" {
		if t, err := time.Parse(time.RFC3339, s); err == nil {
			js.FailedAt = &t
		}
	}
	return js
}

// ==================== ScheduledJob ====================

//export tork_web_scheduled_job_from_json
func tork_web_scheduled_job_from_json(data *C.char) *C.CScheduledJob {
	sj := &tork.ScheduledJob{}
	if err := json.Unmarshal([]byte(C.GoString(data)), sj); err != nil {
		return nil
	}
	return scheduledJobToC(sj)
}

//export tork_web_scheduled_job_to_json
func tork_web_scheduled_job_to_json(cs *C.CScheduledJob) *C.char {
	sj := scheduledJobFromC(cs)
	b, err := json.Marshal(sj)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_scheduled_job_free
func tork_web_scheduled_job_free(cs *C.CScheduledJob) {
	if cs == nil {
		return
	}
	C.free(unsafe.Pointer(cs.id))
	C.free(unsafe.Pointer(cs.name))
	C.free(unsafe.Pointer(cs.description))
	C.free(unsafe.Pointer(cs.state))
	C.free(unsafe.Pointer(cs.cron))
	C.free(unsafe.Pointer(cs.output))
	C.free(unsafe.Pointer(cs.created_at))
	C.free(unsafe.Pointer(cs.tasks_json))
	C.free(unsafe.Pointer(cs.inputs_json))
	C.free(unsafe.Pointer(cs))
}

func scheduledJobToC(sj *tork.ScheduledJob) *C.CScheduledJob {
	cs := (*C.CScheduledJob)(C.malloc(C.size_t(unsafe.Sizeof(C.CScheduledJob{}))))
	cs.id = cstr(sj.ID)
	cs.name = cstr(sj.Name)
	cs.description = cstr(sj.Description)
	cs.state = cstr(string(sj.State))
	cs.cron = cstr(sj.Cron)
	cs.output = cstr(sj.Output)
	cs.created_at = cstr(sj.CreatedAt.Format(time.RFC3339))
	if b, err := json.Marshal(sj.Tasks); err == nil {
		cs.tasks_json = cstr(string(b))
	} else {
		cs.tasks_json = cstr("[]")
	}
	if b, err := json.Marshal(sj.Inputs); err == nil {
		cs.inputs_json = cstr(string(b))
	} else {
		cs.inputs_json = cstr("{}")
	}
	return cs
}

func scheduledJobFromC(cs *C.CScheduledJob) *tork.ScheduledJob {
	sj := &tork.ScheduledJob{
		ID:          C.GoString(cs.id),
		Name:        C.GoString(cs.name),
		Description: C.GoString(cs.description),
		State:       tork.ScheduledJobState(C.GoString(cs.state)),
		Cron:        C.GoString(cs.cron),
		Output:      C.GoString(cs.output),
	}
	if t, err := time.Parse(time.RFC3339, C.GoString(cs.created_at)); err == nil {
		sj.CreatedAt = t
	}
	if tasksJSON := C.GoString(cs.tasks_json); tasksJSON != "" {
		var tasks []*tork.Task
		if err := json.Unmarshal([]byte(tasksJSON), &tasks); err == nil {
			sj.Tasks = tasks
		}
	}
	if inputsJSON := C.GoString(cs.inputs_json); inputsJSON != "" {
		var inputs map[string]string
		if err := json.Unmarshal([]byte(inputsJSON), &inputs); err == nil {
			sj.Inputs = inputs
		}
	}
	return sj
}

// ==================== ScheduledJobSummary ====================

//export tork_web_scheduled_job_summary_from_json
func tork_web_scheduled_job_summary_from_json(data *C.char) *C.CScheduledJobSummary {
	sjs := &tork.ScheduledJobSummary{}
	if err := json.Unmarshal([]byte(C.GoString(data)), sjs); err != nil {
		return nil
	}
	return scheduledJobSummaryToC(sjs)
}

//export tork_web_scheduled_job_summary_to_json
func tork_web_scheduled_job_summary_to_json(cs *C.CScheduledJobSummary) *C.char {
	sjs := scheduledJobSummaryFromC(cs)
	b, err := json.Marshal(sjs)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_scheduled_job_summary_free
func tork_web_scheduled_job_summary_free(cs *C.CScheduledJobSummary) {
	if cs == nil {
		return
	}
	C.free(unsafe.Pointer(cs.id))
	C.free(unsafe.Pointer(cs.name))
	C.free(unsafe.Pointer(cs.description))
	C.free(unsafe.Pointer(cs.state))
	C.free(unsafe.Pointer(cs.cron))
	C.free(unsafe.Pointer(cs.created_at))
	C.free(unsafe.Pointer(cs))
}

func scheduledJobSummaryToC(sjs *tork.ScheduledJobSummary) *C.CScheduledJobSummary {
	cs := (*C.CScheduledJobSummary)(C.malloc(C.size_t(unsafe.Sizeof(C.CScheduledJobSummary{}))))
	cs.id = cstr(sjs.ID)
	cs.name = cstr(sjs.Name)
	cs.description = cstr(sjs.Description)
	cs.state = cstr(string(sjs.State))
	cs.cron = cstr(sjs.Cron)
	cs.created_at = cstr(sjs.CreatedAt.Format(time.RFC3339))
	return cs
}

func scheduledJobSummaryFromC(cs *C.CScheduledJobSummary) *tork.ScheduledJobSummary {
	sjs := &tork.ScheduledJobSummary{
		ID:          C.GoString(cs.id),
		Name:        C.GoString(cs.name),
		Description: C.GoString(cs.description),
		State:       tork.ScheduledJobState(C.GoString(cs.state)),
		Cron:        C.GoString(cs.cron),
	}
	if t, err := time.Parse(time.RFC3339, C.GoString(cs.created_at)); err == nil {
		sjs.CreatedAt = t
	}
	return sjs
}

// ==================== QueueInfo ====================

//export tork_web_queue_info_from_json
func tork_web_queue_info_from_json(data *C.char) *C.CQueueInfo {
	qi := &broker.QueueInfo{}
	if err := json.Unmarshal([]byte(C.GoString(data)), qi); err != nil {
		return nil
	}
	return queueInfoToC(qi)
}

//export tork_web_queue_info_to_json
func tork_web_queue_info_to_json(cq *C.CQueueInfo) *C.char {
	qi := queueInfoFromC(cq)
	b, err := json.Marshal(qi)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_queue_info_free
func tork_web_queue_info_free(cq *C.CQueueInfo) {
	if cq == nil {
		return
	}
	C.free(unsafe.Pointer(cq.name))
	C.free(unsafe.Pointer(cq))
}

func queueInfoToC(qi *broker.QueueInfo) *C.CQueueInfo {
	cq := (*C.CQueueInfo)(C.malloc(C.size_t(unsafe.Sizeof(C.CQueueInfo{}))))
	cq.name = cstr(qi.Name)
	cq.size = C.int(qi.Size)
	cq.subscribers = C.int(qi.Subscribers)
	cq.unacked = C.int(qi.Unacked)
	return cq
}

func queueInfoFromC(cq *C.CQueueInfo) *broker.QueueInfo {
	return &broker.QueueInfo{
		Name:        C.GoString(cq.name),
		Size:        int(cq.size),
		Subscribers: int(cq.subscribers),
		Unacked:     int(cq.unacked),
	}
}

// ==================== HealthCheckResult ====================

//export tork_web_health_from_json
func tork_web_health_from_json(data *C.char) *C.CHealthCheckResult {
	h := &health.HealthCheckResult{}
	if err := json.Unmarshal([]byte(C.GoString(data)), h); err != nil {
		return nil
	}
	ch := (*C.CHealthCheckResult)(C.malloc(C.size_t(unsafe.Sizeof(C.CHealthCheckResult{}))))
	ch.status = cstr(h.Status)
	ch.version = cstr(h.Version)
	return ch
}

//export tork_web_health_to_json
func tork_web_health_to_json(ch *C.CHealthCheckResult) *C.char {
	h := &health.HealthCheckResult{
		Status:  C.GoString(ch.status),
		Version: C.GoString(ch.version),
	}
	b, err := json.Marshal(h)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_health_free
func tork_web_health_free(ch *C.CHealthCheckResult) {
	if ch == nil {
		return
	}
	C.free(unsafe.Pointer(ch.status))
	C.free(unsafe.Pointer(ch.version))
	C.free(unsafe.Pointer(ch))
}

// ==================== TaskLogPart ====================

//export tork_web_task_log_part_from_json
func tork_web_task_log_part_from_json(data *C.char) *C.CTaskLogPart {
	lp := &tork.TaskLogPart{}
	if err := json.Unmarshal([]byte(C.GoString(data)), lp); err != nil {
		return nil
	}
	return taskLogPartToC(lp)
}

//export tork_web_task_log_part_to_json
func tork_web_task_log_part_to_json(cl *C.CTaskLogPart) *C.char {
	lp := taskLogPartFromC(cl)
	b, err := json.Marshal(lp)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_task_log_part_free
func tork_web_task_log_part_free(cl *C.CTaskLogPart) {
	if cl == nil {
		return
	}
	C.free(unsafe.Pointer(cl.id))
	C.free(unsafe.Pointer(cl.task_id))
	C.free(unsafe.Pointer(cl.number))
	C.free(unsafe.Pointer(cl.contents))
	C.free(unsafe.Pointer(cl))
}

func taskLogPartToC(lp *tork.TaskLogPart) *C.CTaskLogPart {
	cl := (*C.CTaskLogPart)(C.malloc(C.size_t(unsafe.Sizeof(C.CTaskLogPart{}))))
	cl.id = cstr(lp.ID)
	cl.task_id = cstr(lp.TaskID)
	cl.number = cstr(fmt.Sprintf("%d", lp.Number))
	cl.contents = cstr(lp.Contents)
	return cl
}

func taskLogPartFromC(cl *C.CTaskLogPart) *tork.TaskLogPart {
	lp := &tork.TaskLogPart{
		ID:       C.GoString(cl.id),
		TaskID:   C.GoString(cl.task_id),
		Contents: C.GoString(cl.contents),
	}
	if n := C.GoString(cl.number); n != "" {
		var num int
		fmt.Sscanf(n, "%d", &num)
		lp.Number = num
	}
	return lp
}

// ==================== Page (generic JSON) ====================

//export tork_web_page_from_json
func tork_web_page_from_json(data *C.char) *C.CPage {
	// Generic page: items as raw JSON
	var raw struct {
		Items      json.RawMessage `json:"items"`
		Number     int             `json:"number"`
		Size       int             `json:"size"`
		TotalPages int             `json:"totalPages"`
		TotalItems int             `json:"totalItems"`
	}
	if err := json.Unmarshal([]byte(C.GoString(data)), &raw); err != nil {
		return nil
	}
	cp := (*C.CPage)(C.malloc(C.size_t(unsafe.Sizeof(C.CPage{}))))
	if raw.Items != nil {
		cp.json_items = cstr(string(raw.Items))
	} else {
		cp.json_items = cstr("[]")
	}
	cp.number = C.int(raw.Number)
	cp.size = C.int(raw.Size)
	cp.total_pages = C.int(raw.TotalPages)
	cp.total_items = C.int(raw.TotalItems)
	return cp
}

//export tork_web_page_to_json
func tork_web_page_to_json(cp *C.CPage) *C.char {
	items := C.GoString(cp.json_items)
	page := struct {
		Items      json.RawMessage `json:"items"`
		Number     int             `json:"number"`
		Size       int             `json:"size"`
		TotalPages int             `json:"totalPages"`
		TotalItems int             `json:"totalItems"`
	}{
		Items:      json.RawMessage(items),
		Number:     int(cp.number),
		Size:       int(cp.size),
		TotalPages: int(cp.total_pages),
		TotalItems: int(cp.total_items),
	}
	b, err := json.Marshal(page)
	if err != nil {
		return nil
	}
	return C.CString(string(b))
}

//export tork_web_page_free
func tork_web_page_free(cp *C.CPage) {
	if cp == nil {
		return
	}
	C.free(unsafe.Pointer(cp.json_items))
	C.free(unsafe.Pointer(cp))
}

// ==================== ScheduledJob State Constants ====================

//export tork_web_scheduled_job_state_active
func tork_web_scheduled_job_state_active() *C.char {
	return C.CString(string(tork.ScheduledJobStateActive))
}

//export tork_web_scheduled_job_state_paused
func tork_web_scheduled_job_state_paused() *C.char {
	return C.CString(string(tork.ScheduledJobStatePaused))
}

// ==================== Health Status Constants ====================

//export tork_web_health_status_up
func tork_web_health_status_up() *C.char { return C.CString(health.StatusUp) }

//export tork_web_health_status_down
func tork_web_health_status_down() *C.char { return C.CString(health.StatusDown) }

// ==================== Queue Name Constants ====================

//export tork_web_queue_pending
func tork_web_queue_pending() *C.char { return C.CString(broker.QUEUE_PENDING) }

//export tork_web_queue_started
func tork_web_queue_started() *C.char { return C.CString(broker.QUEUE_STARTED) }

//export tork_web_queue_completed
func tork_web_queue_completed() *C.char { return C.CString(broker.QUEUE_COMPLETED) }

//export tork_web_queue_error
func tork_web_queue_error() *C.char { return C.CString(broker.QUEUE_ERROR) }

//export tork_web_queue_default
func tork_web_queue_default() *C.char { return C.CString(broker.QUEUE_DEFAULT) }

//export tork_web_queue_heartbeat
func tork_web_queue_heartbeat() *C.char { return C.CString(broker.QUEUE_HEARTBEAT) }

//export tork_web_queue_jobs
func tork_web_queue_jobs() *C.char { return C.CString(broker.QUEUE_JOBS) }

//export tork_web_queue_logs
func tork_web_queue_logs() *C.char { return C.CString(broker.QUEUE_LOGS) }

// ==================== Convenience: Full response parsers ====================

// Parse a list-of-jobs response (Page<JobSummary>) into JSON items array
//
//export tork_web_parse_jobs_page
func tork_web_parse_jobs_page(data *C.char) *C.CPage {
	var page datastore.Page[*tork.JobSummary]
	if err := json.Unmarshal([]byte(C.GoString(data)), &page); err != nil {
		return nil
	}
	itemsJSON, err := json.Marshal(page.Items)
	if err != nil {
		return nil
	}
	cp := (*C.CPage)(C.malloc(C.size_t(unsafe.Sizeof(C.CPage{}))))
	cp.json_items = cstr(string(itemsJSON))
	cp.number = C.int(page.Number)
	cp.size = C.int(page.Size)
	cp.total_pages = C.int(page.TotalPages)
	cp.total_items = C.int(page.TotalItems)
	return cp
}

// Parse a list-of-scheduled-jobs response (Page<ScheduledJobSummary>)
//
//export tork_web_parse_scheduled_jobs_page
func tork_web_parse_scheduled_jobs_page(data *C.char) *C.CPage {
	var page datastore.Page[*tork.ScheduledJobSummary]
	if err := json.Unmarshal([]byte(C.GoString(data)), &page); err != nil {
		return nil
	}
	itemsJSON, err := json.Marshal(page.Items)
	if err != nil {
		return nil
	}
	cp := (*C.CPage)(C.malloc(C.size_t(unsafe.Sizeof(C.CPage{}))))
	cp.json_items = cstr(string(itemsJSON))
	cp.number = C.int(page.Number)
	cp.size = C.int(page.Size)
	cp.total_pages = C.int(page.TotalPages)
	cp.total_items = C.int(page.TotalItems)
	return cp
}

// Parse a task-log response (Page<TaskLogPart>)
//
//export tork_web_parse_log_page
func tork_web_parse_log_page(data *C.char) *C.CPage {
	var page datastore.Page[*tork.TaskLogPart]
	if err := json.Unmarshal([]byte(C.GoString(data)), &page); err != nil {
		return nil
	}
	itemsJSON, err := json.Marshal(page.Items)
	if err != nil {
		return nil
	}
	cp := (*C.CPage)(C.malloc(C.size_t(unsafe.Sizeof(C.CPage{}))))
	cp.json_items = cstr(string(itemsJSON))
	cp.number = C.int(page.Number)
	cp.size = C.int(page.Size)
	cp.total_pages = C.int(page.TotalPages)
	cp.total_items = C.int(page.TotalItems)
	return cp
}

// ==================== Queue helpers ====================

//export tork_web_is_coordinator_queue
func tork_web_is_coordinator_queue(name *C.char) C.int {
	if broker.IsCoordinatorQueue(C.GoString(name)) {
		return 1
	}
	return 0
}

//export tork_web_is_worker_queue
func tork_web_is_worker_queue(name *C.char) C.int {
	if broker.IsWorkerQueue(C.GoString(name)) {
		return 1
	}
	return 0
}

//export tork_web_is_task_queue
func tork_web_is_task_queue(name *C.char) C.int {
	if broker.IsTaskQueue(C.GoString(name)) {
		return 1
	}
	return 0
}


