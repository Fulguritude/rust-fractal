//https://users.rust-lang.org/t/rust-sdl2-and-raw-textures-help/45636/7

use palette::FromColor;
use palette::rgb::Rgb;
use palette::RgbHue;
use sdl2::video::WindowContext;
//use sdl2::video::Window;
//use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::surface::Surface;
//use sdl2::surface::SurfaceRef;
//use sdl2::rect::Rect;
//use sdl2::rect::{Point};

use palette::Hsl;

use core::time::Duration;


use polynomials::poly;
use polynomials::Polynomial;

use num_complex::Complex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Float = f32;

//const COMPLEX_NULL:Complex<Float> = Complex { re: 0., im: 0. };
//const COMPLEX_UNIT:Complex<Float> = Complex { re: 1., im: 0. };

const MAX_DWELL:u8 = 32;
const WINDOW_W:u32 = 800;
const WINDOW_H:u32 = 600;
const WINDOW_BYTES:usize = 4 * (WINDOW_W * WINDOW_H) as usize;
const WINDOW_PITCH:usize = 4 * WINDOW_W as usize; //number of bytes in a window row

const COMPLEX_PLANE_CENTER:Complex<Float> = Complex { re: 0., im: 0. };
const COMPLEX_PLANE_STEP:Float = 1.5 / ((WINDOW_W as Float) / 2.);
//const iter_poly:Polynomial<Complex<Float>> =
//	poly!
//	[
//		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
//		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
//		Complex { re: 1., im: 0. }, //COMPLEX_UNIT,
//	];

pub fn get_dwell(z:Complex<Float>/*, poly:Polynomial<Complex<Float>>*/) -> u8
{
	let iter_poly:Polynomial<Complex<Float>> =
	poly!
	[
		z,//Complex::from_polar(0.7885, 1.),//Complex { re: 0.765, im: 0.432 },//z, //COMPLEX_NULL,
		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
		Complex { re: 1., im: 0. }, //COMPLEX_UNIT,
	];

	let mut z_iter:Complex<Float> = z;

	for dwell in 0..MAX_DWELL
	{
		z_iter = iter_poly.eval(z_iter).unwrap();
		if z_iter.norm_sqr() > 2.
		{
			return dwell;
		}
	}
	return MAX_DWELL;
}

pub fn get_color_from_dwell(dwell:u8) -> Color
{
	if dwell == MAX_DWELL
	{
		return Color::RGB(0, 0, 0);
	}
//	let dwell_to_byte:u8 = dwell * (255 / MAX_DWELL as u16) as u8;
//	let dwell_float:Float = (dwell_to_byte as Float) / 255. * 360.;
	let dwell_float:Float = (dwell as Float) / (MAX_DWELL as Float) * 360. - 180.;
	let hsl:Hsl = Hsl::new(RgbHue::from_degrees(dwell_float), 220., 220.);
	let rgb:Rgb = Rgb::from_hsl(hsl);//hsl.into_color();
	return Color::RGB(
		(rgb.red   * 255.) as u8,
		(rgb.green * 255.) as u8,
		(rgb.blue  * 255.) as u8,
	);
}

pub fn get_complex_value_for_pixel(x:u32, y:u32, w:u32, h:u32) -> Complex<Float>
{
	let rel_x:Float = (x as Float) - (w as Float) / 2.;
	let rel_y:Float = (y as Float) - (h as Float) / 2.;

	let offset_from_center =
		Complex
		{
			re: rel_x * COMPLEX_PLANE_STEP,
			im: rel_y * COMPLEX_PLANE_STEP,
		}
	;

	return COMPLEX_PLANE_CENTER + offset_from_center;
}

pub fn main() -> Result<()>
{
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl2 demo", WINDOW_W, WINDOW_H)
		.position_centered()
		.build()
		.unwrap();

	let mut z:Complex<Float> = Complex{ re: 1.5, im: 1.5 };
	println!("Dwell for {} is {}", z.to_string(), get_dwell(z));

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.set_draw_color(Color::RGB(60, 200, 255));
	canvas.clear();
	canvas.present();

//	let mut fractal:Surface = Surface::new(WINDOW_W, WINDOW_H, sdl2::pixels::PixelFormatEnum::Index8)?;
	let mut fractal:[u8; WINDOW_BYTES] = [0; WINDOW_BYTES];

	for y in 0..WINDOW_H
	{
		for x in 0..WINDOW_W
		{
			z = get_complex_value_for_pixel(x,y,WINDOW_W,WINDOW_H);
			let dwell = get_dwell(z);
//println!("Dwell for {} is {}", z, dwell);
			let color = get_color_from_dwell(dwell);
//println!("Color: {} {} {}", color.r, color.g, color.b);
//			fractal.fill_rect(Rect::new(x as i32,y as i32,1,1), color)?;
			fractal[(4 * (y * WINDOW_W + x)    ) as usize] = color.r;
			fractal[(4 * (y * WINDOW_W + x) + 1) as usize] = color.g;
			fractal[(4 * (y * WINDOW_W + x) + 2) as usize] = color.b;
			fractal[(4 * (y * WINDOW_W + x) + 3) as usize] = color.a;
//			canvas.set_draw_color(color);
//			canvas
//				.draw_point(Point::new(x as i32, y as i32))
//				.expect("could not draw point");
		}
	}

//	let mut canvas_surface:Canvas<Surface> = Canvas::from_surface(fractal)?; 
//	let surface_ref:&mut SurfaceRef = Canvas::surface_mut(&mut canvas_surface);

//	let texture_creator = canvas.texture_creator();
//	let mut texture = texture_creator
//		.create_texture_target(texture_creator.default_pixel_format(), 150, 150)
//		.unwrap();
//	canvas.with_texture_canvas(&mut texture, |texture_canvas|
//		{
//		    texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
//		    texture_canvas.clear();
//		    texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
//		    texture_canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();
//		}
//	)?;

	let texture_creator:TextureCreator<WindowContext> = canvas.texture_creator();
	let mut texture:Texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), WINDOW_W, WINDOW_H)?;
//	Texture::update(&mut texture, None, &fractal, WINDOW_PITCH)?;
	texture.update(None, &fractal, WINDOW_PITCH)?;
	canvas.copy(&texture, None, None)?;


	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop
	{
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

	Ok(())
}