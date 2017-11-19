extern crate libc;

use libc::int16_t;
use libc::c_char;

pub enum SnowboyDetect {}

#[link(name = "snowboy-detect-c-wrapper")]
extern "C" {
    pub fn SnowboyDetectConstructor(
        resource_filename: *const c_char,
        model_str: *const c_char,
    ) -> *mut SnowboyDetect;
    pub fn SnowboyDetectReset(detector: *mut SnowboyDetect);
    pub fn SnowboyDetectRunDetection(
        detector: *mut SnowboyDetect,
        data: *const int16_t,
        array_length: usize,
        is_end: bool,
    ) -> i32;
    pub fn SnowboyDetectSetSensitivity(
        detector: *const SnowboyDetect,
        sensitivity_str: *const c_char,
    ) -> ();
    pub fn SnowboyDetectSetAudioGain(detector: *const SnowboyDetect, audio_gain: f32) -> ();
    pub fn SnowboyDetectSampleRate(detector: *const SnowboyDetect) -> i32;
    pub fn SnowboyDetectBitsPerSample(detector: *const SnowboyDetect) -> i32;
    pub fn SnowboyDetectNumChannels(detector: *const SnowboyDetect) -> i32;
    pub fn SnowboyDetectDestructor(detector: *mut SnowboyDetect) -> ();
}
