use conductor::prelude::*;

fn main() {
    let recorder = AudioRecorder::new();
    let player = AudioPlayer::new();
    let fft = FFT::new(512);
    let band_pass = BandpassFilter::new(3000.0, 11_000.0);
    let inverse_fft = InverseFFT::new(512);
    let writer = FileWriter::new("./output.wav");

    recorder.sample_rate.connect(&band_pass.sample_rate);

    recorder.output.connect(&fft.input);
    fft.output.connect(&band_pass.input);
    band_pass.output.connect(&inverse_fft.input);
    inverse_fft.output.connect(&player.input);
    inverse_fft.output.connect(&writer.input);

    pipeline![recorder, player, fft, band_pass, inverse_fft, writer].run();
}
