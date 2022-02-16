use std::f32::consts::PI;
use std::io::{Write, Seek};
use hound::WavWriter;

const SAMPLE_RATE: f32 = 44100.0;
const BPM: f32 = 140.0;

#[allow(unused_variables)]
fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut fw = WavWriter::create("melody.wav", spec).unwrap();

    let (c4, d4, e4, f4) = (261.626, 293.665, 329.628, 349.228);
    let (g4, a4, b4, c5) = (391.995, 440.000, 493.883, 523.251);
    let non_sounds = 0.0;

    let l4 = ((60.0 / BPM) * SAMPLE_RATE) as u32;
    let l2 = l4 * 2;

    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, e4, l4); // ミ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, f4, l4); // ファ
    write_tone(&mut fw, e4, l4); // ミ
    write_tone(&mut fw, f4, l2); // ファ
    write_tone(&mut fw, non_sounds, l4); // 休
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, c5, l4); // ♯ド
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, g4, l2); // ソ
    write_tone(&mut fw, non_sounds, l4); // 休
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, e4, l4); // ミ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, f4, l4); // ファ
    write_tone(&mut fw, e4, l4); // ミ
    write_tone(&mut fw, f4, l2); // ファ
    write_tone(&mut fw, non_sounds, l4); // 休
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, g4, l4); // ソ
    write_tone(&mut fw, a4, l4); // ラ
    write_tone(&mut fw, b4, l4); // シ
    write_tone(&mut fw, c5, l2); // ♯ド
    write_tone(&mut fw, non_sounds, l4); // 休
}

fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32)
where W: Write + Seek {
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE;
        let v = (a * tone * 2.0 * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}
