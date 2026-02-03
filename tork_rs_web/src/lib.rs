//! # tork-rs-web
//!
//! Rust FFI bindings for the [Tork](https://tork.run) workflow engine **web API** types.
//!
//! This crate exposes the types returned by Tork's REST endpoints:
//!
//! - `JobSummary`  — compact job representation from `GET /jobs`
//! - `ScheduledJob` / `ScheduledJobSummary` — cron-scheduled job types
//! - `QueueInfo` — broker queue stats from `GET /queues`
//! - `HealthCheck` — health probe from `GET /health`
//! - `TaskLogPart` — log entries from `GET /tasks/:id/log`
//! - `Page<T>` — paginated list wrapper
//!
//! Re-exports core types (`Job`, `Task`, `Node`, `Metrics`) from `tork_rs`.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use serde::{Deserialize, Serialize};

// Re-export core types so consumers only need one crate
pub use tork_rs::{self, Job, Metrics, Node, Task};

unsafe fn c_to_string(p: *const c_char) -> String {
    if p.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
    }
}

// ==================== JobSummary ====================

/// Compact job information returned by `GET /jobs` and `POST /jobs`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSummary {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub result: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub created_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub started_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub completed_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub failed_at: String,
    #[serde(default)]
    pub position: i32,
    #[serde(default)]
    pub task_count: i32,
    #[serde(default)]
    pub progress: f64,
}

impl JobSummary {
    /// Parse a JSON string into a `JobSummary` via the Go FFI layer.
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_job_summary_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let js = JobSummary {
                id: c_to_string((*ptr).id),
                parent_id: c_to_string((*ptr).parent_id),
                name: c_to_string((*ptr).name),
                description: c_to_string((*ptr).description),
                state: c_to_string((*ptr).state),
                result: c_to_string((*ptr).result),
                error: c_to_string((*ptr).error),
                created_at: c_to_string((*ptr).created_at),
                started_at: c_to_string((*ptr).started_at),
                completed_at: c_to_string((*ptr).completed_at),
                failed_at: c_to_string((*ptr).failed_at),
                position: (*ptr).position,
                task_count: (*ptr).task_count,
                progress: (*ptr).progress,
            };
            ffi::tork_web_job_summary_free(ptr);
            Some(js)
        }
    }

    /// Serialize this `JobSummary` to JSON via the Go FFI layer.
    pub fn to_json(&self) -> Option<String> {
        let json_in = serde_json::to_string(self).ok()?;
        let cs = CString::new(json_in).ok()?;
        unsafe {
            let ptr = ffi::tork_web_job_summary_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let out = ffi::tork_web_job_summary_to_json(ptr);
            let result = if out.is_null() {
                None
            } else {
                let s = c_to_string(out);
                ffi::tork_free_string(out);
                Some(s)
            };
            ffi::tork_web_job_summary_free(ptr);
            result
        }
    }
}

// ==================== ScheduledJob ====================

/// Full scheduled job returned by `GET /scheduled-jobs/:id`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledJob {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cron: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub output: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub created_at: String,
    /// Tasks serialized as a JSON array string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tasks_json: String,
    /// Inputs serialized as a JSON object string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub inputs_json: String,
}

impl ScheduledJob {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_scheduled_job_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let sj = ScheduledJob {
                id: c_to_string((*ptr).id),
                name: c_to_string((*ptr).name),
                description: c_to_string((*ptr).description),
                state: c_to_string((*ptr).state),
                cron: c_to_string((*ptr).cron),
                output: c_to_string((*ptr).output),
                created_at: c_to_string((*ptr).created_at),
                tasks_json: c_to_string((*ptr).tasks_json),
                inputs_json: c_to_string((*ptr).inputs_json),
            };
            ffi::tork_web_scheduled_job_free(ptr);
            Some(sj)
        }
    }

    /// Parse the embedded tasks_json into a `Vec<Task>`.
    pub fn tasks(&self) -> Vec<Task> {
        if self.tasks_json.is_empty() {
            return Vec::new();
        }
        serde_json::from_str(&self.tasks_json).unwrap_or_default()
    }

    /// Parse the embedded inputs_json into a map.
    pub fn inputs(&self) -> std::collections::HashMap<String, String> {
        if self.inputs_json.is_empty() {
            return std::collections::HashMap::new();
        }
        serde_json::from_str(&self.inputs_json).unwrap_or_default()
    }
}

