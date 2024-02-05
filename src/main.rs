use std::fmt::Display;
use std::time::{Duration, SystemTime};

use beryllium::error::SdlError;
use beryllium::events::Event;
use beryllium::init::InitFlags;
use beryllium::Sdl;
use beryllium::video::{CreateWinArgs, RendererFlags};
use log::{info, LevelFilter, trace};

mod complex;

fn main() -> Result<(), SdlError> {
	simple_logger::init().expect("Failed to setup Logger");
	log::set_max_level(LevelFilter::Debug);

	let sdl = Sdl::init(InitFlags::EVERYTHING);

	for info in sdl.get_renderer_driver_infos().expect("Failed to get render driver info") {
		info!("RendererDriver: {info:#?}");
	}

	let win = sdl.create_renderer_window(
		CreateWinArgs {
			title: "Bruh",
			width: 1000,
			height: 1000,
			allow_high_dpi: true,
			borderless: false,
			resizable: true,
		},
		RendererFlags::ACCELERATED_VSYNC,
	)?;
	info!("Created SDL Window");

	// let pix_buf = [r8g8b8a8_Srgb { r: 255, g: 127, b: 16, a: 255 }; 64];
	// let surface = sdl.create_surface_from(&pix_buf, 8, 8).unwrap();
	// let tex = win.create_texture_from_surface(&surface).unwrap();

	'main_loop: loop {
		#[allow(clippy::single_match)]
		while let Some((event, _timestamp)) = sdl.poll_events() {
			trace!("SDL Event {event:?}");

			match event {
				Event::Quit => break 'main_loop,
				_ => {}
			}
		}
		win.clear()?;

		win.set_draw_color(0, 0, 0, u8::MAX)?;
		win.draw_lines(&[[1, 1], [50, 50], [10, 240]])?;
		win.draw_points(&[[60, 60], [70, 70], [80, 90]])?;
		win.draw_rects(&[[100, 100, 26, 15]])?;
		win.fill_rects(&[[150, 150, 70, 70]])?;

		win.present();
	}

	Ok(())
}
