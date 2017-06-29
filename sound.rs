use libxm::XMContext;
use sdl2;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::fs::File;
use std::io::Read;


const SAMPLE_RATE: i32 = 48000;

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
    device: Option<AudioDevice<XmCallback>>,
}

fn play_xm(raw_xm: &[u8]) -> SoundPlayer {
    let sdl_context = sdl2::init().unwrap();
    let sdl_audio = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(2),
        samples: Some(4096),  // 85ms
    };
    let device = sdl_audio.open_playback(None, &desired_spec, |actual_spec| {
        let xm = XMContext::new(raw_xm, actual_spec.freq as u32).unwrap();

        XmCallback {
            xm: xm,
        }
    }).unwrap();

    device.resume();

    SoundPlayer {
        device: Some(device),
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
    SoundPlayer { device: None }
}

pub fn hit_event(player: &mut SoundPlayer) -> f32 {
    use std::ops::Deref;
    let audio_device_lock = player.device.as_mut().unwrap().lock();
    let xm_callback = audio_device_lock.deref();
    let xm = &xm_callback.xm;
    let n_samples = xm.latest_trigger_of_instrument(0x1D);
    n_samples as f32 / SAMPLE_RATE as f32
}
