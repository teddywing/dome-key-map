use std::io::Cursor;

use rodio;

use errors::*;

// const MODE_ACTIVATED = include_bytes!("../sounds/mode_activated.ogg");
// const MODE_DEACTIVATED = include_bytes!("../sounds/mode_deactivated.ogg");

pub const MODE_ACTIVATED: &'static [u8] = include_bytes!("../sounds/activ.wav");

pub fn play_audio<R>(r: R) -> Result<()>
where R: AsRef<[u8]> + Send + 'static {
    let device = rodio::default_output_device()
        .chain_err(|| "could not find an audio output device")?;

    let reader = Cursor::new(r);
    let sink = rodio::play_once(&device, reader)
        .chain_err(|| "error playing audio")?;

    sink.sleep_until_end();
    sink.play();

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_audio_plays_audio() {
        play_audio(MODE_ACTIVATED).unwrap();
    }
}
