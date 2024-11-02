pub struct RisingEdgeTrigger<I> {
    iter: I,
    prev_value: Option<f64>,
    triggered: bool,
}

impl<I> RisingEdgeTrigger<I>
where
    I: Iterator<Item = f64>,
{
    pub fn new(iter: I) -> Self {
        RisingEdgeTrigger {
            iter,
            prev_value: None,
            triggered: false,
        }
    }
}

impl<I> Iterator for RisingEdgeTrigger<I>
where
    I: Iterator<Item = f64>,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.triggered {
            return self.iter.next();
        }

        // TODO: make this configurable
        let threshold = 0.0;

        while let Some(current_value) = self.iter.next() {
            if let Some(prev_value) = self.prev_value {
                if current_value > threshold && prev_value <= threshold {
                    self.triggered = true;
                    self.prev_value = Some(current_value);
                    return Some(current_value);
                }
            }
            self.prev_value = Some(current_value);
        }

        None
    }
}
