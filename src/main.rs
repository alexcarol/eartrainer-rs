extern crate synthrs;

use synthrs::synthesizer::{make_samples, quantize_samples};
use synthrs::wave::{tangent_wave, sine_wave, organ, square_wave, triangle_wave, bell};
use synthrs::writer::write_wav_file;
use synthrs::music::note;
use rand::prelude::*;

fn main() {
    let notes = generate_notes();

    let interval_names = vec![
        "Uníson",
        "Segona menor",
        "Segona major",
        "Tercera menor",
        "Tercera major",
        "Quarta justa",
        "Quinta disminuida",
        "Quinta justa",
        "Sexta menor",
        "Sexta major",
        "Sèptima menor",
        "Sèptima major",
        "Octava",
    ];

    let mut intervals: Vec<usize> = vec![];
    for _ in 0..5 {
        intervals.push(rand::random::<usize>() % interval_names.len());
    }

    let mut note_index_pairs: Vec<(usize, usize)> = vec![];

    for interval in intervals {
        println!("interval: {}", interval_names[interval]);

        let lower_note_limit = notes.len() - interval - 1;

        // TODO for now only ascending intervals

        let lower_note_index = rand::random::<usize>() % lower_note_limit;
        let higher_note_index = lower_note_limit + interval;

        note_index_pairs.push((lower_note_index, higher_note_index));
    }

    let note_pairs = note_index_pairs.iter().map(|(i, j)| (notes[*i], notes[*j]));

    let silence  =  make_samples(1.0, 44_100, |_| 0.0);

    let samples = note_pairs.map(|(a, b)| {
        let mut sample_a = make_samples(1.0, 44_100, waveform(a));
        let sample_b = make_samples(1.0, 44_100, waveform(b));

        sample_a.extend(sample_b);

        sample_a.extend(silence.clone());

        sample_a
    });

    write_wav_file(
        "out/notes.wav",
        44_100,
        &quantize_samples::<i16>(&( samples.into_iter().flatten().collect::<Vec<f64>>() )),
    ).expect("failed");
}

fn waveform(a: f64) -> impl Fn(f64) -> f64 {
    bell(a, 0.003, 0.5)
}

fn generate_notes() -> Vec<f64> {
    let mut notes = vec!();
    for i in 3..5 {
        for j in 0..12 {
            notes.push(note(440.0, i, j))
        }
    }

    notes
}
