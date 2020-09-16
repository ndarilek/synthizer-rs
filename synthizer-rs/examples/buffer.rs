use std::{env, io, path::Path};

use log::Level;
use synthizer::{Buffer, LoggingBackend, Protocol, Source, Synthizer, SynthizerError};

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
            generator.set_looping(true)?;
            generator.set_buffer(buffer)?;
            let source = context.new_direct_source()?;
            source.add_generator(&generator)?;
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
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
