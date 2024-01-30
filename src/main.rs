use eframe::{App, CreationContext, Frame};
use egui::{Context, Vec2, ViewportBuilder};

const HEIGHT: usize = 32;
const WIDTH: usize = 64;

#[derive(Clone, Copy, Debug, Default)]
struct Tile {}

#[derive(Debug)]
struct Gui {
	grid: [[Option<Tile>; WIDTH]; HEIGHT],
}

impl Gui {
	fn new(_cc: &CreationContext) -> Self {
		Gui {
			grid: [[None; WIDTH]; HEIGHT],
		}
	}
}

impl App for Gui {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				egui::Grid::new(0).num_columns(WIDTH).show(ui, |ui| {
					ui.columns(WIDTH, |cols| {
						for col in cols.iter_mut() {
							col.label("Hi");
						}
					})
				});
				ui.label("Hello!");
			});
		});
	}
}

fn main() {
	std::env::remove_var("WAYLAND_DISPLAY");
	eframe::run_native(
		"GBA Hexgrid editor",
		eframe::NativeOptions {
			default_theme: eframe::Theme::Light,
			viewport: ViewportBuilder::default().with_inner_size(Vec2 {
				x: (WIDTH * 20) as _,
				y: (HEIGHT * 20) as _,
			}),
			..Default::default()
		},
		Box::new(move |cc| Box::new(Gui::new(cc))),
	)
	.unwrap();
}