// ==================== ScheduledJobSummary ====================

/// Compact scheduled job info returned by `GET /scheduled-jobs`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledJobSummary {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cron: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub created_at: String,
}

impl ScheduledJobSummary {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_scheduled_job_summary_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let sjs = ScheduledJobSummary {
                id: c_to_string((*ptr).id),
                name: c_to_string((*ptr).name),
                description: c_to_string((*ptr).description),
                state: c_to_string((*ptr).state),
                cron: c_to_string((*ptr).cron),
                created_at: c_to_string((*ptr).created_at),
            };
            ffi::tork_web_scheduled_job_summary_free(ptr);
            Some(sjs)
        }
    }
}

// ==================== QueueInfo ====================

/// Broker queue statistics returned by `GET /queues`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueueInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: i32,
    #[serde(default)]
    pub subscribers: i32,
    #[serde(default)]
    pub unacked: i32,
}

impl QueueInfo {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_queue_info_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let qi = QueueInfo {
                name: c_to_string((*ptr).name),
                size: (*ptr).size,
                subscribers: (*ptr).subscribers,
                unacked: (*ptr).unacked,
            };
            ffi::tork_web_queue_info_free(ptr);
            Some(qi)
        }
    }

    pub fn to_json(&self) -> Option<String> {
        let json_in = serde_json::to_string(self).ok()?;
        let cs = CString::new(json_in).ok()?;
        unsafe {
            let ptr = ffi::tork_web_queue_info_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let out = ffi::tork_web_queue_info_to_json(ptr);
            let result = if out.is_null() {
                None
            } else {
                let s = c_to_string(out);
                ffi::tork_free_string(out);
                Some(s)
            };
            ffi::tork_web_queue_info_free(ptr);
            result
        }
    }
}

// ==================== HealthCheck ====================

/// Health probe response from `GET /health`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: String,
    pub version: String,
}

impl HealthCheck {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_health_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let hc = HealthCheck {
                status: c_to_string((*ptr).status),
                version: c_to_string((*ptr).version),
            };
            ffi::tork_web_health_free(ptr);
            Some(hc)
        }
    }

    pub fn is_up(&self) -> bool {
        self.status == "UP"
    }
}

// ==================== TaskLogPart ====================

/// A single log entry returned by `GET /tasks/:id/log`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskLogPart {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub task_id: String,
    #[serde(default)]
    pub number: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub contents: String,
}

impl TaskLogPart {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_task_log_part_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let lp = TaskLogPart {
                id: c_to_string((*ptr).id),
                task_id: c_to_string((*ptr).task_id),
                number: c_to_string((*ptr).number).parse().unwrap_or(0),
                contents: c_to_string((*ptr).contents),
            };
            ffi::tork_web_task_log_part_free(ptr);
            Some(lp)
        }
    }
}

// ==================== Page ====================

/// Generic paginated response wrapper. Items are held as a raw JSON string
/// that can be deserialized into the appropriate type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub number: i32,
    pub size: i32,
    pub total_pages: i32,
    pub total_items: i32,
}

/// Raw page with items as unparsed JSON — useful when you need pagination
/// metadata before deciding how to deserialize items.
#[derive(Debug, Clone, Default)]
pub struct RawPage {
    pub items_json: String,
    pub number: i32,
    pub size: i32,
    pub total_pages: i32,
    pub total_items: i32,
}

