use std::{ffi::CString, ops::Deref, ops::DerefMut, path::Path, ptr::null_mut};

use log::Level;
use synthizer_sys::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Synthizer error: {0}")]
pub struct SynthizerError(syz_ErrorCode);

#[derive(Clone, Debug)]
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

#[repr(i32)]
enum Properties {
    Azimuth = SYZ_PROPERTIES_SYZ_P_AZIMUTH,
    Buffer = SYZ_PROPERTIES_SYZ_P_BUFFER,
    ClosenessBoost = SYZ_PROPERTIES_SYZ_P_CLOSENESS_BOOST,
    ClosenessBoostDistance = SYZ_PROPERTIES_SYZ_P_CLOSENESS_BOOST_DISTANCE,
    DistanceMax = SYZ_PROPERTIES_SYZ_P_DISTANCE_MAX,
    DistanceModel = SYZ_PROPERTIES_SYZ_P_DISTANCE_MODEL,
    DistanceRef = SYZ_PROPERTIES_SYZ_P_DISTANCE_REF,
    Elevation = SYZ_PROPERTIES_SYZ_P_ELEVATION,
    Gain = SYZ_PROPERTIES_SYZ_P_GAIN,
    PannerStrategy = SYZ_PROPERTIES_SYZ_P_PANNER_STRATEGY,
    PanningScalar = SYZ_PROPERTIES_SYZ_P_PANNING_SCALAR,
    Position = SYZ_PROPERTIES_SYZ_P_POSITION,
    Orientation = SYZ_PROPERTIES_SYZ_P_ORIENTATION,
    Rolloff = SYZ_PROPERTIES_SYZ_P_ROLLOFF,
    Looping = SYZ_PROPERTIES_SYZ_P_LOOPING,
    NoiseType = SYZ_PROPERTIES_SYZ_P_NOISE_TYPE,
}

impl Handle {
    fn get_i(&self, property: i32) -> Result<*mut i32, SynthizerError> {
        let out: *mut i32 = null_mut();
        let v = unsafe { syz_getI(out, self.0, property) };
        if v == 0 {
            Ok(out)
        } else {
            Err(SynthizerError(v))
        }
    }

    fn set_i(&self, property: i32, value: i32) -> Result<(), SynthizerError> {
        let v = unsafe { syz_setI(self.0, property, value) };
        if v == 0 {
            Ok(())
        } else {
            Err(SynthizerError(v))
        }
    }

    fn get_d(&self, property: i32) -> Result<*mut f64, SynthizerError> {
        let out: *mut f64 = null_mut();
        let v = unsafe { syz_getD(out, self.0, property) };
        if v == 0 {
            Ok(out)
        } else {
            Err(SynthizerError(v))
        }
    }

    fn set_d(&self, property: i32, value: f64) -> Result<(), SynthizerError> {
        let v = unsafe { syz_setD(self.0, property, value) };
        if v == 0 {
            Ok(())
        } else {
            Err(SynthizerError(v))
        }
    }

    fn get_o(&self, property: i32) -> Result<Handle, SynthizerError> {
        let out: *mut u64 = null_mut();
        let v = unsafe { syz_getO(out, self.0, property) };
        if v == 0 {
            Ok(Handle(out as u64))
        } else {
            Err(SynthizerError(v))
        }
    }

    fn set_o(&self, property: i32, value: Handle) -> Result<(), SynthizerError> {
        let v = unsafe { syz_setO(self.0, property, value.0) };
        if v == 0 {
            Ok(())
        } else {
            Err(SynthizerError(v))
        }
    }

    fn get_d3(&self, property: i32) -> Result<(*mut f64, *mut f64, *mut f64), SynthizerError> {
        let x: *mut f64 = null_mut();
        let y: *mut f64 = null_mut();
        let z: *mut f64 = null_mut();
        let v = unsafe { syz_getD3(x, y, z, self.0, property) };
        if v == 0 {
            Ok((x, y, z))
        } else {
            Err(SynthizerError(v))
        }
    }

