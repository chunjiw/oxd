/*! Fetch and play pronunciation audio files */

use crate::models::*;
use reqwest::blocking;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

/// A trait to fetch and play pronunciation audio files
pub trait Pronounce {
    fn pronounce(&self) {}
}

impl Pronounce for Pronunciation {
    fn pronounce(&self) {
        let Some(url) = &self.audio_file else { return };
        let res = blocking::get(url).expect("Expect auido file from url");
        let cursor = Cursor::new(res.bytes().unwrap());
        let decoder = Decoder::new(cursor).unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(decoder);
        sink.sleep_until_end();
    }
}

impl<T: Pronounce> Pronounce for Option<T> {
    fn pronounce(&self) {
        if let Some(p) = &self {
            p.pronounce();
        }
    }
}

impl<T: Pronounce> Pronounce for Vec<T> {
    fn pronounce(&self) {
        for p in self {
            p.pronounce();
        }
    }
}

impl Pronounce for Entry {
    fn pronounce(&self) {
        self.pronunciations.pronounce();
    }
}

impl Pronounce for LexicalEntry {
    fn pronounce(&self) {
        self.entries.pronounce();
    }
}

impl Pronounce for HeadwordEntry {
    fn pronounce(&self) {
        self.lexical_entries.pronounce();
    }
}
