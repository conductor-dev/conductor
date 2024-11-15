use conductor::prelude::*;

fn main() {
    let recorder = AudioRecorder::new();

    let buffer = Buffer::new(false);
    let fft = FFT::new();

    let inverse_fft = InverseFFT::new();
    let sample = Lambdaer::new(|mut x: Vec<f32>| x.pop().unwrap());

    let player = AudioPlayer::new();

    recorder.output.connect(&buffer.input);

    buffer.size.set_initial(10_usize);
    buffer.output.connect(&fft.input);

    fft.output.connect(&inverse_fft.input);

    inverse_fft.output.connect(&sample.input);

    sample.output.connect(&player.input);

    pipeline![recorder, buffer, fft, inverse_fft, sample, player].run();
}
