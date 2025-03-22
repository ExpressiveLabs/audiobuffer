# `audiobuffer`
A simple audio buffer library written in Rust. The syntax is vaguely inspired by JUCE's audio buffer. This library is used in all Mikoto platform products developed by ExpressiveLabs.

## Installation
This library is available on [crates.io](https://crates.io/crates/audiobuffer). To install it:
```
cargo add audiobuffer
```

## How to use
```rust
    // Create an empty buffer
    let mut buffer = AudioBuffer::<f32>::new();
    buffer.set_size(2, 44100);

    // Fill it with silence
    buffer.silence();

    // Set internal samplerate, with resampling support
    buffer.set_samplerate(48000, true);

    // Wrap existing data to avoid allocations
    let mut data = [0.0, 1.0, 1.0, 0.0, 0.32];
    let mut buffer_view = BufferView::new(&mut data, 1);

    buffer_view.set_sample(0, 0, 0.5);
```

## License
This library is licensed under the MIT license.<br>
Copyright &copy; 2025 ExpressiveLabs.