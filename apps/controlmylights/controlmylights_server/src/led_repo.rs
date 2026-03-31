use std::sync::atomic::AtomicUsize;

use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

#[derive(Clone, Copy, Default, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Color> for [u8; 3] {
    fn from(value: Color) -> Self { [value.red, value.green, value.blue] }
}

impl Color {
    pub const WHITE: Self = Self {
        red: 255,
        green: 255,
        blue: 255,
    };
}

#[derive(Clone, Copy, Default)]
pub struct Led {
    pub color: Color,
    pub last_updated: DateTime<Utc>,
}

impl From<Led> for [u8; 11] {
    fn from(value: Led) -> Self {
        let color: [u8; 3] = value.color.into();
        let last_updated = value.last_updated.timestamp().to_be_bytes();

        [
            color[0],
            color[1],
            color[2],
            last_updated[0],
            last_updated[1],
            last_updated[2],
            last_updated[3],
            last_updated[4],
            last_updated[5],
            last_updated[6],
            last_updated[7],
        ]
    }
}

pub struct LedSnapshot {
    pub generation: usize,
    pub leds: Vec<Led>,
}

pub struct LedRepo {
    generation: AtomicUsize,
    leds: Mutex<Vec<Led>>,
}

#[derive(thiserror::Error, Debug)]
pub enum LedRepoError {
    #[error("Id {0} is out of bounds")]
    OutOfBounds(usize),
}

impl LedRepo {
    pub fn new(initial_colors: impl Iterator<Item = Color>) -> Self {
        Self {
            generation: AtomicUsize::new(0),
            leds: initial_colors
                .map(|color| Led {
                    color,
                    last_updated: DateTime::<Utc>::MIN_UTC,
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }

    pub async fn get(&self, id: usize) -> Option<Led> { self.leds.lock().await.get(id).copied() }

    pub async fn set(&self, id: usize, color: Color) -> Result<Led, LedRepoError> {
        let mut leds = self.leds.lock().await;
        let led = leds.get_mut(id).ok_or(LedRepoError::OutOfBounds(id))?;
        led.color = color;
        self.generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(*led)
    }

    pub fn generation(&self) -> usize { self.generation.load(std::sync::atomic::Ordering::Acquire) }

    pub async fn snapshot(&self) -> LedSnapshot {
        let leds = self.leds.lock().await;
        LedSnapshot {
            generation: self.generation(),
            leds: leds.clone(),
        }
    }
}
