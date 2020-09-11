use std::{env, path::Path};

use log::Level;
use synthizer::{Protocol, Synthizer, SynthizerError};

fn main() -> Result<(), SynthizerError> {
    let args = env::args().collect::<Vec<String>>();
    let file = args.get(1);
    if let Some(file) = file {
        synthizer::set_log_level(Level::Debug);
        let synthizer = Synthizer::new()?;
        let mut context = synthizer.new_context()?;
        let path = Path::new(file);
        if path.exists() {
            let generator = context.new_streaming_generator(Protocol::File, path, "")?;
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
