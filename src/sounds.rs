use std::io::Cursor;

use rodio;

use errors::*;

const MODE_ACTIVATED: &'static [u8] = include_bytes!("../sounds/mode_activated.wav");
const MODE_DEACTIVATED: &'static [u8] = include_bytes!("../sounds/mode_deactivated.wav");

pub fn play_mode_activated() -> Result<()> {
    play_audio(MODE_ACTIVATED)
}

pub fn play_mode_deactivated() -> Result<()> {
    play_audio(MODE_DEACTIVATED)
}

fn play_audio<R>(r: R) -> Result<()>
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
    #[ignore]
    fn play_audio_plays_audio() {
        play_mode_activated().unwrap();
        play_mode_deactivated().unwrap();
    }
}