impl RawPage {
    /// Parse any paginated JSON response into a `RawPage`.
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ptr = ffi::tork_web_page_from_json(cs.as_ptr() as *mut c_char);
            if ptr.is_null() {
                return None;
            }
            let page = RawPage {
                items_json: c_to_string((*ptr).json_items),
                number: (*ptr).number,
                size: (*ptr).size,
                total_pages: (*ptr).total_pages,
                total_items: (*ptr).total_items,
            };
            ffi::tork_web_page_free(ptr);
            Some(page)
        }
    }

    /// Deserialize items into a typed `Page<T>`.
    pub fn into_typed<T: serde::de::DeserializeOwned>(self) -> Option<Page<T>> {
        let items: Vec<T> = serde_json::from_str(&self.items_json).ok()?;
        Some(Page {
            items,
            number: self.number,
            size: self.size,
            total_pages: self.total_pages,
            total_items: self.total_items,
        })
    }
}

/// Parse a `GET /jobs` response directly into `Page<JobSummary>`.
pub fn parse_jobs_page(json: &str) -> Option<Page<JobSummary>> {
    let cs = CString::new(json).ok()?;
    unsafe {
        let ptr = ffi::tork_web_parse_jobs_page(cs.as_ptr() as *mut c_char);
        if ptr.is_null() {
            return None;
        }
        let items_json = c_to_string((*ptr).json_items);
        let page = RawPage {
            items_json,
            number: (*ptr).number,
            size: (*ptr).size,
            total_pages: (*ptr).total_pages,
            total_items: (*ptr).total_items,
        };
        ffi::tork_web_page_free(ptr);
        page.into_typed()
    }
}

/// Parse a `GET /scheduled-jobs` response directly into `Page<ScheduledJobSummary>`.
pub fn parse_scheduled_jobs_page(json: &str) -> Option<Page<ScheduledJobSummary>> {
    let cs = CString::new(json).ok()?;
    unsafe {
        let ptr = ffi::tork_web_parse_scheduled_jobs_page(cs.as_ptr() as *mut c_char);
        if ptr.is_null() {
            return None;
        }
        let items_json = c_to_string((*ptr).json_items);
        let page = RawPage {
            items_json,
            number: (*ptr).number,
            size: (*ptr).size,
            total_pages: (*ptr).total_pages,
            total_items: (*ptr).total_items,
        };
        ffi::tork_web_page_free(ptr);
        page.into_typed()
    }
}

/// Parse a task/job log response into `Page<TaskLogPart>`.
pub fn parse_log_page(json: &str) -> Option<Page<TaskLogPart>> {
    let cs = CString::new(json).ok()?;
    unsafe {
        let ptr = ffi::tork_web_parse_log_page(cs.as_ptr() as *mut c_char);
        if ptr.is_null() {
            return None;
        }
        let items_json = c_to_string((*ptr).json_items);
        let page = RawPage {
            items_json,
            number: (*ptr).number,
            size: (*ptr).size,
            total_pages: (*ptr).total_pages,
            total_items: (*ptr).total_items,
        };
        ffi::tork_web_page_free(ptr);
        page.into_typed()
    }
}

// ==================== Queue classification helpers ====================

/// Returns `true` if the queue is a coordinator-internal queue.
pub fn is_coordinator_queue(name: &str) -> bool {
    let cs = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return false,
    };
    unsafe { ffi::tork_web_is_coordinator_queue(cs.as_ptr() as *mut c_char) != 0 }
}

/// Returns `true` if the queue is a worker queue (not coordinator-internal).
pub fn is_worker_queue(name: &str) -> bool {
    let cs = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return false,
    };
    unsafe { ffi::tork_web_is_worker_queue(cs.as_ptr() as *mut c_char) != 0 }
}

/// Returns `true` if the queue is a task queue (worker queue, non-exclusive).
pub fn is_task_queue(name: &str) -> bool {
    let cs = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return false,
    };
    unsafe { ffi::tork_web_is_task_queue(cs.as_ptr() as *mut c_char) != 0 }
}

// ==================== Constants ====================

/// Scheduled job states.
pub mod scheduled_job_state {
    pub const ACTIVE: &str = "ACTIVE";
    pub const PAUSED: &str = "PAUSED";
}

/// Health check statuses.
pub mod health_status {
    pub const UP: &str = "UP";
    pub const DOWN: &str = "DOWN";
}

