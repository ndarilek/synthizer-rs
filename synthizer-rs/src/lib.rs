use std::{ops::Deref, ops::DerefMut};

use log::Level;
use synthizer_sys::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Synthizer error: {0}")]
pub struct SynthizerError(syz_ErrorCode);

struct Handle(syz_Handle);

impl Deref for Handle {
    type Target = syz_Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Handle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn wrap(v: i32) -> Result<(), SynthizerError> {
    if v == 0 {
        Ok(())
    } else {
        Err(SynthizerError(v))
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let v = unsafe { syz_handleFree(self.0) };
        if v != 0 {
            panic!(format!("Failed to free handle: error code {}", v));
        }
    }
}

pub enum LoggingBackend {
    Stderr = SYZ_LOGGING_BACKEND_SYZ_LOGGING_BACKEND_STDERR as isize,
}

pub fn set_log_level(level: Level) {
    let level = match level {
        Level::Error => SYZ_LOG_LEVEL_SYZ_LOG_LEVEL_ERROR,
        Level::Warn => SYZ_LOG_LEVEL_SYZ_LOG_LEVEL_WARN,
        Level::Info => SYZ_LOG_LEVEL_SYZ_LOG_LEVEL_INFO,
        Level::Debug => SYZ_LOG_LEVEL_SYZ_LOG_LEVEL_DEBUG,
        _ => panic!("Level not supported"),
    };
    unsafe { syz_setLogLevel(level) };
}

pub fn initialize() -> Result<(), SynthizerError> {
    wrap(unsafe { syz_initialize() })
}

pub fn shutdown() -> Result<(), SynthizerError> {
    wrap(unsafe { syz_shutdown() })
}

pub struct Context(Handle);

impl Context {
    fn new() -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        let v = unsafe { syz_createContext(&mut *handle) };
        if v == 0 {
            Ok(Self(handle))
        } else {
            Err(SynthizerError(v))
        }
    }
}

pub struct Synthizer;

impl Synthizer {
    pub fn new() -> Result<Self, SynthizerError> {
        initialize()?;
        Ok(Synthizer)
    }

    pub fn new_context(&self) -> Result<Context, SynthizerError> {
        Context::new()
    }
}

impl Drop for Synthizer {
    fn drop(&mut self) {
        shutdown()
            .expect("Failed to shut down");
    }
}