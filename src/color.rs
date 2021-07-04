use crate::Float;
use crate::MAX_DWELL;

use sdl2::pixels::Color;

use palette::FromColor;
use palette::rgb::Rgb;
use palette::RgbHue;
use palette::Hsl;



pub fn get_grayscale_from_dwell(dwell:u8) -> Color
{
	if dwell == MAX_DWELL
	{
		return Color::RGB(0, 0, 0);
	}
	let dwell_to_byte:u8 = dwell * (255 / MAX_DWELL as u16) as u8;
	return Color::RGB(dwell_to_byte, dwell_to_byte, dwell_to_byte);
}


pub fn get_hsl_from_dwell(dwell:u8) -> Color
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