use ewin_cfg::log::*;
use ewin_const::models::model::*;
use ewin_job::{global::*, job::*};
use ewin_key::{global::*, model::*, watcher::*};
use ewin_utils::{files::file::*, util::*};
use std::{
    path::PathBuf,
    str::FromStr,
    sync::mpsc::Sender,
    thread::{self},
    time::{self},
};

pub fn watching_file() {
    if let Ok(tx) = TX_JOB.get().unwrap().try_lock() {
        let tx_watch = Sender::clone(&tx);

        tokio::spawn(async move {
            //  let (tx, _) = unbounded();
            let (tx, _) = std::sync::mpsc::channel();
            let mut watcher = FileWatcher::new(tx);
            loop {
                if let Some(Ok(mut watch_info)) = WATCH_INFO.get().map(|watch_info| watch_info.try_lock()) {
                    // Log::debug("watch_info", &watch_info);

                    if watch_info.mode == WatchMode::NotMonitor {
                        continue;
                    }
                    if watch_info.fullpath_org != watch_info.fullpath {
                        if watch_info.fullpath_org != String::default() {
                            Log::debug_s("w.unwatch");
                            watcher.unwatch(&PathBuf::from_str(&watch_info.fullpath_org).unwrap());
                        }
                        set_watch_history(watch_info.fullpath.clone(), &mut watch_info, &mut watcher, true);
                        watcher.watch(&PathBuf::from_str(&watch_info.fullpath).unwrap());
                    } else {
                        set_watch_history(watch_info.fullpath.clone(), &mut watch_info, &mut watcher, false);
                    }

                    for (fullpath_str, unixtime_str) in watch_info.history_set.clone() {
                        if watch_info.fullpath == fullpath_str {
                            let job = Job { cont: JobCont::Watch(JobWatch { fullpath_str, unixtime_str }) }; // job_type: JobType::Watch, job_watch: Some(JobWatch { fullpath_str, unixtime_str }), ..Job::default() };
                            let _ = tx_watch.send(job);
                        }
                    }
                    watch_info.fullpath_org = watch_info.fullpath.clone();
                }
                thread::sleep(time::Duration::from_millis(3000));
            }
        });
    }
}

pub fn set_watch_history(path_str: String, watch_info: &mut tokio::sync::MutexGuard<WatchInfo>, watcher: &mut FileWatcher, is_forced: bool) {
    if is_forced {
        if let Some(modified_time) = File::get_modified_time(&path_str) {
            let unixtime_str = to_unixtime_str(modified_time);
            watch_info.history_set.insert((path_str, unixtime_str));
            watcher.state.lock().unwrap().events.clear();
        }
    } else {
        let unixtime_str = get_unixtime_str();
        let events = watcher.take_events();
        let modified = events.iter().any(|event| watcher.wants_event(event));

        if modified {
            Log::debug("modified events", &events);

            watch_info.history_set.retain(|(path, _)| *path == path_str);
            watch_info.history_set.insert((path_str, unixtime_str));
            watcher.state.lock().unwrap().events.clear();
        }
    }
}
