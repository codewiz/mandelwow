use libxm::XMContext;
use sdl2;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::fs::File;
use std::io::Read;


struct XmCallback {
    xm: XMContext,
}

impl AudioCallback for XmCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.xm.generate_samples(out);
    }
}

pub struct SoundPlayer {
    _device: Option<AudioDevice<XmCallback>>,
}

fn play_xm(raw_xm: &[u8]) -> SoundPlayer {
    let sdl_context = sdl2::init().unwrap();
    let sdl_audio = sdl_context.audio().unwrap();

    let sample_rate = 48000u32;
    let desired_spec = AudioSpecDesired {
        freq: Some(sample_rate as i32),
        channels: Some(2u8),
        samples: None,
    };
    let device = sdl_audio.open_playback(None, &desired_spec, |actual_spec| {
        let xm = XMContext::new(&raw_xm, actual_spec.freq as u32).unwrap();

        XmCallback {
            xm: xm,
        }
    }).unwrap();

    device.resume();

    SoundPlayer {
        _device: Some(device),
    }
}

pub fn start() -> SoundPlayer {
    let filename = "flora.xm";
    match File::open(filename) {
        Result::Ok(mut f) => {
            let mut xm = Vec::new();
            f.read_to_end(&mut xm).unwrap();
            return play_xm(&xm);
        },
        Result::Err(err) => {
            println!("Couldn't open module {}: {:?}", filename, err);
        },
    }
    return SoundPlayer { _device: None };
}
