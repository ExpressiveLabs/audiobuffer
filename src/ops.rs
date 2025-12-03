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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let mut buf1 = AudioBuffer::new_with_size(2, 3);
        buf1.data[0] = vec![1.0, 2.0, 3.0];
        buf1.data[1] = vec![4.0, 5.0, 6.0];

        let mut buf2 = AudioBuffer::new_with_size(2, 3);
        buf2.data[0] = vec![10.0, 20.0, 30.0];
        buf2.data[1] = vec![40.0, 50.0, 60.0];

        // Test adding two buffers
        let result = buf1 + buf2;
        assert_eq!(result.data[0], vec![11.0, 22.0, 33.0]);
        assert_eq!(result.data[1], vec![44.0, 55.0, 66.0]);

        // Test adding scalar
        let result = result + 5.0;
        assert_eq!(result.data[0], vec![16.0, 27.0, 38.0]);
        assert_eq!(result.data[1], vec![49.0, 60.0, 71.0]);


        // Test in-place addition
        let mut buf3 = AudioBuffer::new_with_size(2, 3);
        buf3.data[0] = vec![1.0, 1.0, 1.0];
        buf3.data[1] = vec![2.0, 2.0, 2.0];
        buf3 += buf3.clone();
        assert_eq!(buf3.data[0], vec![2.0, 2.0, 2.0]);
        assert_eq!(buf3.data[1], vec![4.0, 4.0, 4.0]);

        // Test in-place addition with scalar
        buf3 += 2.0;
        assert_eq!(buf3.data[0], vec![4.0, 4.0, 4.0]);
        assert_eq!(buf3.data[1], vec![6.0, 6.0, 6.0]);

        // Test negation
        let neg_buf = -buf3.clone();
        assert_eq!(neg_buf.data[0], vec![-4.0, -4.0, -4.0]);
        assert_eq!(neg_buf.data[1], vec![-6.0, -6.0, -6.0]);

        // Test subtraction
        let sub_buf = buf3.clone() - 2.0;
        assert_eq!(sub_buf.data[0], vec![2.0, 2.0, 2.0]);
        assert_eq!(sub_buf.data[1], vec![4.0, 4.0, 4.0]);

        // Test multiplication
        let mul_buf = buf3.clone() * 2.0;
        assert_eq!(mul_buf.data[0], vec![8.0, 8.0, 8.0]);
        assert_eq!(mul_buf.data[1], vec![12.0, 12.0, 12.0]);

        // Test division
        let div_buf = buf3.clone() / 2.0;
        assert_eq!(div_buf.data[0], vec![2.0, 2.0, 2.0]);
        assert_eq!(div_buf.data[1], vec![3.0, 3.0, 3.0]);
    }
}