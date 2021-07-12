
// http://www.fractalforums.com/movies-showcase-(rate-my-movie)/misshapen-mandelbrot/
// z^4 - z^2 - c^2
// https://docs.rs/rusymbols/0.1.2/rusymbols/
// https://crates.io/crates/algebra
// https://crates.io/crates/un_algebra


use std::ops::Index;
use std::ops::IndexMut;

use crate::Polynomial;
use crate::Complex;
use crate::Float;
//use crate::poly;

use crate::WINDOW_W;
use crate::MAX_DWELL;

use crate::get_complex_value_from_pixel;

use crate::ColorProtocol;
use crate::RenderProtocol;



struct PolynomialFraction<T>
{
	pub num: Polynomial<T>,
	pub den: Polynomial<T>,
}

pub enum FractalProtocol<T>
{
	Julia,//(Complex<T>),
	Mandelbrot,
	Burningship,
	Duquesne,
	Newton(Complex<T>),
}

/*
** protocol:		type of dwell function
** zoom:			zoom on fractal (adjusts window rendering around anchor)
** anchor:			point of the complex plane around which the window is
**						centered
** radius:			radius of circle of convergence
** radius_sqrd:		quadratic radius of circle of convergence
** is_static:		if false, mouse hover alters iter_cpoly
** palette:			value that keeps track of which palette to use
** param:			param for newton dwell
** cur_coef:		index of current coef being edited by mouse movement
**						NB: for newton_dwell, coef == -1 alters the param
** iter_cpoly:		polynomial that is called by dwell_func
** iter_cpolyfrac:	polynomial fraction to be used by dwell that uses a
**						cpolyfrac
**
** TODO 	t_cpoly		negroots_cpolymul;
*/

pub struct Fractal<T = Float>
{
	pub iteration_protocol: FractalProtocol<T>,
	pub render_protocol:    RenderProtocol,
	pub color_protocol:     ColorProtocol,
	pub zoom:               Float,
	pub anchor:             Complex<T>,
//	pub radius:               Float,
	pub radius_sqrd:        Float,
	//pub is_static:          bool,
	pub render_w:           usize,
	pub render_h:           usize,
	//pub palette_index:      u32, //if selection from a list of predefined palettes
	//pub palette:            Palette,
	//pub cur_coef:           u32,
	pub iter_poly:          Polynomial<Complex<T>>,
	//pub iter_polyfrac:      PolynomialFraction<Complex<T>>,
}
//	t_u8		(*dwell_func)(struct s_fractol *f, t_complex z);



#[derive(Copy, Clone)]
pub struct Point
{
	pub x: usize,
	pub y: usize,
}


pub type Depth = u16;

#[derive(Copy, Clone)]
pub enum DwellState
{
	Unchecked,	// No value yet
	Computed,	// Computed directly: dwell is escape bailout value; depth is recursion depth for Mariani-Silver
	Inferred,	// Inferred from local data, like Mariani-Silver
}

#[derive(Copy, Clone)]
pub struct Dwell
{
	pub state: DwellState,
	pub value: u8,
	pub depth: Depth,
}
pub const NULL_DWELL:Dwell = Dwell { state: DwellState::Unchecked, value: 0, depth: 0 };

//pub enum Dwell
//{
//	Unchecked,							// No value yet
//	Computed { dwell: u8, depth: u8 },	// Computed directly: dwell is escape bailout value; depth is recursion depth for Mariani-Silver
//	Inferred { dwell: u8, depth: u8 },	// Inferred from local data, like Mariani-Silver
//}
//pub const NULL_DWELL:Dwell = Dwell::Unchecked;

pub type DwellArray = Vec<Dwell>;

impl Index<Point> for DwellArray
{
	type Output = Dwell;

	fn index(&self, point: Point) -> &Self::Output 
	{
		return &self[point.y * WINDOW_W + point.x];
	}
}

impl IndexMut<Point> for DwellArray
{
	fn index_mut(&mut self, point: Point) -> &mut Self::Output 
	{
		return &mut self[point.y * WINDOW_W + point.x];
	}
}

pub fn get_empty_dwell_array(fractal:&Fractal<Float>) -> DwellArray
{
	let mut dwell_array: DwellArray = vec![];

	for _ in 0 .. fractal.render_h
	{
		for _ in 0 .. fractal.render_w
		{
			dwell_array.push(NULL_DWELL);
		}
	}
	return dwell_array;
}



