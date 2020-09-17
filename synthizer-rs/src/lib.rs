use std::{ffi::CString, ops::Deref, ops::DerefMut, path::Path, ptr::null_mut, time::Duration};

use enum_primitive_derive::Primitive;
use log::Level;
use num_traits::ToPrimitive;
use synthizer_sys::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Synthizer error: {0}")]
pub struct SynthizerError(syz_ErrorCode);

macro_rules! wrap {
    ($call:expr) => {{
        let v = $call;
        if v == 0 {
            Ok(())
        } else {
            Err(SynthizerError(v))
        }
    }};
    ($call:expr, $rv:expr) => {{
        let v = $call;
        if v == 0 {
            Ok($rv)
        } else {
            Err(SynthizerError(v))
        }
    }};
}

pub enum LoggingBackend {
    Stderr = SYZ_LOGGING_BACKEND_SYZ_LOGGING_BACKEND_STDERR as isize,
}

pub fn configure_logging_backend(backend: LoggingBackend) -> Result<(), SynthizerError> {
    let backend = match backend {
        LoggingBackend::Stderr => SYZ_LOGGING_BACKEND_SYZ_LOGGING_BACKEND_STDERR,
    };
    let param = null_mut();
    wrap!(unsafe { syz_configureLoggingBackend(backend, param) })
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
    wrap!(unsafe { syz_initialize() })
}

fn shutdown() -> Result<(), SynthizerError> {
    wrap!(unsafe { syz_shutdown() })
}

#[derive(Clone, Debug)]
pub struct Handle(syz_Handle);

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

#[derive(Primitive)]
#[repr(i32)]
enum Property {
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

#[derive(Primitive)]
#[repr(i32)]
enum PannerStrategy {
    HRTF = SYZ_PANNER_STRATEGY_SYZ_PANNER_STRATEGY_HRTF,
    Stereo = SYZ_PANNER_STRATEGY_SYZ_PANNER_STRATEGY_STEREO,
    Count = SYZ_PANNER_STRATEGY_SYZ_PANNER_STRATEGY_COUNT,
}

impl Handle {
    fn get_i(&self, property: i32) -> Result<i32, SynthizerError> {
        let out: *mut i32 = null_mut();
        wrap!(unsafe { syz_getI(out, self.0, property) }, out as i32)
    }

    fn set_i(&self, property: i32, value: i32) -> Result<(), SynthizerError> {
        wrap!(unsafe { syz_setI(self.0, property, value) })
    }

    fn get_d(&self, property: i32) -> Result<*mut f64, SynthizerError> {
        let out: *mut f64 = null_mut();
        wrap!(unsafe { syz_getD(out, self.0, property) }, out)
    }

    fn set_d(&self, property: i32, value: f64) -> Result<(), SynthizerError> {
        wrap!(unsafe { syz_setD(self.0, property, value) })
    }

    fn get_o(&self, property: i32) -> Result<Handle, SynthizerError> {
        let out: *mut u64 = null_mut();
        wrap!(
            unsafe { syz_getO(out, self.0, property) },
            Handle(out as u64)
        )
    }

    fn set_o(&self, property: i32, value: Handle) -> Result<(), SynthizerError> {
        wrap!(unsafe { syz_setO(self.0, property, value.0) })
    }

    fn get_d3(&self, property: i32) -> Result<(*mut f64, *mut f64, *mut f64), SynthizerError> {
        let x: *mut f64 = null_mut();
        let y: *mut f64 = null_mut();
        let z: *mut f64 = null_mut();
        wrap!(unsafe { syz_getD3(x, y, z, self.0, property) }, (x, y, z))
    }

