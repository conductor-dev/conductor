use super::trigger::RisingEdgeTrigger;
use crate::{args::PlotterCommand, error::ConductorSimResult};
use conductor_core::buffer::CircularBuffer;
use egui::{Color32, ScrollArea};
use egui_plot::{Legend, Line, MarkerShape, Plot, PlotPoints, Points};
use std::{
    sync::{Arc, RwLock},
    thread,
};

struct App {
    axes: bool,
    grid: bool,
    sample_markers: bool,
    sample_marker_radius: f32,
    data: Arc<RwLock<CircularBuffer<f32>>>,
}

impl App {
    fn new(data: Arc<RwLock<CircularBuffer<f32>>>) -> Self {
        Self {
            axes: true,
            grid: true,
            sample_markers: false,
            sample_marker_radius: 5.0,
            data,
        }
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.checkbox(&mut self.axes, "Show axes");
            ui.checkbox(&mut self.grid, "Show grid");
            ui.checkbox(&mut self.sample_markers, "Show sample markers");
            ui.add(
                egui::DragValue::new(&mut self.sample_marker_radius)
                    .speed(0.1)
                    .range(0.5..=20.0),
            )
        });

        ui.separator();
    }

    fn _plot_points(&self) -> PlotPoints {
        let data = self.data.read().unwrap();
        PlotPoints::from_iter(
            RisingEdgeTrigger::new(data.iter().map(|v| *v as f64), 0.0) // TODO: make threshold configurable
                .enumerate()
                .map(|(i, v)| [i as f64, v])
                .take(data.len() / 2), // TODO: make this configurable
        )
    }

    fn signal(&self) -> Line {
        Line::new(self._plot_points())
            .color(Color32::LIGHT_RED)
            .name("Signal")
    }

    fn sample_markers(&self) -> Points {
        Points::new(self._plot_points())
            .shape(MarkerShape::Circle)
            .color(Color32::RED)
            .radius(self.sample_marker_radius)
            .name("Sample markers")
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                self.options_ui(ui);
            });

            ui.ctx().request_repaint();

            let plot = Plot::new("Plot")
                .legend(Legend::default())
                .show_axes(self.axes)
                .show_grid(self.grid)
                .x_axis_label("Samples");

            plot.show(ui, |plot_ui| {
                plot_ui.line(self.signal());
                if self.sample_markers {
                    plot_ui.points(self.sample_markers());
                }
            })
        });
    }
}

pub fn udp_listener(
    address: String,
    data: Arc<RwLock<CircularBuffer<f32>>>,
) -> std::io::Result<()> {
    let socket = std::net::UdpSocket::bind(address)?;
    let mut buffer = [0; size_of::<f32>()];

    loop {
        socket.recv(&mut buffer)?;
        data.write().unwrap().push(f32::from_ne_bytes(buffer));
    }
}

pub fn plotter(cmd: PlotterCommand) -> ConductorSimResult<()> {
    let udp_data = Arc::new(RwLock::new(CircularBuffer::new(cmd.buffer_size)));
    let eframe_data = udp_data.clone();

    thread::spawn(move || udp_listener(cmd.bind_address, udp_data));

    eframe::run_native(
        "Plotter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new(eframe_data)))),
    )?;

    Ok(())
}
