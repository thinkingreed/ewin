#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use std::sync::mpsc::Sender;

    use crate::job::*;
    use once_cell::sync::OnceCell;
    use tokio::sync::Mutex;
    pub static TX_JOB: OnceCell<Mutex<Sender<Job>>> = OnceCell::new();
}

pub mod job;
