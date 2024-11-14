use conductor::prelude::*;

fn main() {
    let recorder = AudioRecorder::new();

    let fft_size = Immediate::new(10_usize);

    let buffer = Buffer::new(false);
    let fft = FFT::new();

    let inverse_fft = InverseFFT::new();
    let sample = Lambdaer::new(|mut x: Vec<f32>| x.pop().unwrap());

    let player = AudioPlayer::new();

    recorder.output.connect(&buffer.input);

    fft_size.output.connect(&buffer.size);

    buffer.output.connect(&fft.input);

    fft.output.connect(&inverse_fft.input);

    inverse_fft.output.connect(&sample.input);

    sample.output.connect(&player.input);

    pipeline![recorder, fft_size, buffer, fft, inverse_fft, sample, player].run();
}