    fn set_d3(&self, property: i32, x: f64, y: f64, z: f64) -> Result<(), SynthizerError> {
        wrap!(unsafe { syz_setD3(self.0, property, x, y, z) })
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
        wrap!(
            unsafe { syz_getD6(x1, y1, z1, x2, y2, z2, self.0, property) },
            (x1, y1, z1, x2, y2, z2)
        )
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
        wrap!(unsafe { syz_setD6(self.0, property, x1, y1, z1, x2, y2, z2) })
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

unsafe impl Send for Handle {}

unsafe impl Sync for Handle {}

pub enum Protocol {
    File,
}

#[derive(Clone, Debug)]
pub struct Buffer(Handle);

impl Buffer {
    pub fn new<S: Into<String>>(
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
        wrap!(
            unsafe { syz_createBufferFromStream(&mut *handle, protocol, path, options) },
            Self(handle)
        )
    }

    pub fn get_channels(&self) -> Result<u32, SynthizerError> {
        let out: *mut u32 = null_mut();
        wrap!(unsafe { syz_bufferGetChannels(out, *self.0) }, out as u32)
    }

    pub fn get_length_in_samples(&self) -> Result<u32, SynthizerError> {
        let out: *mut u32 = null_mut();
        wrap!(
            unsafe { syz_bufferGetLengthInSamples(out, *self.0) },
            out as u32
        )
    }

    pub fn get_length_in_seconds(&self) -> Result<f64, SynthizerError> {
        let out: *mut f64 = null_mut();
        wrap!(unsafe { syz_bufferGetLengthInSeconds(out, *self.0) }, {
            let out = unsafe { out.as_ref() };
            let out = out.cloned();
            out.unwrap()
        })
    }

    pub fn get_duration(&self) -> Result<Duration, SynthizerError> {
        let seconds = self.get_length_in_seconds()?;
        let seconds = seconds as u64;
        Ok(Duration::from_secs(seconds))
    }
}

unsafe impl Send for Buffer {}

unsafe impl Sync for Buffer {}

#[derive(Clone, Debug)]
pub struct Context(Handle);

impl Context {
    fn new() -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        wrap!(unsafe { syz_createContext(&mut *handle) }, Self(handle))
    }

    pub fn new_streaming_generator<S: Into<String>>(
        &mut self,
        protocol: Protocol,
        path: &Path,
        options: S,
    ) -> Result<StreamingGenerator, SynthizerError> {
        StreamingGenerator::new(&self, protocol, path, options)
    }

    pub fn new_buffer_generator(&mut self) -> Result<BufferGenerator, SynthizerError> {
        BufferGenerator::new(&self)
    }

    pub fn new_direct_source(&mut self) -> Result<DirectSource, SynthizerError> {
        DirectSource::new(&self)
    }

    pub fn new_panned_source(&mut self) -> Result<PannedSource, SynthizerError> {
        PannedSource::new(&self)
    }
}

impl Deref for Context {
    type Target = syz_Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for Context {}

unsafe impl Sync for Context {}

macro_rules! make_subclass {
    ($subclass:ident, $superclass:ty) => {
        impl $superclass for $subclass {
            fn handle(&self) -> &Handle {
                &self.0
            }
        }
    };
}

pub trait Generator {
    fn handle(&self) -> &Handle;
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
        wrap!(
            unsafe {
                syz_createStreamingGenerator(&mut *handle, **context, protocol, path, options)
            },
            Self(handle)
        )
    }
}

make_subclass!(StreamingGenerator, Generator);

unsafe impl Send for StreamingGenerator {}

unsafe impl Sync for StreamingGenerator {}

pub struct BufferGenerator(Handle);

impl BufferGenerator {
    fn new(context: &Context) -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        wrap!(
            unsafe { syz_createBufferGenerator(&mut *handle, **context) },
            Self(handle)
        )
    }

    pub fn get_buffer(&self) -> Result<Buffer, SynthizerError> {
        let handle = self.handle().get_o(Property::Buffer.to_i32().unwrap())?;
        Ok(Buffer(handle))
    }

    pub fn set_buffer(&self, buffer: Buffer) -> Result<(), SynthizerError> {
        self.handle()
            .set_o(Property::Buffer.to_i32().unwrap(), buffer.0)
    }

    pub fn get_position(&self) -> Result<f64, SynthizerError> {
        let out = self.handle().get_d(Property::Position.to_i32().unwrap())?;
        let out = unsafe { out.as_ref() };
        let out = out.cloned();
        Ok(out.unwrap())
    }

    pub fn set_position(&self, v: f64) -> Result<(), SynthizerError> {
        self.handle().set_d(Property::Position.to_i32().unwrap(), v)
    }

