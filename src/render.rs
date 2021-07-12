
use crate::Depth;
use crate::Float;
use crate::Complex;
use crate::Fractal;
use crate::DwellArray;
use crate::Point;
use crate::Color;
use crate::Dwell;
use crate::DwellState;
//use crate::NULL_DWELL;

//use crate::WINDOW_W;
//use crate::WINDOW_H;
//use crate::WINDOW_BYTES;

use crate::get_complex_value_from_pixel;
use crate::compute_dwell;
use crate::get_dwell;
use crate::get_empty_dwell_array;

use crate::colorarray_from_dwellarray;
use crate::dataarray_from_colorarray;



pub enum RenderProtocol
{
	PixelByPixel,
	MarianiSilver,
}



pub fn compute_pixel_per_pixel(fractal:&Fractal<Float>) -> DwellArray
{
	let mut dwell_array:DwellArray = vec![];

	for y in 0..fractal.render_h
	{
		for x in 0..fractal.render_w
		{
			let point:Point       = Point { x: x, y: y };
			let z: Complex<Float> = get_complex_value_from_pixel(fractal, point);
			let dwell: Dwell      = compute_dwell(fractal, z);
			dwell_array.push(dwell);
//println!("Dwell for {} is {}", z, dwell);
		}
	}

	return dwell_array;
}



/*
pub struct FractalPoint
{
	pub point:Point,
	pub dwell:Dwell,
}
*/

pub type Figure = Vec<Point>;

// NB: "end" is included in the rectangle
pub fn get_rectangle_boundary(ini: Point, end: Point) -> Option<Figure>
{
	let mut result:Figure = vec![];

	if end.x < ini.x ||
	   end.y < ini.y
	{
		return None
	}

	for i in ini.x ..= end.x
	{
		let mut point: Point;
		point = Point { x: i, y: ini.y };
		result.push(point);
		point = Point { x: i, y: end.y };
		result.push(point);
	}

	for i in ini.y+1 .. end.y
	{
		let mut point: Point;
		point = Point { x: ini.x, y: i };
		result.push(point);
		point = Point { x: end.x, y: i };
		result.push(point);
	}

	return Some(result);
}

pub fn get_rectangle_interior(ini: Point, end: Point) -> Option<Figure>
{
	let mut result:Figure = vec![];

	if end.x <= ini.x + 1 ||
	   end.y <= ini.y + 1
	{
		return None
	}

	//TODO functional-style improvement ? 
	for y in ini.y+1 .. end.y
	{
		for x in ini.x+1 .. end.x
		{
			let point: Point = Point { x: x, y: y };
			result.push(point);
		}
	}

	return Some(result);
}

type QuadTreeRef = Option<Box<QuadTree>>;

pub struct QuadTree
{
	pub rect:   Option<Figure>,
//	pub top_l:  QuadTreeRef,
//	pub top_r:  QuadTreeRef,
//	pub bot_l:  QuadTreeRef,
//	pub bot_r:  QuadTreeRef,
}

fn get_quadtreeref(ini: Point, end: Point) -> QuadTreeRef
{
	if end.x < ini.x ||
	   end.y < ini.y
	{
		return None
	}

//	let ini_x: usize = ini.x;
//	let ini_y: usize = ini.y;
//
//	let mid_x: usize = ini.x + (ini.x + end.x) / 2;
//	let mid_y: usize = ini.y + (ini.y + end.y) / 2;
//
//	let end_x: usize = end.x;
//	let end_y: usize = end.y;

	let quadtree: Box<QuadTree> = Box::new(
		QuadTree
		{
			rect:   get_rectangle_boundary(ini, end),
//			top_l:  None,
//			top_r:  None,
//			bot_l:  None,
//			bot_r:  None,
		}
	);

	return Some(quadtree);
}

