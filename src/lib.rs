pub mod view;
pub mod ops;

#[cfg(feature = "resample")]
pub mod resample;

use abi_stable::{StableAbi, rvec, std_types::RVec};

#[allow(unused)]
pub use ops::*;

use num_traits::Float;
use anyhow::{bail, Result};

#[derive(Default, Debug, Clone, StableAbi)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
/// flutter_rust_bridge:ignore
pub struct AudioBuffer<T: Float + Clone + StableAbi> {
    pub data: RVec<RVec<T>>,
    pub sample_rate: f32,
}

impl<T> AudioBuffer<T> where T: Float + Clone + std::ops::AddAssign<T> + StableAbi {
    pub fn add_sample(&mut self, channel: usize, index: usize, value: T) -> Result<()> {
        if channel >= self.data.len() {
            log::error!("[libmikoto::AudioBuffer::add_sample] Channel index out of bounds");
            bail!("Channel index out of bounds");
        } else if index >= self.data[channel].len() {
            log::error!("[libmikoto::AudioBuffer::add_sample] Sample index out of bounds");
            bail!("Sample index out of bounds");
        }

        self.data[channel][index] += value;
        Ok(())
    }

    pub fn merge_channels(&mut self) {
        let mut new_data = RVec::new();
        let div = T::from(self.get_num_channels()).unwrap();

        for i in 0..self.get_num_samples() {
            let mut sample = T::zero();
            for c in 0..self.get_num_channels() {
                sample += self.get_sample(c, i).unwrap() / div;
            }
            new_data.push(sample);
        }

        self.data = rvec![new_data];
    }
}

impl<T: Float + Clone + StableAbi> AudioBuffer<T> {
    pub fn new() -> Self {
        Self { data: RVec::new(), sample_rate: 44100.0 }
    }
    pub fn new_with_sample_rate(sample_rate: f32) -> Self {
        Self { data: RVec::new(), sample_rate }
    }
    pub fn new_with_properties(channels: usize, length: usize, sample_rate: f32) -> Self {
        let mut data = RVec::new();
        for _i in 0..channels {
            data.push(rvec![T::zero(); length]);
        }
        Self { data, sample_rate }
    }

    pub fn new_with_size(channels: usize, length: usize) -> Self {
        let mut data = RVec::new();
        for _i in 0..channels {
            data.push(rvec![T::zero(); length]);
        }
        Self { data, sample_rate: 44100.0 }
    }

    pub fn set_size(&mut self, channels: usize, length: usize, clear: bool) {
        if clear {
            self.data.clear();

            for _i in 0..channels {
                self.data.push(rvec![T::zero(); length]);
            }
        } else {
            for i in 0..channels {
                if i >= self.data.len() {
                    self.data.push(rvec![T::zero(); length]);
                } else {
                    self.data[i].resize(length, T::zero());
                }
            }
        }
    }

    pub fn get_num_samples(&self) -> usize {
        if self.data.is_empty() {
            log::warn!("[libmikoto::AudioBuffer::get_num_samples] No channels in buffer");
            0
        } else {
            self.data[0].len()
        }
    }
    pub fn get_num_channels(&self) -> usize {
        self.data.len()
    }
    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn get_sample(&self, channel: usize, index: usize) -> Result<T> {
        if channel >= self.data.len() {
            log::error!("[libmikoto::AudioBuffer::get_sample] Channel index out of bounds");
            bail!("Channel index out of bounds");
        } else if index >= self.data[channel].len() {
            log::error!("[libmikoto::AudioBuffer::get_sample] Sample index out of bounds");
            bail!("Sample index out of bounds");
        }

        Ok(self.data[channel][index].clone())
    }

    pub fn set_sample(&mut self, channel: usize, index: usize, value: T) -> Result<()> {
        if channel >= self.data.len() {
            log::error!("[libmikoto::AudioBuffer::set_sample] Channel index out of bounds");
            bail!("Channel index out of bounds");
        } else if index >= self.data[channel].len() {
            log::error!("[libmikoto::AudioBuffer::set_sample] Sample index out of bounds");
            bail!("Sample index out of bounds");
        }

        self.data[channel][index] = value;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn clear_channels(&mut self) {
        for i in 0..self.data.len() {
            self.data[i].clear();
        }
    }

    pub fn silence(&mut self) {
        let zero = T::zero();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = zero;
            }
        }
    }

    pub fn copy_from_with_typecast<U: Float + Clone + StableAbi>(&mut self, source: &AudioBuffer<U>, source_start: usize, dest_start: usize, length: usize) -> Result<()> {
        if source.get_num_channels() != self.get_num_channels() {
            log::error!("[libmikoto::AudioBuffer::copy_from_with_typecast] Channel count mismatch");
            bail!("Channel count mismatch");
        } else if dest_start + length > self.get_num_samples() {
            log::error!("[libmikoto::AudioBuffer::copy_from_with_typecast] Sample index outside of target buffer bounds");
            bail!("Sample index outside of target buffer bounds");
        } else if source_start + length > source.get_num_samples() {
            log::error!("[libmikoto::AudioBuffer::copy_from_with_typecast] Sample index outside of source buffer bounds");
            bail!("Sample index outside of source buffer bounds");
        }

        for c in 0..source.get_num_channels() {
            for i in 0..length {
                let smp = source.get_sample(c, source_start + i).unwrap();
                let smp = T::from(smp).unwrap();

                self.set_sample(c, dest_start + i, smp).unwrap();
            }
        }

        Ok(())
    }

    pub fn copy_from(&mut self, source: &AudioBuffer<T>, source_start: usize, dest_start: usize, length: usize) -> Result<()> {
        if source.get_num_channels() != self.get_num_channels() {
            log::error!("[libmikoto::AudioBuffer::copy_from] Channel count mismatch");
            bail!("Channel count mismatch");
        } else if dest_start + length > self.get_num_samples() {
            log::error!("[libmikoto::AudioBuffer::copy_from] Sample index outside of target buffer bounds");
            bail!("Sample index outside of target buffer bounds");
        } else if source_start + length > source.get_num_samples() {
            log::error!("[libmikoto::AudioBuffer::copy_from] Sample index outside of source buffer bounds");
            bail!("Sample index outside of source buffer bounds");
        }

        for c in 0..source.get_num_channels() {
            for i in 0..length {
                self.set_sample(c, dest_start + i, source.get_sample(c, source_start + i).unwrap()).unwrap();
            }
        }

        Ok(())
    }
}

#[cfg(feature = "resample")]
impl<T: Float + Clone + rubato::Sample + StableAbi> AudioBuffer<T> {
    pub fn set_sample_rate(&mut self, sample_rate: f32, resample: bool) -> Result<()> {
        if self.sample_rate == sample_rate {
            return Ok(());
        }

        if resample {
            resample::resample(self, self.sample_rate, sample_rate)?;
        }
        self.sample_rate = sample_rate;

        Ok(())
    }
}