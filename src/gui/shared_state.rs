// Thread-safe shared state for egui worker thread communication

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use eframe::egui::Context;

/// Shared state between UI thread and worker threads
#[derive(Default)]
pub struct SharedState
{
        pub progress: f32,
        pub total_ops: u32,
        pub completed_ops: u32,
        pub status_message: String,
        pub log_messages: Vec<String>,
        pub is_running: bool,
        pub is_complete: bool,
        pub error: Option<String>,
}

impl SharedState
{
        pub fn reset(&mut self, total_ops: u32)
        {
                self.progress = 0.0;
                self.total_ops = total_ops;
                self.completed_ops = 0;
                self.status_message.clear();
                self.log_messages.clear();
                self.is_running = true;
                self.is_complete = false;
                self.error = None;
        }
}

/// Context for worker threads to report progress (replaces `ParallelContext`)
pub struct WorkerContext
{
        pub shared: Arc<Mutex<SharedState>>,
        pub ctx: Context,
        pub completed_ops: AtomicU32,
        pub total_ops: u32,
        pub input_values: std::collections::HashMap<String, String>,
}

impl WorkerContext
{
        pub fn new(
                shared: Arc<Mutex<SharedState>>,
                ctx: Context,
                total_ops: u32,
                input_values: std::collections::HashMap<String, String>,
        ) -> Arc<Self>
        {
                {
                        let mut state = shared.lock().unwrap();
                        state.reset(total_ops);
                }
                Arc::new(Self {
                        shared,
                        ctx,
                        completed_ops: AtomicU32::new(0),
                        total_ops,
                        input_values,
                })
        }

        /// Post status message to log (replaces `WM_STATUS`)
        pub fn post_status(&self, status: &str)
        {
                {
                        let mut state = self.shared.lock().unwrap();
                        state.status_message = status.to_string();
                        state.log_messages.push(status.to_string());
                }
                self.ctx.request_repaint();
        }

        /// Report progress increment (replaces `WM_PROGRESS`)
        pub fn report_progress(&self)
        {
                let completed = self.completed_ops.fetch_add(1, Ordering::SeqCst) + 1;
                {
                        let mut state = self.shared.lock().unwrap();
                        state.completed_ops = completed;
                        state.progress = completed as f32 / self.total_ops as f32;
                }
                self.ctx.request_repaint();
        }

        /// Signal completion (replaces `WM_COMPLETE`)
        pub fn post_complete(&self)
        {
                {
                        let mut state = self.shared.lock().unwrap();
                        state.is_running = false;
                        state.is_complete = true;
                        state.progress = 1.0;
                        state.status_message = "Done! Reboot for changes to take full effect.".to_string();
                }
                self.ctx.request_repaint();
        }

        /// Signal error (replaces `WM_ERROR`)
        pub fn post_error(&self, error: String)
        {
                {
                        let mut state = self.shared.lock().unwrap();
                        state.is_running = false;
                        state.error = Some(error.clone());
                        state.status_message = format!("Error: {error}");
                        state.log_messages.push(format!("ERROR: {error}"));
                }
                self.ctx.request_repaint();
        }
}
