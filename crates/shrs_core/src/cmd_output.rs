use std::{os::unix::process::ExitStatusExt, process::ExitStatus};
/// Describes the output of a command
#[derive(Clone, Debug)]
pub struct CmdOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}
impl CmdOutput {
    pub fn new(status: i32) -> Self {
        CmdOutput {
            stdout: String::new(),
            stderr: String::new(),
            status: ExitStatus::from_raw(status << 8),
        }
    }
    pub fn success() -> Self {
        CmdOutput::new(0)
    }
    pub fn error() -> Self {
        CmdOutput::new(1)
    }
    pub fn error_with_status(status: i32) -> Self {
        CmdOutput::new(status)
    }
    pub fn set_output(&mut self, out: String, err: String) {
        self.stdout = out;
        self.stderr = err;
    }
}
