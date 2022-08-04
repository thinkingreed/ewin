use crossbeam_channel::unbounded;
use ewin_cfg::log::Log;
use notify::{event::*, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::VecDeque,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

pub struct FileWatcher {
    inner: RecommendedWatcher,
    pub state: Arc<Mutex<WatcherState>>,
}
#[derive(Debug, Default)]
pub struct WatcherState {
    pub events: EventQueue,
}

/// A trait for types which can be notified of new events.
/// New events are accessible through the `FileWatcher` instance.
pub trait Notify: Send {
    fn notify(&self);
}

pub type EventQueue = VecDeque<Event>;

impl Notify for std::sync::mpsc::Sender<bool> {
    fn notify(&self) {
        if let Err(err) = self.send(true) {
            Log::debug("channel send err", &err)
        }
    }
}

impl FileWatcher {
    pub fn new<T: Notify + 'static>(peer: T) -> Self {
        let (tx_event, rx_event) = unbounded();

        let state = Arc::new(Mutex::new(WatcherState::default()));
        let state_clone = state.clone();

        let inner = Watcher::new(tx_event).unwrap();
        thread::spawn(move || loop {
            while let Ok(Ok(event)) = rx_event.recv() {
                let mut state = state_clone.lock().unwrap();
                state.events.push_back(event);
                peer.notify();
            }
        });

        FileWatcher { inner, state }
    }

    /// Begin watching `path`. As `Event`s (documented in the
    /// [notify](https://docs.rs/notify) crate) arrive, they are stored
    /// with the associated `token` and a task is added to the runloop's
    /// idle queue.
    ///
    /// Delivery of events then requires that the runloop's handler
    /// correctly forward the `handle_idle` call to the interested party.
    pub fn watch(&mut self, path: &Path) {
        self.watch_impl(path);
    }

    fn watch_impl(&mut self, path: &Path) {
        let path = match path.canonicalize() {
            Ok(ref p) => p.to_owned(),
            Err(e) => {
                Log::error("error watching", &format!("{:?}:{:?}", path, e));
                return;
            }
        };
        if let Err(e) = self.inner.watch(&path, RecursiveMode::NonRecursive) {
            Log::error("watching error ", &e);
        }
    }

    /// Removes the provided token/path pair from the watch list.
    /// Does not stop watching this path, if it is associated with
    /// other tokens.
    pub fn unwatch(&mut self, path: &Path) {
        if let Err(e) = self.inner.unwatch(path) {
            Log::error("unwatching error", &e);
        }
    }

    pub fn take_events(&mut self) -> VecDeque<Event> {
        // let result = self.state.lock();
        match self.state.lock() {
            Ok(state) => {
                let WatcherState { ref events, .. } = *state;
                return events.clone();
            }
            Err(err) => {
                Log::error("take_events error", &err);
            }
        };

        return VecDeque::new();
    }

    pub fn wants_event(&self, event: &Event) -> bool {
        Log::debug("event.kind", &event.kind);
        match &event.kind {
            EventKind::Modify(ModifyKind::Data(DataChange::Any)) | EventKind::Modify(ModifyKind::Any) | EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) => return true,
            EventKind::Create(CreateKind::Any) | EventKind::Remove(RemoveKind::Any) | EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => return false,
            _ => false,
        }
    }
}
