use conductor::prelude::*;

fn main() {
    let sample_generator = SampleGenerator::<i32>::new(0);
    sample_generator.sample_rate.set_initial(4400000);
    sample_generator.step.set_initial(1);

    let less_than_equal = LessThanEqual::new();
    less_than_equal.input2.set_initial(1000);

    let synchronize_gate = Synchronize::new();

    let gate = Gate::new();
    gate.condition.set_kind(PortKind::LazyBuffer);

    let synchronize_addition = Synchronize::new();
    synchronize_addition.input1.set_initial(0);

    let addition = AddNode::new();
    addition.input1.set_kind(PortKind::LazyBuffer);

    let console = ConsolePrinter::new();

    sample_generator.output.connect(&synchronize_gate.input2);
    sample_generator.output.connect(&less_than_equal.input1);

    less_than_equal.output.connect(&synchronize_gate.input1);

    synchronize_gate.output1.connect(&gate.condition);
    synchronize_gate.output2.connect(&gate.input);

    gate.output.connect(&synchronize_addition.input2);

    synchronize_addition.output1.connect(&addition.input1);
    synchronize_addition.output2.connect(&addition.input2);

    addition.output.connect(&synchronize_addition.input1);
    addition.output.connect(&console.input);

    pipeline![
        sample_generator,
        less_than_equal,
        synchronize_gate,
        gate,
        synchronize_addition,
        addition,
        console
    ]
    .run();
}
