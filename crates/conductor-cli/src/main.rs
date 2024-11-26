use conductor::prelude::*;

fn main() {
    let sample_generator = SampleGenerator::<i32>::new();
    sample_generator.sample_rate.set_initial(1);

    let less_than_equal = LessThanEqual::new();
    less_than_equal.input2.set_initial(100);

    let synchronize_gate = Synchronize::new();

    let gate = Gate::new();
    gate.condition.set_lazy(true);

    let addition = Adder::<i32, i32, i32>::new();
    addition.input2.set_initial(0);
    addition.input2.set_lazy(true);

    let console = ConsolePrinter::new();

    sample_generator.output.connect(&synchronize_gate.input2);
    sample_generator.output.connect(&less_than_equal.input1);

    less_than_equal.output.connect(&synchronize_gate.input1);

    synchronize_gate.output1.connect(&gate.condition);
    synchronize_gate.output2.connect(&gate.input);

    gate.output.connect(&addition.input1);

    addition.output.connect(&addition.input2);
    addition.output.connect(&console.input);

    pipeline![
        sample_generator,
        less_than_equal,
        synchronize_gate,
        gate,
        addition,
        console
    ]
    .run();
}
