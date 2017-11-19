extern crate snowboy_sys;

use std::ffi::CString;
use snowboy_sys::*;


#[derive(Debug, PartialEq, Eq)]
pub enum DetectResult {
    Silence,
    Error,
    NoEvent,
    Hotword(usize),
}

#[derive(Debug, PartialEq)]
pub struct Sensitivities<'a> {
    inner: &'a mut [f32],
}

impl<'a> Sensitivities<'a> {
    pub fn get<'b>(&'b mut self) -> &'b mut [f32] {
        self.inner
    }
}

pub struct SnowboyDetector {
    inner: *mut SnowboyDetect,
    sample_rate: i32,
    num_channels: i32,
    bits_per_sample: i32,
}

impl SnowboyDetector {
    pub fn new(resource: &str, model_fps: &[&str]) -> SnowboyDetector {
        let concated_fp = model_fps.join(",");
        let (inner, sample_rate, num_channels, bits_per_sample) = unsafe {
            let inner = SnowboyDetectConstructor(
                CString::new(resource).unwrap().as_ptr(),
                CString::new(concated_fp).unwrap().as_ptr(),
            );
            (
                inner,
                SnowboyDetectSampleRate(inner),
                SnowboyDetectNumChannels(inner),
                SnowboyDetectBitsPerSample(inner),
            )
        };
        SnowboyDetector {
            inner,
            sample_rate,
            num_channels,
            bits_per_sample,
        }
    }
    pub fn sensitivities<'a>(&'a self) -> Sensitivities<'a> {
        Sensitivities { inner: &mut [] }
    }

    pub fn set_sensitivities<'a>(&mut self, sens: Sensitivities<'a>) {
        self.set_sensitivities_unsafe(sens.inner)
    }

    pub fn set_sensitivities_unsafe(&mut self, sens: &[f32]) {
        let sens_str = sens.iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sens_c = CString::new(sens_str).unwrap();
        unsafe { SnowboyDetectSetSensitivity(self.inner, sens_c.as_ptr()) }
    }
    pub fn sample_rate(&self) -> i32 {
        self.sample_rate
    }
    pub fn num_channels(&self) -> i32 {
        self.num_channels
    }
    pub fn bits_per_sample(&self) -> i32 {
        self.bits_per_sample
    }
    pub fn detect(&mut self, data: &[i16], array_length: usize, is_end: bool) -> DetectResult {
        use DetectResult::*;
        let detect =
            unsafe { SnowboyDetectRunDetection(self.inner, data.as_ptr(), array_length, is_end) };
        match detect {
            -2 => Silence,
            -1 => Error,
            0 => NoEvent,
            x if x > 0 => Hotword(x as usize),
            _ => Error,
        }
    }
}

impl Drop for SnowboyDetector {
    fn drop(&mut self) {
        unsafe { SnowboyDetectDestructor(self.inner) }
    }
}
