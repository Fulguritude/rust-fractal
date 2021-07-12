use crate::Fractal;
use crate::Dwell;
use crate::DwellState;
use crate::DwellArray;
use crate::Float;
use crate::MAX_DWELL;
use crate::WINDOW_BYTES;

use contracts::ensures;

use sdl2::pixels::Color;

use palette::FromColor;
use palette::rgb::Rgb;
use palette::RgbHue;
use palette::Hsl;



pub enum ColorProtocol
{
	Grayscale,
	Hue,
	MarianiSilver,
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

pub fn get_ms_color_from_dwell(dwell:Dwell) -> Color
{
	match dwell.state
	{
		DwellState::Computed  => { return Color::RGB(0, 0, 0); }
		DwellState::Inferred  => { return get_hue_from_dwell(dwell.value); }
		DwellState::Unchecked => { panic!("get_ms_color_from_dwell: Unchecked dwell"); } //{ return Color::RGB(255, 255, 255); }
	}
}

pub fn get_color_from_dwell(fractal: &Fractal<Float>, dwell: Dwell) -> Color
{
	match fractal.color_protocol
	{
		ColorProtocol::Grayscale     => get_grayscale_from_dwell(dwell.value),
		ColorProtocol::Hue           =>       get_hue_from_dwell(dwell.value),
		ColorProtocol::MarianiSilver =>  get_ms_color_from_dwell(dwell),
	}
}



pub fn colorarray_from_dwellarray(fractal:&Fractal<Float>, dwell_array:DwellArray) -> Vec<Color>
{
	let color_array :Vec<Color>;
	color_array = dwell_array
		.into_iter()
		.map(|dwell| get_color_from_dwell(fractal, dwell))
		.collect();
	return color_array;
}

#[ensures(ret.len() == WINDOW_BYTES)]
pub fn dataarray_from_colorarray
(
//	fractal:     &Fractal<Float>,
	color_array: Vec<Color>
)
-> Vec<u8>
{
	let fractal_data_array: Vec<u8> = color_array
		.into_iter()
		.flat_map(|color| vec![color.r, color.g, color.b, color.a])
		.collect()
	;

	return fractal_data_array;
}
