use parking_lot::Mutex;
use std::{
    ffi::{c_void, CString},
    os::raw::{c_char, c_int},
    sync::atomic::AtomicPtr,
};

pub fn init(
    debug_callback: extern "C" fn(*mut c_void, c_int, *const c_char),
    context_ptr: *mut c_void,
) {
    *DEBUG_INFO.lock() = Some(DebugInfo::new(debug_callback, context_ptr));
}

pub static DEBUG_INFO: Mutex<Option<DebugInfo>> = Mutex::new(None);

macro_rules! debug_print {
    ($level:expr, $s:expr) => {
        debug_print!($level, $s,)
    };
    ($level:expr, $s:expr, $($arg:expr),*) => {{
        if cfg!(debug_assertions) || $level <= $crate::debug::M64Message::Warning {
            $crate::debug::__print_debug_message($level, format!($s $(, $arg)*));
        }
    }};
}

#[doc(hidden)]
pub(crate) fn __print_debug_message(level: M64Message, message: String) {
    if let Some(ref di) = *DEBUG_INFO.lock() {
        let context = di.context_ptr.load(::std::sync::atomic::Ordering::Relaxed);
        let message = CString::new(message).unwrap();

        (di.callback)(context, level as c_int, message.as_ptr());
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum M64Message {
    Error = 1,
    Warning,
    Info,
    Status,
    Verbose,
}

#[derive(Debug)]
pub struct DebugInfo {
    callback: extern "C" fn(*mut c_void, c_int, *const c_char),
    context_ptr: AtomicPtr<c_void>,
}

impl DebugInfo {
    fn new(
        debug_callback: extern "C" fn(*mut c_void, c_int, *const c_char),
        context: *mut c_void,
    ) -> Self {
        Self {
            callback: debug_callback,
            context_ptr: AtomicPtr::new(context),
        }
    }
}
