use conductor::prelude::*;

fn main() {
    let sample_generator = SampleGenerator::<i32>::new(0);
    sample_generator.sample_rate.set_initial(440);
    sample_generator.step.set_initial(1);

    let panic = Lambda::<_, i32>::new(|_| {
        panic!("panic");
    });

    let printer = ConsolePrinter::new();

    sample_generator.output.connect(&panic.input);
    panic.output.connect(&printer.input);

    pipeline![sample_generator, panic, printer].run();
}
