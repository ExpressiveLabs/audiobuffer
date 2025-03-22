use num_traits::Float;
use anyhow::Result;
use rubato::{SincInterpolationParameters, SincInterpolationType, Resampler, SincFixedIn, WindowFunction};
use crate::AudioBuffer;

pub fn resample<T: Float + Clone + rubato::Sample>(buffer: &mut AudioBuffer<T>, source_sr: f32, target_sr: f32) -> Result<()> {
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<T>::new(
        target_sr as f64 / source_sr as f64,
        2.0,
        params,
        buffer.get_num_samples(),
        buffer.get_num_channels(),
    )?;

    buffer.data = resampler.process(&buffer.data, None)?;

    Ok(())
}