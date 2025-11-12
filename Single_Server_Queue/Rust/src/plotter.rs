use crate::time_series::SimulationTimeSeries;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct InteractivePlotViewer {
    time_series: SimulationTimeSeries,
    plot_states: PlotStates,
}

#[derive(Clone)]
struct PlotStates {
    queue: PlotState,
    wait: PlotState,
    util: PlotState,
    customers: PlotState,
    customers_in_system: PlotState,
    throughput: PlotState,
}

impl PlotStates {
    fn new() -> Self {
        Self {
            queue: PlotState::new(),
            wait: PlotState::new(),
            util: PlotState::new(),
            customers: PlotState::new(),
            customers_in_system: PlotState::new(),
            throughput: PlotState::new(),
        }
    }
}

#[derive(Clone)]
struct PlotState {
    reset_bounds: bool,
    target_bounds: Option<egui_plot::PlotBounds>,
}

impl PlotState {
    fn new() -> Self {
        Self {
            reset_bounds: false,
            target_bounds: None,
        }
    }

    fn zoom_in(&mut self, current_bounds: egui_plot::PlotBounds) {
        let min = current_bounds.min();
        let max = current_bounds.max();

        let center_x = (min[0] + max[0]) / 2.0;
        let center_y = (min[1] + max[1]) / 2.0;
        let width = (max[0] - min[0]) * 0.4;
        let height = (max[1] - min[1]) * 0.4;

        self.target_bounds = Some(egui_plot::PlotBounds::from_min_max(
            [center_x - width, center_y - height],
            [center_x + width, center_y + height],
        ));
    }

    fn zoom_out(&mut self, current_bounds: egui_plot::PlotBounds) {
        let min = current_bounds.min();
        let max = current_bounds.max();

        let center_x = (min[0] + max[0]) / 2.0;
        let center_y = (min[1] + max[1]) / 2.0;
        let width = (max[0] - min[0]) * 0.6;
        let height = (max[1] - min[1]) * 0.6;

        self.target_bounds = Some(egui_plot::PlotBounds::from_min_max(
            [center_x - width, center_y - height],
            [center_x + width, center_y + height],
        ));
    }

    fn reset(&mut self) {
        self.reset_bounds = true;
        self.target_bounds = None;
    }

    fn take_target_bounds(&mut self) -> Option<egui_plot::PlotBounds> {
        self.target_bounds.take()
    }

    fn take_reset(&mut self) -> bool {
        let reset = self.reset_bounds;
        self.reset_bounds = false;
        reset
    }
}

struct ThemeColors {
    frame_fill: egui::Color32,
    frame_stroke: egui::Color32,
    button_fill: egui::Color32,
    button_stroke: egui::Color32,
    text: egui::Color32,
}

impl ThemeColors {
    fn from_visuals(visuals: &egui::Visuals) -> Self {
        let is_dark = visuals.dark_mode;

        if is_dark {
            Self {
                frame_fill: egui::Color32::from_rgb(40, 40, 40),
                frame_stroke: egui::Color32::from_rgb(70, 70, 70),
                button_fill: egui::Color32::from_rgba_unmultiplied(50, 50, 50, 240),
                button_stroke: egui::Color32::from_rgb(80, 80, 80),
                text: egui::Color32::from_rgb(220, 220, 220),
            }
        } else {
            Self {
                frame_fill: egui::Color32::WHITE,
                frame_stroke: egui::Color32::from_rgb(200, 200, 200),
                button_fill: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 240),
                button_stroke: egui::Color32::from_rgb(180, 180, 180),
                text: egui::Color32::from_rgb(40, 40, 40),
            }
        }
    }
}

impl InteractivePlotViewer {
    pub fn new(time_series: SimulationTimeSeries) -> Self {
        Self {
            time_series,
            plot_states: PlotStates::new(),
        }
    }