    pub fn get_looping(&self) -> Result<bool, SynthizerError> {
        let v = self.handle().get_i(Property::Looping.to_i32().unwrap())?;
        Ok(v == 1)
    }

    pub fn set_looping(&self, v: bool) -> Result<(), SynthizerError> {
        let v = if v { 1 } else { 0 };
        self.handle().set_i(Property::Looping.to_i32().unwrap(), v)
    }
}

make_subclass!(BufferGenerator, Generator);

unsafe impl Send for BufferGenerator {}

unsafe impl Sync for BufferGenerator {}

pub trait Source {
    fn handle(&self) -> &Handle;

    fn add_generator(&self, generator: &impl Generator) -> Result<(), SynthizerError> {
        wrap!(unsafe { syz_sourceAddGenerator(**self.handle(), **generator.handle()) })
    }

    fn remove_generator(&self, generator: &impl Generator) -> Result<(), SynthizerError> {
        wrap!(unsafe { syz_sourceRemoveGenerator(**self.handle(), **generator.handle()) })
    }

    fn get_gain(&self) -> Result<i32, SynthizerError> {
        self.handle().get_i(Property::Gain.to_i32().unwrap())
    }

    fn set_gain(&self, value: i32) -> Result<(), SynthizerError> {
        self.handle().set_i(Property::Gain.to_i32().unwrap(), value)
    }
}

#[derive(Clone, Debug)]
pub struct DirectSource(Handle);

impl DirectSource {
    fn new(context: &Context) -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        wrap!(
            unsafe { syz_createDirectSource(&mut *handle, **context) },
            Self(handle)
        )
    }
}

make_subclass!(DirectSource, Source);

unsafe impl Send for DirectSource {}

unsafe impl Sync for DirectSource {}

trait SpatializedSource: Source {
    fn get_panner_strategy(&self) -> Result<i32, SynthizerError> {
        self.handle()
            .get_i(Property::PannerStrategy.to_i32().unwrap())
    }

    fn set_panner_strategy(&self, value: i32) -> Result<(), SynthizerError> {
        self.handle()
            .set_i(Property::PannerStrategy.to_i32().unwrap(), value)
    }
}

#[derive(Clone, Debug)]
pub struct PannedSource(Handle);

impl PannedSource {
    fn new(context: &Context) -> Result<Self, SynthizerError> {
        let mut handle = Handle(0);
        wrap!(
            unsafe { syz_createPannedSource(&mut *handle, **context) },
            Self(handle)
        )
    }

    pub fn get_azimuth(&self) -> Result<f64, SynthizerError> {
        let out = self.handle().get_d(Property::Azimuth.to_i32().unwrap())?;
        let out = unsafe { out.as_ref() };
        let out = out.cloned();
        Ok(out.unwrap())
    }

    pub fn set_azimuth(&self, v: f64) -> Result<(), SynthizerError> {
        self.handle().set_d(Property::Azimuth.to_i32().unwrap(), v)
    }

    pub fn get_elevation(&self) -> Result<f64, SynthizerError> {
        let out = self.handle().get_d(Property::Elevation.to_i32().unwrap())?;
        let out = unsafe { out.as_ref() };
        let out = out.cloned();
        Ok(out.unwrap())
    }

    pub fn set_elevation(&self, v: f64) -> Result<(), SynthizerError> {
        self.handle()
            .set_d(Property::Elevation.to_i32().unwrap(), v)
    }

    pub fn get_panning_scalar(&self) -> Result<f64, SynthizerError> {
        let out = self
            .handle()
            .get_d(Property::PanningScalar.to_i32().unwrap())?;
        let out = unsafe { out.as_ref() };
        let out = out.cloned();
        Ok(out.unwrap())
    }

    pub fn set_panning_scalar(&self, v: f64) -> Result<(), SynthizerError> {
        self.handle()
            .set_d(Property::PanningScalar.to_i32().unwrap(), v)
    }
}

make_subclass!(PannedSource, Source);

impl SpatializedSource for PannedSource {}

unsafe impl Send for PannedSource {}

unsafe impl Sync for PannedSource {}

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
