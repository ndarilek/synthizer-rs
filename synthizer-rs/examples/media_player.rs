use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::{env, sync::Mutex};

use log::Level;
use shrust::{Shell, ShellIO};
use synthizer::{
    Buffer, BufferGenerator, LoggingBackend, Protocol, Source, Source3D, Synthizer, SynthizerError,
};

struct Data {
    source: Source3D,
    buffer: Buffer,
    generator: BufferGenerator,
}

fn main() -> Result<(), SynthizerError> {
    let args = env::args().collect::<Vec<String>>();
    let file = args.get(1);
    if let Some(file) = file {
        synthizer::set_log_level(Level::Debug);
        synthizer::configure_logging_backend(LoggingBackend::Stderr)?;
        let synthizer = Synthizer::new()?;
        let mut context = synthizer.new_context()?;
        let path = Path::new(file);
        if path.exists() {
            let buffer = Buffer::new(Protocol::File, path, "")?;
            let generator = context.new_buffer_generator()?;
            generator.set_buffer(buffer.clone())?;
            let source = context.new_source3d()?;
            let data = Data {
                source: source,
                buffer: buffer,
                generator: generator,
            };
            data.source.add_generator(&data.generator)?;
            let mut shell = Shell::new(Arc::new(data));
            shell.new_command_noargs("play", "Play media.", move |io, data| {
                data.source.add_generator(&data.generator)?;
                Ok(())
            });
            shell.new_command_noargs("pause", "Pause media.", |io, data| {
                data.source.remove_generator(&data.generator)?;
                Ok(())
            });
            // Track this here because I'm too lazy to implement `DerefMut` on `Data`.
            let looping = Arc::new(Mutex::new(false));
            shell.new_command_noargs("loop", "Toggle looping.", move |io, data| {
                let looping = looping.clone();
                let mut looping = looping.lock().unwrap();
                *looping = !*looping;
                data.generator.set_looping(*looping)?;
                if *looping {
                    writeln!(io, "Looping")?;
                } else {
                    writeln!(io, "Not looping")?;
                }
                Ok(())
            });
            shell.new_command(
                "gain",
                "Control the gain of the generator, in DB.",
                1,
                |io, data, args| {
                    let gain = args[0].parse::<f64>();
                    if let Some(gain) = gain.ok() {
                        let base: f64 = 10.;
                        let gain = base.powf(gain / 20.);
                        writeln!(io, "Setting gain to {} DB", args[0])?;
                        data.source.set_gain(gain)?;
                    } else {
                        writeln!(io, "{} not a valid gain", args[0])?;
                    }
                    Ok(())
                },
            );
            shell.new_command("seek", "Seek in seconds.", 1, |io, data, args| {
                let position = args[0].parse::<f64>();
                if let Some(position) = position.ok() {
                    writeln!(io, "Seeking to {}", args[0])?;
                    data.generator.set_position(position)?;
                } else {
                    writeln!(io, "{} not a valid position", args[0])?;
                }
                Ok(())
            });
            shell.new_command(
                "pos",
                "Move the source. X is right, Y is forward, Z is up.",
                3,
                |io, data, args| {
                    let x = args[0].parse::<f64>();
                    let y = args[1].parse::<f64>();
                    let z = args[2].parse::<f64>();
                    if let (Some(x), Some(y), Some(z)) = (x.ok(), y.ok(), z.ok()) {
                        writeln!(io, "Moving to ({}, {}, {})", x, y, z)?;
                        data.source.set_position(x, y, z)?;
                    } else {
                        writeln!(io, "{:?} not a valid position", args)?;
                    }
                    Ok(())
                },
            );
            shell.new_command_noargs("quit", "End this madness.", |io, data| {
                std::process::exit(0);
                Ok(())
            });
            shell.run_loop(&mut ShellIO::default());
        } else {
            eprintln!("Path not found");
        }
    } else {
        eprintln!(
            "Usage: {} <path>",
            env::current_exe().unwrap().to_string_lossy()
        );
    }
    Ok(())
}
