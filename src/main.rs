//https://users.rust-lang.org/t/rust-sdl2-and-raw-textures-help/45636/7

mod color;
use color::*;

mod canvas;
use canvas::*;

mod fractal;
use fractal::*;

mod render;
use render::*;

mod event;
use event::*;



use sdl2::video::WindowContext;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::surface::Surface;
//use sdl2::surface::SurfaceRef;
//use sdl2::rect::Rect;
//use sdl2::rect::{Point};


use core::time::Duration;
use time::Instant;

use polynomials::poly;
use polynomials::Polynomial;


mod complex;
use complex::*;

//use num_complex::Complex;



pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Float = f32;

//const COMPLEX_NULL: Complex<Float> = Complex { re: 0., im: 0. };
//const COMPLEX_UNIT: Complex<Float> = Complex { re: 1., im: 0. };

const MAX_DWELL:    u8    = 32;
const WINDOW_W:     usize = 600;
const WINDOW_H:     usize = 600;
const WINDOW_BYTES: usize = 4 * (WINDOW_W * WINDOW_H) as usize;
const WINDOW_PITCH: usize = 4 * WINDOW_W as usize; //number of bytes in a window row

//const COMPLEX_PLANE_STEP:   Float = 1.5 / ((WINDOW_W as Float) / 2.);
const COMPLEX_PLANE_CENTER: Complex<Float> = Complex { re: 0., im: 0. };
const DEFAULT_ZOOM:         Float = 3.;



fn init_fractal() -> Fractal<Float>
{
	let fractal:Fractal<Float> = Fractal
	{
		anchor:             COMPLEX_PLANE_CENTER,
		color_protocol:     ColorProtocol::Hue,
		iteration_protocol: FractalProtocol::Mandelbrot,
		render_protocol:    RenderProtocol::MarianiSilver,
//		radius:             2.,
		radius_sqrd:        4.,
		render_w:           WINDOW_W,
		render_h:           WINDOW_H,
		zoom:				DEFAULT_ZOOM,
		iter_poly:
			poly!
			[
				Complex { re: 1., im: 0.5 }, //COMPLEX_NULL,
				Complex { re: 0., im: 0. }, //COMPLEX_NULL,
				Complex { re: 1., im: 0. }, //COMPLEX_UNIT,
			]
	};


	return fractal;
}


pub fn main() -> Result<()>
{
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl2 demo", WINDOW_W as u32, WINDOW_H as u32)
		.position_centered()
		.build()?;

	let mut canvas:Canvas<Window> = window.into_canvas().build().unwrap();

	canvas.set_draw_color(Color::RGB(60, 200, 255));
	canvas.clear();
	canvas.present();


	let mut fractal_settings:Fractal<Float> = init_fractal();
//	let fractal_data:[u8; WINDOW_BYTES] = render_fractal(fractal_settings);

	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop
	{
		for event in event_pump.poll_iter()
		{
			let should_quit: bool = handle_key_press(event, &mut fractal_settings);
			if should_quit
			{
				break 'running
			}
		}
println!("Loop");

let ini = Instant::now();
		draw_fractal(&mut canvas, &fractal_settings)?;
let end = Instant::now();
println!("{} microseconds", (end - ini).whole_microseconds());
		canvas.present();
		::std::thread::sleep(Duration::new(0, 2_000_000_000u32));
	}

	Ok(())
}