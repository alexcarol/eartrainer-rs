extern crate synthrs;

use synthrs::synthesizer::{make_samples, quantize_samples};
use synthrs::wave::{sine_wave, organ, sawtooth_wave, triangle_wave, tangent_wave, bell};
use synthrs::writer::write_wav_file;

fn main() {
    let mut a = make_samples(1.0, 44_100, tangent_wave(440.0));
    let b = make_samples(1.0, 44_100, bell(880.0, 0.003, 0.5));

    a.extend(b);

    write_wav_file(
        "out/twoorgannotes.wav",
        44_100,
        &quantize_samples::<i16>(&a),
    ).expect("failed");
}