    fn set_d3(&self, property: i32, x: f64, y: f64, z: f64) -> Result<(), SynthizerError> {
        let v = unsafe { syz_setD3(self.0, property, x, y, z) };
        if v == 0 {
            Ok(())
        } else {
            Err(SynthizerError(v))
        }
    }

    fn get_d6(
        &self,
        property: i32,
    ) -> Result<(*mut f64, *mut f64, *mut f64, *mut f64, *mut f64, *mut f64), SynthizerError> {
        let x1: *mut f64 = null_mut();
        let y1: *mut f64 = null_mut();
        let z1: *mut f64 = null_mut();
        let x2: *mut f64 = null_mut();
        let y2: *mut f64 = null_mut();
        let z2: *mut f64 = null_mut();
        let v = unsafe { syz_getD6(x1, y1, z1, x2, y2, z2, self.0, property) };
        if v == 0 {
            Ok((x1, y1, z1, x2, y2, z2))
        } else {
            Err(SynthizerError(v))
        }
    }

    fn set_d6(
        &self,
        property: i32,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> Result<(), SynthizerError> {
        let v = unsafe { syz_setD6(self.0, property, x1, y1, z1, x2, y2, z2) };
        if v == 0 {
            Ok(())
        } else {
            Err(SynthizerError(v))
        }
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

pub fn configure_logging_backend(backend: LoggingBackend) -> Result<(), SynthizerError> {
    let backend = match backend {
        LoggingBackend::Stderr => SYZ_LOGGING_BACKEND_SYZ_LOGGING_BACKEND_STDERR,
    };
    let param = null_mut();
    wrap(unsafe { syz_configureLoggingBackend(backend, param) })
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

fn initialize() -> Result<(), SynthizerError> {
    wrap(unsafe { syz_initialize() })
}

fn shutdown() -> Result<(), SynthizerError> {
    wrap(unsafe { syz_shutdown() })
}

#[derive(Clone, Debug)]
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

    pub fn new_streaming_generator<S: Into<String>>(
        &mut self,
        protocol: Protocol,
        path: &Path,
        options: S,
    ) -> Result<StreamingGenerator, SynthizerError> {
        StreamingGenerator::new(&self, protocol, path, options)
    }
}

impl Deref for Context {
    type Target = syz_Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! make_superclass {
    ($superclass:ident) => {
        trait $superclass {
            fn handle(&self) -> &Handle;
        }
    };
}

macro_rules! make_subclass {
    ($subclass:ident, $superclass:ty) => {
        impl $superclass for $subclass {
            fn handle(&self) -> &Handle {
                &self.0
            }
        }
    };
}

make_superclass!(Generator);

make_superclass!(Source);

pub enum Protocol {
    File,
}

#[derive(Clone, Debug)]
pub struct StreamingGenerator(Handle);

impl StreamingGenerator {
    fn new<S: Into<String>>(
        context: &Context,
        protocol: Protocol,
        path: &Path,
        options: S,
    ) -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        let protocol = match protocol {
            Protocol::File => String::from("file"),
        };
        let protocol = CString::new(protocol.as_bytes()).expect("Unable to create C string");
        let protocol = protocol.as_ptr() as *const i8;
        let path = path.as_os_str().to_string_lossy();
        let path = CString::new(path.as_bytes()).expect("Unable to create C string");
        let path = path.as_ptr() as *const i8;
        let options = options.into();
        let options = CString::new(options.as_bytes()).expect("Unable to create C string");
        let options = options.as_ptr() as *const i8;
        let v = unsafe {
            syz_createStreamingGenerator(&mut *handle, **context, protocol, path, options)
        };
        if v == 0 {
            Ok(Self(handle))
        } else {
            Err(SynthizerError(v))
        }
    }
}

make_subclass!(StreamingGenerator, Generator);

#[derive(Clone, Debug)]
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

unsafe impl Send for Synthizer {}

unsafe impl Sync for Synthizer {}

impl Drop for Synthizer {
    fn drop(&mut self) {
        shutdown().expect("Failed to shut down");
    }
}
