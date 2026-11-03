// --- Imports ---

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};

// --- Constants ---

const AUDIO_ASSETS_PATH: &str = "projects/01-pong/assets/audio";

// --- Audio Data ---

struct SoundEffect {
    channels: u16,
    sample_rate: u32,
    samples: Vec<f32>,
}

// --- Audio Manager ---

pub struct AudioManager {
    stream: OutputStream,
    music_sink: Option<Sink>,
    sound_effects: HashMap<String, SoundEffect>,
    music_volume: f32,
    sound_effects_volume: f32,
}

// --- Construction and Initialization ---

impl AudioManager {
    pub fn new() -> Self {
        let stream =
            OutputStreamBuilder::open_default_stream().expect("Failed to open audio output");

        let mut manager = Self {
            stream,
            music_sink: None,
            sound_effects: HashMap::new(),
            music_volume: 1.0,
            sound_effects_volume: 1.0,
        };

        manager
            .sound_effects
            .insert("paddle".to_string(), manager.load_sound("paddle.ogg"));

        manager
            .sound_effects
            .insert("wall".to_string(), manager.load_sound("wall.ogg"));

        manager
            .sound_effects
            .insert("brick".to_string(), manager.load_sound("brick.ogg"));

        manager
            .sound_effects
            .insert("game_over".to_string(), manager.load_sound("game_over.ogg"));

        manager
            .sound_effects
            .insert("victory".to_string(), manager.load_sound("victory.ogg"));

        manager
    }

    // --- Asset Loading ---

    fn load_sound(&self, filename: &str) -> SoundEffect {
        let path = format!("{AUDIO_ASSETS_PATH}/{filename}");

        let file =
            File::open(&path).unwrap_or_else(|_| panic!("Failed to open audio file: {path}"));

        let decoder = Decoder::try_from(BufReader::new(file))
            .unwrap_or_else(|_| panic!("Failed to decode audio file: {path}"));

        let sample_rate = decoder.sample_rate();
        let channels = decoder.channels();
        let samples: Vec<f32> = decoder.collect();

        SoundEffect {
            channels,
            sample_rate,
            samples,
        }
    }

    // --- Background Music ---

    pub fn play_music(&mut self) {
        // Avoid starting another music track if one is already active.
        if self.music_sink.is_some() {
            return;
        }

        let path = format!("{AUDIO_ASSETS_PATH}/music.ogg");
        let file = File::open(&path).expect("Failed to open music file");

        let source =
            Decoder::new_looped(BufReader::new(file)).expect("Failed to decode music file");

        let sink = Sink::connect_new(self.stream.mixer());

        sink.set_volume(self.music_volume);
        sink.append(source);

        self.music_sink = Some(sink);
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);

        if let Some(sink) = &self.music_sink {
            sink.set_volume(self.music_volume);
        }
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
        }
    }

    // --- Sound Effects Volume ---

    pub fn set_sound_effects_volume(&mut self, volume: f32) {
        self.sound_effects_volume = volume.clamp(0.0, 1.0);
    }

    // --- Internal Playback ---

    fn play(&self, filename: &str) {
        if let Some(effect) = self.sound_effects.get(filename) {
            let source = rodio::buffer::SamplesBuffer::new(
                effect.channels,
                effect.sample_rate,
                effect.samples.clone(),
            );

            let sink = Sink::connect_new(self.stream.mixer());

            sink.set_volume(self.sound_effects_volume);
            sink.append(source);
            sink.detach();
        }
    }

    // --- Sound Effects ---

    pub fn play_paddle(&self) {
        self.play("paddle");
    }

    pub fn play_wall(&self) {
        self.play("wall");
    }

    pub fn play_brick(&self) {
        self.play("brick");
    }

    pub fn play_game_over(&self) {
        self.play("game_over");
    }

    pub fn play_victory(&self) {
        self.play("victory");
    }
}
