use std::{fmt::Display, ops::Deref, ops::DerefMut};

use synthizer_sys::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct SynthizerError(syz_ErrorCode);

impl Display for SynthizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SynthizerError({})", self.0)
    }
}

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
        let rv = unsafe { syz_handleFree(self.0) };
        if rv != 0 {
            panic!("Failed to free handle");
        }
    }
}

pub fn initialize() -> Result<(), SynthizerError> {
    wrap(unsafe { syz_initialize() })
}

pub fn shutdown() -> Result<(), SynthizerError> {
    wrap(unsafe { syz_shutdown() })
}

pub struct Context(Handle);

impl Context {
    pub fn new() -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        let v = unsafe { syz_createContext(&mut *handle) };
        if v == 0 {
            Ok(Self(handle))
        } else {
            Err(SynthizerError(v))
        }
    }
}
