use anyhow::{bail, Result};

#[derive(Default)]
pub struct BufferView<'a> {
    data: &'a mut [f32],
    num_channels: usize,
}

impl<'a> BufferView<'a> {
    pub fn new(data: &'a mut [f32], num_channels: usize) -> Self {
        Self { data, num_channels }
    }

    pub fn get_sample(&self, channel: usize, index: usize) -> Result<f32> {
        if channel >= self.num_channels {
            log::error!("[libmikoto::BufferView::get_sample] Channel index out of bounds");
            bail!("channel out of bounds")
        } else if index >= self.data.len() / self.num_channels {
            log::error!("[libmikoto::BufferView::get_sample] Sample index out of bounds");
            bail!("index out of bounds")
        }

        Ok(self.data[index * self.num_channels + channel])
    }

    pub fn sample_mut(&mut self, channel: usize, index: usize) -> Result<&mut f32> {
        if channel >= self.num_channels {
            log::error!("[libmikoto::BufferView::sample_mut] Channel index out of bounds");
            bail!("channel out of bounds")
        } else if index >= self.data.len() / self.num_channels {
            log::error!("[libmikoto::BufferView::sample_mut] Sample index out of bounds");
            bail!("index out of bounds")
        }

        Ok(&mut self.data[index * self.num_channels + channel])
    }

    pub fn add_sample(&mut self, channel: usize, index: usize, value: f32) -> Result<()> {
        if channel >= self.num_channels {
            log::error!("[libmikoto::BufferView::add_sample] Channel index out of bounds");
            bail!("channel out of bounds")
        } else if index >= self.data.len() / self.num_channels {
            log::error!("[libmikoto::BufferView::add_sample] Sample index out of bounds");
            bail!("index out of bounds")
        }

        self.data[index * self.num_channels + channel] += value;

        Ok(())
    }

    pub fn set_sample(&mut self, channel: usize, index: usize, value: f32) -> Result<()> {
        if channel >= self.num_channels {
            log::error!("[libmikoto::BufferView::set_sample] Channel index out of bounds");
            bail!("channel out of bounds")
        } else if index >= self.data.len() / self.num_channels {
            log::error!("[libmikoto::BufferView::set_sample] Sample index out of bounds");
            bail!("index out of bounds")
        }

        self.data[index * self.num_channels + channel] = value;

        Ok(())
    }

    pub fn get_num_samples(&self) -> usize {
        self.data.len() / self.num_channels
    }

    pub fn get_num_channels(&self) -> usize {
        self.num_channels
    }
}