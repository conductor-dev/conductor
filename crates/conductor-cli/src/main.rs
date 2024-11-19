use conductor::prelude::*;

fn main() {
    let recorder = AudioRecorder::new();

    let buffer = Buffer::new(false);

    let hann_window = Window::new(WindowType::Hann);

    let fft = FFT::new();

    let band_pass = BandpassFilter::new(300.0, 3400.0);

    let inverse_fft = InverseFFT::new();
    let sample = Lambdaer::new(|x: Vec<f32>| x[x.len() / 2]);

    let player = AudioPlayer::new();

    recorder.output.connect(&buffer.input);

    buffer.size.set_initial(512_usize);
    buffer.output.connect(&hann_window.input);

    hann_window.output.connect(&fft.input);

    fft.output.connect(&band_pass.input);

    recorder.sample_rate.connect(&band_pass.sample_rate);
    band_pass.output.connect(&inverse_fft.input);

    inverse_fft.output.connect(&sample.input);

    sample.output.connect(&player.input);

    pipeline![
        recorder,
        buffer,
        hann_window,
        fft,
        band_pass,
        inverse_fft,
        sample,
        player
    ]
    .run();
}
