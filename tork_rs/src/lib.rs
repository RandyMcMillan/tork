//! # tork-rs
//!
//! Rust FFI bindings for the [Tork](https://tork.run) workflow engine.

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

unsafe fn c_to_string(p: *const c_char) -> String {
    if p.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
    }
}

// --- Job ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
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
    pub output: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub result: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(default)]
    pub position: i32,
    #[serde(default)]
    pub task_count: i32,
}

impl Job {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let cj = ffi::tork_job_from_json(cs.as_ptr() as *mut c_char);
            if cj.is_null() { return None; }
            let job = Job {
                id: c_to_string((*cj).id),
                parent_id: c_to_string((*cj).parent_id),
                name: c_to_string((*cj).name),
                description: c_to_string((*cj).description),
                state: c_to_string((*cj).state),
                output: c_to_string((*cj).output),
                result: c_to_string((*cj).result),
                error: c_to_string((*cj).error),
                position: (*cj).position,
                task_count: (*cj).task_count,
            };
            ffi::tork_job_free(cj);
            Some(job)
        }
    }

    pub fn to_json(&self) -> Option<String> {
        let json_in = serde_json::to_string(self).ok()?;
        let cs = CString::new(json_in).ok()?;
        unsafe {
            let cj = ffi::tork_job_from_json(cs.as_ptr() as *mut c_char);
            if cj.is_null() { return None; }
            let out = ffi::tork_job_to_json(cj);
            let result = if out.is_null() {
                None
            } else {
                let s = c_to_string(out);
                ffi::tork_free_string(out);
                Some(s)
            };
            ffi::tork_job_free(cj);
            result
        }
    }
}

// --- Task ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub run: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub queue: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub result: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "var")]
    pub var_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "if")]
    pub if_expr: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub timeout: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub gpus: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub workdir: String,
    #[serde(default)]
    pub position: i32,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub progress: f64,
}

impl Task {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let ct = ffi::tork_task_from_json(cs.as_ptr() as *mut c_char);
            if ct.is_null() { return None; }
            let task = Task {
                id: c_to_string((*ct).id),
                name: c_to_string((*ct).name),
                description: c_to_string((*ct).description),
                state: c_to_string((*ct).state),
                image: c_to_string((*ct).image),
                run: c_to_string((*ct).run),
                queue: c_to_string((*ct).queue),
                error: c_to_string((*ct).error),
                result: c_to_string((*ct).result),
                var_name: c_to_string((*ct).var_name),
                if_expr: c_to_string((*ct).if_expr),
                timeout: c_to_string((*ct).timeout),
                gpus: c_to_string((*ct).gpus),
                workdir: c_to_string((*ct).workdir),
                position: (*ct).position,
                priority: (*ct).priority,
                progress: (*ct).progress,
            };
            ffi::tork_task_free(ct);
            Some(task)
        }
    }

    pub fn is_active(&self) -> bool {
        let cs = match CString::new(self.state.as_str()) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::tork_task_is_active(cs.as_ptr() as *mut c_char) != 0 }
    }
}

// --- Node ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub queue: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    #[serde(default)]
    pub cpu_percent: f64,
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub task_count: i32,
}

impl Node {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let cn = ffi::tork_node_from_json(cs.as_ptr() as *mut c_char);
            if cn.is_null() { return None; }
            let node = Node {
                id: c_to_string((*cn).id),
                name: c_to_string((*cn).name),
                hostname: c_to_string((*cn).hostname),
                queue: c_to_string((*cn).queue),
                status: c_to_string((*cn).status),
                version: c_to_string((*cn).version),
                cpu_percent: (*cn).cpu_percent,
                port: (*cn).port,
                task_count: (*cn).task_count,
            };
            ffi::tork_node_free(cn);
            Some(node)
        }
    }
}

// --- Metrics ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    pub jobs_running: i32,
    pub tasks_running: i32,
    pub nodes_running: i32,
    pub nodes_cpu_percent: f64,
}

