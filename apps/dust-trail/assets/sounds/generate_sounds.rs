// Simple WAV file generator for procedural sound effects
// Run with: rustc --edition 2021 generate_sounds.rs && ./generate_sounds

use std::fs::File;
use std::io::Write;

fn write_wav(path: &str, samples: &[i16], sample_rate: u32) {
    let mut file = File::create(path).unwrap();
    let num_channels = 1;
    let bits_per_sample = 16;
    let byte_rate = sample_rate * num_channels * bits_per_sample / 8;
    let block_align = num_channels * bits_per_sample / 8;
    let data_size = samples.len() * 2;
    let file_size = 36 + data_size;

    // RIFF header
    file.write_all(b"RIFF").unwrap();
    file.write_all(&(file_size as u32).to_le_bytes()).unwrap();
    file.write_all(b"WAVE").unwrap();

    // fmt chunk
    file.write_all(b"fmt ").unwrap();
    file.write_all(&16u32.to_le_bytes()).unwrap(); // chunk size
    file.write_all(&1u16.to_le_bytes()).unwrap(); // audio format (PCM)
    file.write_all(&num_channels.to_le_bytes()).unwrap();
    file.write_all(&sample_rate.to_le_bytes()).unwrap();
    file.write_all(&byte_rate.to_le_bytes()).unwrap();
    file.write_all(&block_align.to_le_bytes()).unwrap();
    file.write_all(&(bits_per_sample as u16).to_le_bytes()).unwrap();

    // data chunk
    file.write_all(b"data").unwrap();
    file.write_all(&(data_size as u32).to_le_bytes()).unwrap();
    for sample in samples {
        file.write_all(&sample.to_le_bytes()).unwrap();
    }
}

fn generate_tone(freq: f32, duration: f32, sample_rate: u32, volume: f32) -> Vec<i16> {
    let num_samples = (duration * sample_rate as f32) as usize;
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let envelope = if t < 0.01 {
                t / 0.01 // attack
            } else if t > duration - 0.05 {
                (duration - t) / 0.05 // release
            } else {
                1.0
            };
            let sample = (t * freq * 2.0 * std::f32::consts::PI).sin() * volume * envelope;
            (sample * 32767.0) as i16
        })
        .collect()
}

fn generate_noise(duration: f32, sample_rate: u32, volume: f32) -> Vec<i16> {
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut rng = 0x12345678u32;
    (0..num_samples)
        .map(|i| {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let sample = ((rng >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            let envelope = if i < (0.01 * sample_rate as f32) as usize {
                i as f32 / (0.01 * sample_rate as f32)
            } else if i > num_samples - (0.05 * sample_rate as f32) as usize {
                (num_samples - i) as f32 / (0.05 * sample_rate as f32)
            } else {
                1.0
            };
            (sample * volume * envelope * 32767.0) as i16
        })
        .collect()
}

fn generate_click(sample_rate: u32) -> Vec<i16> {
    // Short click/pop sound
    let num_samples = (0.02 * sample_rate as f32) as usize;
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let envelope = (-t * 100.0).exp();
            let sample = (t * 800.0 * 2.0 * std::f32::consts::PI).sin() * envelope;
            (sample * 32767.0 * 0.3) as i16
        })
        .collect()
}

fn main() {
    let sample_rate = 44100;

    // Footstep - low thud
    write_wav("footstep.wav", &generate_tone(80.0, 0.15, sample_rate, 0.5), sample_rate);

    // Loot pickup - bright chime
    write_wav("loot_pickup.wav", &generate_tone(800.0, 0.1, sample_rate, 0.4), sample_rate);

    // Horse mount - neigh-like
    write_wav("horse_mount.wav", &generate_tone(400.0, 0.2, sample_rate, 0.4), sample_rate);

    // Hit - sharp impact
    write_wav("hit.wav", &generate_click(sample_rate), sample_rate);

    // Enemy death - descending tone
    write_wav("enemy_death.wav", &generate_tone(200.0, 0.3, sample_rate, 0.4), sample_rate);

    // Save - soft click
    write_wav("save.wav", &generate_click(sample_rate), sample_rate);

    // Load - double click
    let mut load_samples = generate_click(sample_rate);
    load_samples.extend(generate_click(sample_rate));
    write_wav("load.wav", &load_samples, sample_rate);

    // Game over - descending sad tone
    write_wav("game_over.wav", &generate_tone(150.0, 0.5, sample_rate, 0.4), sample_rate);

    // Victory - ascending arpeggio
    let mut victory = Vec::new();
    for &freq in &[523.0, 659.0, 784.0, 1047.0] {
        victory.extend(generate_tone(freq, 0.15, sample_rate, 0.3));
    }
    write_wav("victory.wav", &victory, sample_rate);

    // Ambient - low wind noise
    write_wav("ambient.wav", &generate_noise(2.0, sample_rate, 0.1), sample_rate);

    println!("Generated all sound files!");
}
