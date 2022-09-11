use crate::{global::*, model::*};
use ewin_cfg::log::*;

impl GrepCancel {
    pub fn is_canceled() -> bool {
        let state = if let Some(Some(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        GrepCancelType::Canceled == state || GrepCancelType::None == state
    }
    pub fn is_canceling() -> bool {
        let state = if let Some(Some(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        GrepCancelType::Canceling == state
    }
    pub fn is_cancel() -> bool {
        {
            let state = if let Some(Some(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
            Log::debug("state", &state);
        }

        return GrepCancel::is_canceling() || GrepCancel::is_canceled();
    }
}

#[derive(Default, Debug, Clone)]
pub struct GrepCancel {}
