use log::Level;
use synthizer::{Synthizer, SynthizerError};

fn main() -> Result<(), SynthizerError> {
    synthizer::set_log_level(Level::Debug);
    let synthizer = Synthizer::new()?;
    let context = synthizer.new_context()?;
    Ok(())
}