pub fn compute_mariani_silver_quadtree
(
	fractal:     &Fractal<Float>
)
-> DwellArray
{
	//TODO improve dwell array; use figures as references to instances of the dwell array ? Or indices ?
	fn rec_mariani_silver_quadtree
	(
		fractal:     &Fractal<Float>,
		dwell_array: &mut DwellArray,
		ini:         Point,
		end:         Point,
		depth:       Depth,
	)
	-> ()
	{
		let quadtreeref: QuadTreeRef = get_quadtreeref(ini, end);

		let quadtree:Box<QuadTree>;
		match quadtreeref
		{
			Some(qt) => { quadtree = qt; }
			None     => { return ; }
		}

		let rect:Figure;
		match quadtree.rect
		{
			Some(r) => { rect = r; }
			None    => { return ; }
		}

		let mut all_same_dwell: bool  = true;
		let first_dwell:        Dwell = get_dwell(dwell_array, rect[0], depth, fractal);
		for point in rect
		{
			let dwell: Dwell = get_dwell(dwell_array, point, depth, fractal);
			match dwell.state
			{
				DwellState::Computed =>
				{
					if dwell.value != first_dwell.value
					{
						all_same_dwell = false;
					}
				},
				_ => panic!("MarianiSilver recursion error")
			} 
		}

		if all_same_dwell
		{
			let rect_interior: Option<Figure> = get_rectangle_interior(ini, end);
			match rect_interior
			{
				Some(r) =>
				{
					for point in r
					{
						dwell_array[point] = Dwell
						{
							state: DwellState::Inferred,
							value: first_dwell.value,
							depth: depth,
						}
					}
				}
				None    => { return ; }
			}
		}
		else
		{
			let ini_x: usize = ini.x;
			let ini_y: usize = ini.y;
			let mid_x: usize = (ini.x + end.x) / 2;
			let mid_y: usize = (ini.y + end.y) / 2;
			let end_x: usize = end.x;
			let end_y: usize = end.y;

//println!(
//"ini_x: {} | ini_y: {} | mid_x: {} | mid_y: {} | end_x: {} | end_y: {} ",
//ini_x.to_string(), ini_y.to_string(), mid_x.to_string(), mid_y.to_string(), end_x.to_string(), end_y.to_string()
//);
			if mid_x != end.x || mid_y != end_y { rec_mariani_silver_quadtree(fractal, dwell_array, Point { x: ini_x, y: ini_y }, Point { x: mid_x, y: mid_y }, depth + 1); }
			if mid_x != ini.x || mid_y != end_y { rec_mariani_silver_quadtree(fractal, dwell_array, Point { x: mid_x, y: ini_y }, Point { x: end_x, y: mid_y }, depth + 1); }
			if mid_x != end.x || mid_y != ini_y { rec_mariani_silver_quadtree(fractal, dwell_array, Point { x: ini_x, y: mid_y }, Point { x: mid_x, y: end_y }, depth + 1); }
			if mid_x != ini.x || mid_y != ini_y { rec_mariani_silver_quadtree(fractal, dwell_array, Point { x: mid_x, y: mid_y }, Point { x: end_x, y: end_y }, depth + 1); }
		}
  	}

  	let mut dwell_array: DwellArray = get_empty_dwell_array(fractal);

	let ini: Point = Point { x:                    0, y:                    0 };
	let end:   Point = Point { x: fractal.render_w - 1, y: fractal.render_h - 1 };

	rec_mariani_silver_quadtree(fractal, &mut dwell_array, ini, end, 0);


	return dwell_array;
}


pub fn render_fractal(fractal:&Fractal<Float>) -> Vec<u8>
{
	let result:DwellArray;

	result = match fractal.render_protocol
	{
		RenderProtocol::PixelByPixel  => compute_pixel_per_pixel(fractal),
		RenderProtocol::MarianiSilver => compute_mariani_silver_quadtree(fractal),
//		_                             => panic!("Render protocol not implemented!")
	};
	let color_array: Vec<Color> = colorarray_from_dwellarray(fractal, result);
	let data_array:  Vec<u8>    =  dataarray_from_colorarray(/*fractal,*/ color_array);
	return data_array;
} 