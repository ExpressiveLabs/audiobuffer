use num_traits::Float;

use crate::AudioBuffer;

impl<T: Float + Clone> std::ops::Index<usize> for AudioBuffer<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Float + Clone> std::ops::IndexMut<usize> for AudioBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Float + Clone> std::ops::Index<(usize, usize)> for AudioBuffer<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T: Float + Clone> std::ops::IndexMut<(usize, usize)> for AudioBuffer<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<T: Float + Clone> std::ops::Add for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn add(self, rhs: AudioBuffer<T>) -> AudioBuffer<T> {
        let mut result = self.clone();
        let min_channels = std::cmp::min(self.get_num_channels(), rhs.get_num_channels());
        let min_samples = std::cmp::min(self.get_num_samples(), rhs.get_num_samples());

        for c in 0..min_channels {
            for i in 0..min_samples {
                result.data[c][i] = self.data[c][i] + rhs.data[c][i];
            }
        }

        result
    }
}

impl<T: Float + Clone> std::ops::Add<T> for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn add(self, rhs: T) -> AudioBuffer<T> {
        let mut result = self.clone();

        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                result.data[c][i] = self.data[c][i] + rhs;
            }
        }

        result
    }
}

impl<T: Float + Clone> std::ops::AddAssign for AudioBuffer<T> {
    fn add_assign(&mut self, rhs: AudioBuffer<T>) {
        let min_channels = std::cmp::min(self.get_num_channels(), rhs.get_num_channels());
        let min_samples = std::cmp::min(self.get_num_samples(), rhs.get_num_samples());

        for c in 0..min_channels {
            for i in 0..min_samples {
                self.data[c][i] = self.data[c][i] + rhs.data[c][i];
            }
        }
    }
}

impl<T: Float + Clone> std::ops::AddAssign<T> for AudioBuffer<T> {
    fn add_assign(&mut self, rhs: T) {
        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                self.data[c][i] = self.data[c][i] + rhs;
            }
        }
    }
}

impl<T: Float + Clone> std::ops::Mul<T> for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn mul(self, rhs: T) -> AudioBuffer<T> {
        let mut result = self.clone();

        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                result.data[c][i] = self.data[c][i] * rhs;
            }
        }

        result
    }
}

impl<T: Float + Clone> std::ops::MulAssign<T> for AudioBuffer<T> {
    fn mul_assign(&mut self, rhs: T) {
        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                self.data[c][i] = self.data[c][i] * rhs;
            }
        }
    }
}

impl<T: Float + Clone> std::ops::Div<T> for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn div(self, rhs: T) -> AudioBuffer<T> {
        let mut result = self.clone();

        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                result.data[c][i] = self.data[c][i] / rhs;
            }
        }

        result
    }
}

impl<T: Float + Clone> std::ops::DivAssign<T> for AudioBuffer<T> {
    fn div_assign(&mut self, rhs: T) {
        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                self.data[c][i] = self.data[c][i] / rhs;
            }
        }
    }
}

impl<T: Float + Clone> std::ops::Sub for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn sub(self, rhs: AudioBuffer<T>) -> AudioBuffer<T> {
        let mut result = self.clone();
        let min_channels = std::cmp::min(self.get_num_channels(), rhs.get_num_channels());
        let min_samples = std::cmp::min(self.get_num_samples(), rhs.get_num_samples());

        for c in 0..min_channels {
            for i in 0..min_samples {
                result.data[c][i] = self.data[c][i] - rhs.data[c][i];
            }
        }

        result
    }
}

impl<T: Float + Clone> std::ops::Sub<T> for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn sub(self, rhs: T) -> AudioBuffer<T> {
        let mut result = self.clone();

        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                result.data[c][i] = self.data[c][i] - rhs;
            }
        }

        result
    }
}

impl<T: Float + Clone> std::ops::SubAssign for AudioBuffer<T> {
    fn sub_assign(&mut self, rhs: AudioBuffer<T>) {
        let min_channels = std::cmp::min(self.get_num_channels(), rhs.get_num_channels());
        let min_samples = std::cmp::min(self.get_num_samples(), rhs.get_num_samples());

        for c in 0..min_channels {
            for i in 0..min_samples {
                self.data[c][i] = self.data[c][i] - rhs.data[c][i];
            }
        }
    }
}

impl<T: Float + Clone> std::ops::SubAssign<T> for AudioBuffer<T> {
    fn sub_assign(&mut self, rhs: T) {
        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                self.data[c][i] = self.data[c][i] - rhs;
            }
        }
    }
}

impl<T: Float + Clone> std::ops::Neg for AudioBuffer<T> {
    type Output = AudioBuffer<T>;

    fn neg(self) -> AudioBuffer<T> {
        let mut result = self.clone();

        for c in 0..self.get_num_channels() {
            for i in 0..self.get_num_samples() {
                result.data[c][i] = -self.data[c][i];
            }
        }

        result
    }
}