    pub fn launch(self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1600.0, 1200.0])
                .with_title("Simulation Results - Interactive Viewer"),
            ..Default::default()
        };

        eframe::run_native(
            "Simulation Results",
            options,
            Box::new(|_cc| Ok(Box::new(self))),
        )
    }

    /// Generic plot creation function that handles all data types
    fn create_plot<T, F>(
        ui: &mut egui::Ui,
        plot_id: &str,
        legend_name: &str,
        color: egui::Color32,
        data: &[(f64, T)],
        state: &mut PlotState,
        theme: &ThemeColors,
        to_f64: F,
    ) where
        T: Copy,
        F: Fn(T) -> f64,
    {
        if data.is_empty() {
            return;
        }

        egui::Frame::none()
            .fill(theme.frame_fill)
            .stroke(egui::Stroke::new(1.0, theme.frame_stroke))
            .rounding(5.0)
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.vertical(|ui| {
                    let mut zoom_in_clicked = false;
                    let mut zoom_out_clicked = false;
                    let mut reset_clicked = false;
                    let available_width = ui.available_width();

                    // Create plot with overlay buttons
                    let plot_response = egui::Area::new(egui::Id::new(plot_id).with("_area"))
                        .fixed_pos(ui.cursor().min)
                        .show(ui.ctx(), |ui| {
                            let mut plot = Plot::new(plot_id)
                                .legend(
                                    egui_plot::Legend::default()
                                        .position(egui_plot::Corner::LeftTop),
                                )
                                .height(260.0)
                                .width(available_width)
                                .allow_zoom(true)
                                .allow_drag(true)
                                .allow_scroll(true);

                            if state.take_reset() {
                                plot = plot.auto_bounds(egui::Vec2b::TRUE);
                            }

                            let plot_response = plot.show(ui, |plot_ui| {
                                if let Some(bounds) = state.take_target_bounds() {
                                    plot_ui.set_plot_bounds(bounds);
                                }

                                let points: PlotPoints =
                                    data.iter().map(|(t, v)| [*t, to_f64(*v)]).collect();
                                plot_ui.line(Line::new(points).color(color).name(legend_name));

                                plot_ui.plot_bounds()
                            });

                            (plot_response.inner, plot_response.response.rect)
                        });

                    let (current_bounds, plot_rect) = plot_response.inner;
                    ui.allocate_space(egui::vec2(available_width, 260.0));

                    // Overlay control buttons
                    let button_pos = egui::pos2(plot_rect.right() - 135.0, plot_rect.top() + 8.0);

                    egui::Area::new(egui::Id::new(plot_id).with("_controls"))
                        .fixed_pos(button_pos)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::none()
                                .fill(theme.button_fill)
                                .stroke(egui::Stroke::new(1.0, theme.button_stroke))
                                .rounding(4.0)
                                .inner_margin(egui::Margin::same(5.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.0;

                                        if ui
                                            .button(egui::RichText::new("🔍+").size(13.0))
                                            .clicked()
                                        {
                                            zoom_in_clicked = true;
                                        }
                                        if ui
                                            .button(egui::RichText::new("🔍−").size(13.0))
                                            .clicked()
                                        {
                                            zoom_out_clicked = true;
                                        }
                                        if ui
                                            .button(egui::RichText::new("Reset").size(13.0))
                                            .clicked()
                                        {
                                            reset_clicked = true;
                                        }
                                    });
                                });
                        });

                    // Handle button clicks
                    if zoom_in_clicked {
                        state.zoom_in(current_bounds);
                        ui.ctx().request_repaint();
                    }
                    if zoom_out_clicked {
                        state.zoom_out(current_bounds);
                        ui.ctx().request_repaint();
                    }
                    if reset_clicked {
                        state.reset();
                        ui.ctx().request_repaint();
                    }
                });
            });
    }

    fn plot_queue_length(&mut self, ui: &mut egui::Ui, theme: &ThemeColors) {
        let data = self.time_series.queue_length.data();
        Self::create_plot(
            ui,
            "queue_length",
            "Queue Length Over Time",
            egui::Color32::BLUE,
            data,
            &mut self.plot_states.queue,
            theme,
            |v| v as f64,
        );
    }

    fn plot_mean_wait_time(&mut self, ui: &mut egui::Ui, theme: &ThemeColors) {
        let data = self.time_series.mean_wait_time.data();
        Self::create_plot(
            ui,
            "mean_wait_time",
            "Mean Wait Time Over Time",
            egui::Color32::RED,
            data,
            &mut self.plot_states.wait,
            theme,
            |v| v,
        );
    }

    fn plot_utilization(&mut self, ui: &mut egui::Ui, theme: &ThemeColors) {
        let data = self.time_series.utilization.data();
        Self::create_plot(
            ui,
            "utilization",
            "Server Utilization Over Time (0-1)",
            egui::Color32::GREEN,
            data,
            &mut self.plot_states.util,
            theme,
            |v| v,
        );
    }

    fn plot_customers_served(&mut self, ui: &mut egui::Ui, theme: &ThemeColors) {
        let data = self.time_series.customers_served.data();
        Self::create_plot(
            ui,
            "customers_served",
            "Customers Served Over Time",
            egui::Color32::from_rgb(128, 0, 128),
            data,
            &mut self.plot_states.customers,
            theme,
            |v| v as f64,
        );
    }

    fn plot_customers_in_system(&mut self, ui: &mut egui::Ui, theme: &ThemeColors) {
        let data = self.time_series.customers_in_system.data();
        Self::create_plot(
            ui,
            "customers_in_system",
            "Customers in System Over Time",
            egui::Color32::from_rgb(255, 140, 0),
            data,
            &mut self.plot_states.customers_in_system,
            theme,
            |v| v as f64,
        );
    }

    fn plot_throughput(&mut self, ui: &mut egui::Ui, theme: &ThemeColors) {
        let data = self.time_series.throughput.data();
        Self::create_plot(
            ui,
            "throughput",
            "System Throughput (customers/time)",
            egui::Color32::from_rgb(0, 128, 128),
            data,
            &mut self.plot_states.throughput,
            theme,
            |v| v,
        );
    }
}

impl eframe::App for InteractivePlotViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ThemeColors::from_visuals(&ctx.style().visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("SSQ Simulation")
                        .size(28.0)
                        .strong()
                        .color(theme.text),
                );
                ui.add_space(10.0);
            });

            ui.separator();
            ui.add_space(10.0);

            // 3x2 Grid layout for all 6 plots
            let available_width = ui.available_width();
            let plot_width = (available_width - 30.0) / 2.0;

            egui::Grid::new("plot_grid")
                .spacing([15.0, 15.0])
                .min_col_width(plot_width)
                .max_col_width(plot_width)
                .show(ui, |ui| {
                    self.plot_queue_length(ui, &theme);
                    self.plot_mean_wait_time(ui, &theme);
                    ui.end_row();

                    self.plot_customers_in_system(ui, &theme);
                    self.plot_utilization(ui, &theme);
                    ui.end_row();

                    self.plot_throughput(ui, &theme);
                    self.plot_customers_served(ui, &theme);
                    ui.end_row();
                });
        });
    }
}
