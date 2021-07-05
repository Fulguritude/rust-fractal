
use crate::Color;
use crate::Dwell;
use crate::Fractal;
use crate::Float;

//use crate::WINDOW_W;
//use crate::WINDOW_H;
//use crate::WINDOW_BYTES;

use crate::get_complex_value_from_pixel;
use crate::get_dwell;
use crate::colorarray_from_dwellarray;
use crate::dataarray_from_colorarray;



pub enum RenderProtocol
{
	PixelByPixel,
	MarianiSilver,
}



pub fn compute_pixel_per_pixel(fractal:&Fractal<Float>) -> Vec<Dwell>
{
	let mut dwell_array:Vec<Dwell> = vec![];

	for y in 0..fractal.render_h
	{
		for x in 0..fractal.render_w
		{
			let z = get_complex_value_from_pixel(fractal, x, y);
			let dwell:Dwell = get_dwell(fractal, z);
			dwell_array.push(dwell);
//println!("Dwell for {} is {}", z, dwell);
		}
	}

	return dwell_array;
}

/*
pub fn compute_mariani_silver_quadtree(fractal:&Fractal<Float>, dwell_array:&mut DwellArray) -> ()
{

}
*/

pub fn render_fractal(fractal:&Fractal<Float>) -> Vec<u8>
{
	let result:Vec<Dwell>;

	result = match fractal.render_protocol
	{
		RenderProtocol::PixelByPixel  => compute_pixel_per_pixel(fractal),
//		RenderProtocol::MarianiSilver => compute_mariani_silver_quadtree(fractal),
		_                             => panic!("Render protocol not implemented!")
	};
	let color_array:Vec<Color> = colorarray_from_dwellarray(fractal, result);
	let data_array:Vec<u8> = dataarray_from_colorarray(fractal, color_array);
	return data_array;
} 