impl Metrics {
    pub fn from_json(json: &str) -> Option<Self> {
        let cs = CString::new(json).ok()?;
        unsafe {
            let cm = ffi::tork_metrics_from_json(cs.as_ptr() as *mut c_char);
            if cm.is_null() { return None; }
            let metrics = Metrics {
                jobs_running: (*cm).jobs_running,
                tasks_running: (*cm).tasks_running,
                nodes_running: (*cm).nodes_running,
                nodes_cpu_percent: (*cm).nodes_cpu_percent,
            };
            ffi::tork_metrics_free(cm);
            Some(metrics)
        }
    }
}

// --- Version ---

pub fn version() -> String {
    unsafe {
        let v = ffi::tork_version();
        let s = c_to_string(v);
        ffi::tork_free_string(v);
        s
    }
}

pub fn git_commit() -> String {
    unsafe {
        let v = ffi::tork_git_commit();
        let s = c_to_string(v);
        ffi::tork_free_string(v);
        s
    }
}

// --- State constants ---

pub mod job_state {
    pub const PENDING: &str = "PENDING";
    pub const SCHEDULED: &str = "SCHEDULED";
    pub const RUNNING: &str = "RUNNING";
    pub const CANCELLED: &str = "CANCELLED";
    pub const COMPLETED: &str = "COMPLETED";
    pub const FAILED: &str = "FAILED";
    pub const RESTART: &str = "RESTART";
}

pub mod task_state {
    pub const CREATED: &str = "CREATED";
    pub const PENDING: &str = "PENDING";
    pub const SCHEDULED: &str = "SCHEDULED";
    pub const RUNNING: &str = "RUNNING";
    pub const CANCELLED: &str = "CANCELLED";
    pub const STOPPED: &str = "STOPPED";
    pub const COMPLETED: &str = "COMPLETED";
    pub const FAILED: &str = "FAILED";
    pub const SKIPPED: &str = "SKIPPED";
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
        println!("tork version: {v}");
    }

    #[test]
    fn test_job_roundtrip() {
        let json = r#"{"id":"j1","name":"test-job","state":"RUNNING","position":3}"#;
        let job = Job::from_json(json).expect("parse job");
        assert_eq!(job.id, "j1");
        assert_eq!(job.name, "test-job");
        assert_eq!(job.state, "RUNNING");
        assert_eq!(job.position, 3);
        let out = job.to_json().expect("serialize job");
        assert!(out.contains("\"id\":\"j1\""));
    }

    #[test]
    fn test_task_from_json() {
        let json = r#"{"id":"t1","name":"hello","state":"RUNNING","image":"alpine:latest","run":"echo hello","priority":5,"progress":0.75}"#;
        let task = Task::from_json(json).expect("parse task");
        assert_eq!(task.id, "t1");
        assert_eq!(task.image, "alpine:latest");
        assert_eq!(task.priority, 5);
        assert!(task.is_active());
    }

    #[test]
    fn test_task_is_active() {
        let mut t = Task::default();
        t.state = "RUNNING".into();
        assert!(t.is_active());
        t.state = "COMPLETED".into();
        assert!(!t.is_active());
    }

    #[test]
    fn test_node_from_json() {
        let json = r#"{"id":"n1","name":"worker-1","status":"UP","cpuPercent":45.5,"port":8080}"#;
        let node = Node::from_json(json).expect("parse node");
        assert_eq!(node.id, "n1");
        assert_eq!(node.status, "UP");
        assert_eq!(node.port, 8080);
    }

    #[test]
    fn test_metrics_from_json() {
        let json = r#"{"jobs":{"running":5},"tasks":{"running":12},"nodes":{"online":3,"cpuPercent":67.2}}"#;
        let m = Metrics::from_json(json).expect("parse metrics");
        assert_eq!(m.jobs_running, 5);
        assert_eq!(m.tasks_running, 12);
        assert_eq!(m.nodes_running, 3);
    }

    #[test]
    fn test_invalid_json_returns_none() {
        assert!(Job::from_json("not json{{{").is_none());
        assert!(Task::from_json("").is_none());
    }
}
