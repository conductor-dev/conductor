use conductor::prelude::*;

fn main() {
    let sample_generator = SampleGenerator::<i32>::new();
    sample_generator.sample_rate.set_initial(1);

    let less_than_equal = LessThanEqual::new();
    less_than_equal.input2.set_initial(5);

    let gate = SynchronizedGate::new();

    let console = ConsolePrinter::new();

    sample_generator.output.connect(&gate.input);
    sample_generator.output.connect(&less_than_equal.input1);

    less_than_equal.output.connect(&gate.condition);

    gate.output.connect(&console.input);

    pipeline![sample_generator, less_than_equal, gate, console].run();
}
