//! Звуки включения/выключения. Синтезируются в WAV в памяти и играются через
//! WinAPI `PlaySound` асинхронно — без файлов и зависимостей.

use std::{f32::consts::TAU, sync::OnceLock};

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HMODULE,
        Media::Audio::{PlaySoundW, SND_ASYNC, SND_MEMORY, SND_NODEFAULT},
    },
};

const RATE: u32 = 44_100;

#[derive(Clone, Copy)]
pub enum Sound {
    On,
    Off,
}

// Буферы живут всю программу: при SND_ASYNC Windows читает их уже после возврата.
static ON: OnceLock<Vec<u8>> = OnceLock::new();
static OFF: OnceLock<Vec<u8>> = OnceLock::new();

pub fn play(sound: Sound) {
    // Ноты: (частота Гц, начало в секундах)
    let data = match sound {
        // E5 → B5: восходящий "дзинь" — запуск
        Sound::On => ON.get_or_init(|| chime(&[(659.25, 0.0), (987.77, 0.075)])),
        // B5 → E5: нисходящий — остановка
        Sound::Off => OFF.get_or_init(|| chime(&[(987.77, 0.0), (659.25, 0.075)])),
    };
    unsafe {
        let _ = PlaySoundW(
            PCWSTR(data.as_ptr().cast()),
            HMODULE::default(),
            SND_MEMORY | SND_ASYNC | SND_NODEFAULT,
        );
    }
}

/// Мягкий колокольчик: синус + немного обертонов, быстрая атака, экспоненциальный спад.
fn chime(notes: &[(f32, f32)]) -> Vec<u8> {
    const NOTE_LEN: f32 = 0.38;
    const VOLUME: f32 = 0.22;

    let total = notes.iter().map(|&(_, start)| start).fold(0.0, f32::max) + NOTE_LEN;
    let len = (total * RATE as f32) as usize;
    let mut samples = vec![0.0f32; len];

    for &(freq, start) in notes {
        let offset = (start * RATE as f32) as usize;
        for (i, sample) in samples[offset..].iter_mut().enumerate() {
            let t = i as f32 / RATE as f32;
            if t > NOTE_LEN {
                break;
            }
            let attack = (t / 0.004).min(1.0);
            let decay = (-t * 11.0).exp();
            let tone = (TAU * freq * t).sin()
                + 0.35 * (TAU * freq * 2.0 * t).sin() * (-t * 6.0).exp()
                + 0.12 * (TAU * freq * 3.0 * t).sin() * (-t * 20.0).exp();
            *sample += tone * attack * decay;
        }
    }

    let pcm: Vec<i16> = samples
        .iter()
        .map(|s| ((s * VOLUME).clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
        .collect();
    wav(&pcm)
}

/// 16-бит моно PCM WAV.
fn wav(pcm: &[i16]) -> Vec<u8> {
    let data_len = (pcm.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend(b"RIFF");
    out.extend((36 + data_len).to_le_bytes());
    out.extend(b"WAVEfmt ");
    out.extend(16u32.to_le_bytes());
    out.extend(1u16.to_le_bytes()); // PCM
    out.extend(1u16.to_le_bytes()); // моно
    out.extend(RATE.to_le_bytes());
    out.extend((RATE * 2).to_le_bytes()); // байт в секунду
    out.extend(2u16.to_le_bytes()); // выравнивание блока
    out.extend(16u16.to_le_bytes()); // бит на сэмпл
    out.extend(b"data");
    out.extend(data_len.to_le_bytes());
    for s in pcm {
        out.extend(s.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chime_is_valid_wav_without_clipping() {
        let data = chime(&[(659.25, 0.0), (987.77, 0.075)]);
        assert_eq!(&data[0..4], b"RIFF");
        assert_eq!(&data[8..16], b"WAVEfmt ");
        assert_eq!(&data[36..40], b"data");

        let riff_len = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
        assert_eq!(riff_len + 8, data.len());

        let peak = data[44..]
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]).unsigned_abs())
            .max()
            .unwrap();
        // Слышно, но не громко и без клиппинга.
        assert!(peak > 3_000, "слишком тихо: {peak}");
        assert!(peak < 20_000, "слишком громко: {peak}");

        let secs = (data.len() - 44) as f32 / 2.0 / RATE as f32;
        assert!((0.4..0.5).contains(&secs), "длительность {secs}");
    }
}