/// Well-known broker queue names.
pub mod queue {
    pub const PENDING: &str = "pending";
    pub const STARTED: &str = "started";
    pub const COMPLETED: &str = "completed";
    pub const ERROR: &str = "error";
    pub const DEFAULT: &str = "default";
    pub const HEARTBEAT: &str = "heartbeat";
    pub const JOBS: &str = "jobs";
    pub const LOGS: &str = "logs";
    pub const PROGRESS: &str = "progress";
    pub const REDELIVERIES: &str = "redeliveries";
    pub const EXCLUSIVE_PREFIX: &str = "x-";
}

// ==================== Tests ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_summary_roundtrip() {
        let json = r#"{"id":"js1","name":"my-job","state":"RUNNING","position":2,"taskCount":5,"progress":0.4,"createdAt":"2025-01-15T10:00:00Z"}"#;
        let js = JobSummary::from_json(json).expect("parse job summary");
        assert_eq!(js.id, "js1");
        assert_eq!(js.name, "my-job");
        assert_eq!(js.state, "RUNNING");
        assert_eq!(js.position, 2);
        assert_eq!(js.task_count, 5);
        assert!((js.progress - 0.4).abs() < f64::EPSILON);
        assert!(!js.created_at.is_empty());

        let out = js.to_json().expect("serialize");
        assert!(out.contains("\"id\":\"js1\""));
    }

    #[test]
    fn test_job_summary_with_timestamps() {
        let json = r#"{"id":"js2","state":"COMPLETED","createdAt":"2025-01-15T10:00:00Z","startedAt":"2025-01-15T10:00:01Z","completedAt":"2025-01-15T10:05:00Z"}"#;
        let js = JobSummary::from_json(json).expect("parse");
        assert!(!js.started_at.is_empty());
        assert!(!js.completed_at.is_empty());
        assert!(js.failed_at.is_empty());
    }

    #[test]
    fn test_scheduled_job_summary() {
        let json = r#"{"id":"sj1","name":"nightly-build","state":"ACTIVE","cron":"0 0 * * *","createdAt":"2025-01-01T00:00:00Z"}"#;
        let sjs = ScheduledJobSummary::from_json(json).expect("parse");
        assert_eq!(sjs.id, "sj1");
        assert_eq!(sjs.state, "ACTIVE");
        assert_eq!(sjs.cron, "0 0 * * *");
    }

    #[test]
    fn test_scheduled_job_full() {
        let json = r#"{
            "id":"sj2",
            "name":"etl-pipeline",
            "state":"ACTIVE",
            "cron":"*/5 * * * *",
            "output":"{{ tasks.extract.result }}",
            "createdAt":"2025-06-01T00:00:00Z",
            "tasks":[{"id":"t1","name":"extract","image":"python:3","run":"python extract.py"}],
            "inputs":{"bucket":"s3://data","region":"us-east-1"}
        }"#;
        let sj = ScheduledJob::from_json(json).expect("parse");
        assert_eq!(sj.id, "sj2");
        assert_eq!(sj.cron, "*/5 * * * *");
        assert!(!sj.tasks_json.is_empty());
        assert!(!sj.inputs_json.is_empty());

        let tasks = sj.tasks();
        assert_eq!(tasks.len(), 1);

        let inputs = sj.inputs();
        assert_eq!(inputs.get("bucket").map(String::as_str), Some("s3://data"));
    }

    #[test]
    fn test_queue_info_roundtrip() {
        let json = r#"{"name":"default","size":42,"subscribers":3,"unacked":7}"#;
        let qi = QueueInfo::from_json(json).expect("parse");
        assert_eq!(qi.name, "default");
        assert_eq!(qi.size, 42);
        assert_eq!(qi.subscribers, 3);
        assert_eq!(qi.unacked, 7);

        let out = qi.to_json().expect("serialize");
        assert!(out.contains("\"name\":\"default\""));
    }

    #[test]
    fn test_health_check() {
        let json = r#"{"status":"UP","version":"0.8.0"}"#;
        let hc = HealthCheck::from_json(json).expect("parse");
        assert_eq!(hc.status, "UP");
        assert!(hc.is_up());
        assert_eq!(hc.version, "0.8.0");

        let down = HealthCheck::from_json(r#"{"status":"DOWN","version":"0.8.0"}"#).unwrap();
        assert!(!down.is_up());
    }

    #[test]
    fn test_task_log_part() {
        let json = r#"{"id":"lp1","taskId":"t42","number":3,"contents":"hello world"}"#;
        let lp = TaskLogPart::from_json(json).expect("parse");
        assert_eq!(lp.id, "lp1");
        assert_eq!(lp.task_id, "t42");
        assert_eq!(lp.number, 3);
        assert_eq!(lp.contents, "hello world");
    }

    #[test]
    fn test_raw_page() {
        let json = r#"{"items":[{"name":"q1","size":10},{"name":"q2","size":20}],"number":1,"size":10,"totalPages":1,"totalItems":2}"#;
        let page = RawPage::from_json(json).expect("parse page");
        assert_eq!(page.number, 1);
        assert_eq!(page.total_items, 2);

        let typed: Page<QueueInfo> = page.into_typed().expect("deserialize items");
        assert_eq!(typed.items.len(), 2);
        assert_eq!(typed.items[0].name, "q1");
    }

    #[test]
    fn test_parse_jobs_page() {
        let json = r#"{"items":[{"id":"j1","name":"build","state":"COMPLETED"},{"id":"j2","name":"deploy","state":"RUNNING"}],"number":1,"size":10,"totalPages":1,"totalItems":2}"#;
        let page = parse_jobs_page(json).expect("parse");
        assert_eq!(page.items.len(), 2);
        assert_eq!(page.items[0].id, "j1");
        assert_eq!(page.items[1].state, "RUNNING");
        assert_eq!(page.total_items, 2);
    }

    #[test]
    fn test_parse_scheduled_jobs_page() {
        let json = r#"{"items":[{"id":"sj1","name":"nightly","state":"ACTIVE","cron":"0 0 * * *"}],"number":1,"size":10,"totalPages":1,"totalItems":1}"#;
        let page = parse_scheduled_jobs_page(json).expect("parse");
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].cron, "0 0 * * *");
    }

    #[test]
    fn test_parse_log_page() {
        let json = r#"{"items":[{"id":"lp1","taskId":"t1","number":1,"contents":"line 1"},{"id":"lp2","taskId":"t1","number":2,"contents":"line 2"}],"number":1,"size":25,"totalPages":1,"totalItems":2}"#;
        let page = parse_log_page(json).expect("parse");
        assert_eq!(page.items.len(), 2);
        assert_eq!(page.items[0].contents, "line 1");
        assert_eq!(page.items[1].number, 2);
    }

    #[test]
    fn test_queue_classification() {
        assert!(is_coordinator_queue("pending"));
        assert!(is_coordinator_queue("heartbeat"));
        assert!(is_coordinator_queue("jobs"));
        assert!(!is_coordinator_queue("default"));
        assert!(!is_coordinator_queue("my-custom-queue"));

        assert!(is_worker_queue("default"));
        assert!(!is_worker_queue("pending"));

        assert!(is_task_queue("default"));
        assert!(is_task_queue("gpu-workers"));
        assert!(!is_task_queue("pending"));
        assert!(!is_task_queue("x-exclusive-1"));
    }

    #[test]
    fn test_invalid_json_returns_none() {
        assert!(JobSummary::from_json("not json{{{").is_none());
        assert!(QueueInfo::from_json("").is_none());
        assert!(HealthCheck::from_json("{bad").is_none());
        assert!(TaskLogPart::from_json("").is_none());
        assert!(RawPage::from_json("nope").is_none());
    }

    #[test]
    fn test_constants() {
        assert_eq!(scheduled_job_state::ACTIVE, "ACTIVE");
        assert_eq!(scheduled_job_state::PAUSED, "PAUSED");
        assert_eq!(health_status::UP, "UP");
        assert_eq!(health_status::DOWN, "DOWN");
        assert_eq!(queue::PENDING, "pending");
        assert_eq!(queue::DEFAULT, "default");
        assert_eq!(queue::EXCLUSIVE_PREFIX, "x-");
    }
}
