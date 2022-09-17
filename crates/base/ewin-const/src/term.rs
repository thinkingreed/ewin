use crossterm::terminal::size;
use std::sync::Mutex;

use crate::{def::*, global::*};

pub fn get_term_size() -> (usize, usize) {
    let term_size = if let Some(Ok(_term_size)) = TERM_SIZE.get().map(|term| term.try_lock()) {
        //     if let Some(Ok(tx_job)) = TX_JOB.get().map(|tx| tx.try_lock()) {

        // let term_size = if let Some(Ok(_term_size)) = TERM_SIZE.get().try_lock() {
        //  if let Ok(_term_size) = ___term_size.try_lock() {
        (_term_size.cols, _term_size.rows)
    } else {
        let size = size().unwrap_or((TERM_MINIMUM_WIDTH as u16, TERM_MINIMUM_HEIGHT as u16));
        let _ = TERM_SIZE.set(Mutex::new(TermSize { cols: size.0, rows: size.1 }));
        (size.0, size.1)
    };

    let (cols, rows) = (term_size.0, term_size.1);
    if (cols, rows) == (1, 1) {
        (TERM_MINIMUM_WIDTH, TERM_MINIMUM_HEIGHT)
    } else {
        (cols as usize, rows as usize)
    }
}

pub fn set_term_size() {
    let (cols, rows) = size().unwrap_or((TERM_MINIMUM_WIDTH as u16, TERM_MINIMUM_HEIGHT as u16));
    TERM_SIZE.get().unwrap().try_lock().map(|mut term_size| *term_size = TermSize { cols, rows }).unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TermSize {
    pub cols: u16,
    pub rows: u16,
}