//TODO fix unwrap into Result<>

fn loop_dwell< F: Fn(Complex<Float>) -> Complex<Float> >
(
	z:Complex<Float>,
	bound:Float,
	iter_lambda: F
)
-> Dwell
{
	let mut z_iter:Complex<Float> = z;

	for dwell in 0..MAX_DWELL
	{
		z_iter = iter_lambda(z_iter);
		if z_iter.norm_sqr() > bound
		{
			return Dwell
			{
				state: DwellState::Computed,
				value: dwell,
				depth: 1,
			}
		}
	}
	return Dwell
	{
		state: DwellState::Computed,
		value: MAX_DWELL,
		depth: 1,
	};
}

pub fn compute_dwell_julia
(
	fractal: &Fractal<Float>,
	z:       Complex<Float>
) -> Dwell
{
	let iter_poly:Polynomial<Complex<Float>> = fractal.iter_poly.clone();

	return loop_dwell(z, fractal.radius_sqrd,
		|c|
		{
			iter_poly.eval(c).unwrap()
		}
	);
}

pub fn compute_dwell_mandelbrot
(
	fractal: &Fractal<Float>,
	z:       Complex<Float>
) -> Dwell
{
	let mut iter_poly:Polynomial<Complex<Float>> = fractal.iter_poly.clone();
	iter_poly[0] = z;

	return loop_dwell(z, fractal.radius_sqrd,
		|c|
		{
			iter_poly.eval(c).unwrap()
		}
	);
}

pub fn compute_dwell_burningship
(
	fractal: &Fractal<Float>,
	z:       Complex<Float>
) -> Dwell
{
	let mut iter_poly:Polynomial<Complex<Float>> = fractal.iter_poly.clone();
	iter_poly[0] = z;

	return loop_dwell(z, fractal.radius_sqrd,
		|c|
		{
			let abs_c = Complex
			{
				re: c.re.abs(),
				im: c.im.abs(),
			};
			return iter_poly.eval(abs_c).unwrap();
		}
	);
}

pub fn compute_dwell_duquesne
(
	fractal: &Fractal<Float>,
	z:       Complex<Float>
) -> Dwell
{
	let iter_poly:Polynomial<Complex<Float>> = fractal.iter_poly.clone();

	return loop_dwell(z, fractal.radius_sqrd,
		|c|
		{
			let mut z1 = iter_poly.eval(c).unwrap();
			z1 = Complex
			{
				re: z1.re.abs(),
				im: -z1.im,
			};
			let z2 = iter_poly.eval(z1).unwrap();
			return 0.5 * (z1 + z2);
		}
	);
}




/*
	let iter_poly:Polynomial<Complex<Float>> =
	poly!
	[
		z,//Complex::from_polar(0.7885, 1.),//Complex { re: 0.765, im: 0.432 },//z, //COMPLEX_NULL,
		Complex { re: 0., im: 0. }, //COMPLEX_NULL,
		Complex { re: 1., im: 0. }, //COMPLEX_UNIT,
	];
*/

pub fn compute_dwell
(
	fractal: &Fractal<Float>,
	z:       Complex<Float>
)
-> Dwell
{
	match fractal.iteration_protocol
	{
		FractalProtocol::Julia       => compute_dwell_julia       (fractal, z),
		FractalProtocol::Mandelbrot  => compute_dwell_mandelbrot  (fractal, z),
		FractalProtocol::Burningship => compute_dwell_burningship (fractal, z),
		FractalProtocol::Duquesne    => compute_dwell_duquesne    (fractal, z),
//		FractalProtocol::Newton(c)   => compute_dwell_newton      (fractal, z, c),
		_                            => panic!("ETF dwell protocol not implemented !"),
	}
}

pub fn get_dwell
(
	dwell_array: &mut DwellArray,
	point:       Point,
	depth:       Depth,
	fractal:     &Fractal<Float>
)
-> Dwell
{
	match dwell_array[point].state
	{
		DwellState::Unchecked =>
		{
			let z:         Complex<Float>  = get_complex_value_from_pixel(fractal, point);
			let mut dwell: Dwell           = compute_dwell(fractal, z);
			dwell.depth        = depth;
			dwell_array[point] = dwell;
			return dwell;
		},
		_ => dwell_array[point]
	}
}