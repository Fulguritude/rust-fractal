use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

use core::time::Duration;

use sdl2::rect::{Point};

use polynomials::poly;
use polynomials::Polynomial;

use num_complex::Complex;

const COMPLEX_NULL:Complex<f64> = Complex { re: 0., im: 0. };
const COMPLEX_UNIT:Complex<f64> = Complex { re: 1., im: 0. };

const max_dwell:u8 = 255;
const window_w:u32 = 200;
const window_h:u32 = 150;

const complex_plane_center:Complex<f64> = COMPLEX_NULL;
const complex_plane_step:f64 = 1.5 / ((window_w as f64) / 2.);
//const iter_poly:Polynomial<Complex<f64>> =
//	poly!
//	[
//		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
//		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
//		Complex { re: 1., im: 0. }, //COMPLEX_UNIT,
//	];

pub fn get_dwell(z:Complex<f64>/*, poly:Polynomial<Complex<f64>>*/) -> u8
{
	let iter_poly:Polynomial<Complex<f64>> =
	poly!
	[
		z, //COMPLEX_NULL,
		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
		Complex { re: 1., im: 0. }, //COMPLEX_UNIT,
	];

	let mut z_iter:Complex<f64> = z;

	for dwell in 0..max_dwell
	{
		z_iter = iter_poly.eval(z_iter).unwrap();
		if z_iter.norm_sqr() > 2.
		{
			return dwell;
		}
	}
	return max_dwell;
}

pub fn get_color_from_dwell(dwell:u8) -> Color
{
	return Color::RGB(dwell, dwell, dwell);
}

pub fn get_complex_value_for_pixel(x:u32, y:u32, w:u32, h:u32) -> Complex<f64>
{
	let rel_x:f64 = (x as f64) - (w as f64) / 2.;
	let rel_y:f64 = (y as f64) - (h as f64) / 2.;

	let offset_from_center =
		Complex
		{
			re: rel_x * complex_plane_step,
			im: rel_y * complex_plane_step,
		}
	;

	return complex_plane_center + offset_from_center;
}

pub fn main()
{
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl2 demo", window_w, window_h)
		.position_centered()
		.build()
		.unwrap();

	let mut z:Complex<f64> = Complex{ re: 1.5, im: 1.5};
	println!("Dwell for {} is {}", z.to_string(), get_dwell(z));

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.set_draw_color(Color::RGB(255, 255, 255));
	canvas.clear();
	canvas.present();

	let fractal:Surface = Surface::new(window_w, window_h, sdl2::pixels::PixelFormatEnum::Index8).unwrap();

	

	for y in 0..window_h
	{
		for x in 0..window_w
		{
			z = get_complex_value_for_pixel(x,y,window_w,window_h);
			let dwell = get_dwell(z);
			println!("Dwell for {} is {}", z, dwell);
			let color = get_color_from_dwell(dwell);
//			println!("Color: {} {} {}", color.r, color.g, color.b);
			canvas.set_draw_color(color);
			canvas
				.draw_point(Point::new(x as i32, y as i32))
				.expect("could not draw point");
//			canvas.pixel(x, y, color);
		}
	}


	let mut event_pump = sdl_context.event_pump().unwrap();
//	let mut i = 0;
	'running: loop
	{
//		i = (i + 1) % 255;
//		canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//		canvas.clear();
		for event in event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					break 'running
				},
				_ => {}
			}
		}
		// The rest of the game loop goes here...

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}