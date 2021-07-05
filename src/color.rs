use crate::Dwell;
use crate::Fractal;
use crate::Float;
use crate::MAX_DWELL;
use crate::WINDOW_BYTES;

use sdl2::pixels::Color;

use palette::FromColor;
use palette::rgb::Rgb;
use palette::RgbHue;
use palette::Hsl;



pub enum ColorProtocol
{
	Grayscale,
	Hue,
}

pub fn get_grayscale_from_dwell(dwell:u8) -> Color
{
	if dwell == MAX_DWELL
	{
		return Color::RGB(0, 0, 0);
	}
	let dwell_to_byte:u8 = dwell * (255 / MAX_DWELL as u16) as u8;
	return Color::RGB(dwell_to_byte, dwell_to_byte, dwell_to_byte);
}

pub fn get_hue_from_dwell(dwell:u8) -> Color
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

pub fn get_color_from_dwell(fractal:&Fractal<Float>, dwell:u8) -> Color
{
	match fractal.color_protocol
	{
		ColorProtocol::Grayscale => get_grayscale_from_dwell(dwell),
		ColorProtocol::Hue       =>       get_hue_from_dwell(dwell),
	}
}



pub fn colorarray_from_dwellarray(fractal:&Fractal<Float>, dwell_array:Vec<Dwell>) -> Vec<Color>
{
	let mut color_array:Vec<Color>;
	color_array = dwell_array
		.into_iter()
		.map(|dwell| get_color_from_dwell(fractal, dwell.value))
		.collect::<Vec<Color>>();
	return color_array;
}

pub fn dataarray_from_colorarray(fractal:&Fractal<Float>, color_array:Vec<Color>) -> Vec<u8>
//-> [u8; WINDOW_BYTES]
{
//	let mut fractal_data_array:[u8; WINDOW_BYTES] = [0; WINDOW_BYTES];
//	for y in 0..fractal.render_h
//	{
//		for x in 0..fractal.render_w
//		{
//			let pixel_index:usize = (y * fractal.render_w + x) as usize;
//			fractal_data_array[(4 * pixel_index    ) as usize] = color_array[pixel_index].r;
//			fractal_data_array[(4 * pixel_index + 1) as usize] = color_array[pixel_index].g;
//			fractal_data_array[(4 * pixel_index + 2) as usize] = color_array[pixel_index].b;
//			fractal_data_array[(4 * pixel_index + 3) as usize] = color_array[pixel_index].a;
//		}
//	}

//	let fractal_data_array:Vec<u8> = color_array
//		.into_iter()
//		.map(|color| vec![color.r, color.g, color.b, color.a])
//		.collect::<Vec<Vec<u8>>>()
//		.into_iter()
//		.fold
//		(
//			vec![] as Vec<u8>,
//			|acc, next|
//				acc
//					.into_iter()
//					.chain(next.into_iter())
//					.collect::<Vec<u8>>()
//		)
//	;

	let fractal_data_array:Vec<u8> = color_array
		.into_iter()
		.flat_map(|color| vec![color.r, color.g, color.b, color.a])
		.collect::<Vec<u8>>()
	;

	assert_eq!(WINDOW_BYTES, fractal_data_array.len());
	return fractal_data_array;
}