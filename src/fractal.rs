
use crate::Polynomial;
use crate::Complex;
use crate::Float;

use crate::MAX_DWELL;

//use crate::poly;

use crate::ColorProtocol;
use crate::RenderProtocol;

struct PolynomialFraction<T>
{
	num:Polynomial<T>,
	den:Polynomial<T>,
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
	pub iteration_protocol:FractalProtocol<T>,
	pub render_protocol:RenderProtocol,
	pub color_protocol:ColorProtocol,
	pub zoom:Float,
	pub anchor:Complex<T>,
//	pub radius:Float,
	pub radius_sqrd:Float,
	//pub is_static:bool,
	pub render_w:u32,
	pub render_h:u32,
	//pub palette_index:u32, //if selection from a list of predefined palettes
	//pub palette:Palette,
	//pub cur_coef:u32,
	pub iter_poly:Polynomial<Complex<T>>,
	//pub iter_polyfrac:PolynomialFraction<Complex<T>>,
}
//	t_u8		(*dwell_func)(struct s_fractol *f, t_complex z);

#[derive(Copy, Clone)]
pub struct Dwell
{
	pub value:u8,
	pub checked:bool,
}

#[derive(Copy, Clone)]
pub struct Point
{
	pub x:u16,
	pub y:u16,
}

pub type Figure = Vec<Point>;



//TODO fix unwrap into Result<>

fn loop_dwell< F: Fn(Complex<Float>) -> Complex<Float> >
(
	z:Complex<Float>,
	bound:Float,
	iter_lambda: F
) -> Dwell
{
	let mut z_iter:Complex<Float> = z;

	for dwell in 0..MAX_DWELL
	{
		z_iter = iter_lambda(z_iter);
		if z_iter.norm_sqr() > bound
		{
			return Dwell
			{
				value: dwell,
				checked: true,
			}
		}
	}
	return Dwell
	{
		value: MAX_DWELL,
		checked: true,
	};
}

pub fn get_dwell_julia
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

pub fn get_dwell_mandelbrot
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

pub fn get_dwell_burningship
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

pub fn get_dwell_duquesne
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

pub fn get_dwell
(
	fractal: &Fractal<Float>,
	z:       Complex<Float>
) -> Dwell
{
	match fractal.iteration_protocol
	{
		FractalProtocol::Julia       => get_dwell_julia       (fractal, z),
		FractalProtocol::Mandelbrot  => get_dwell_mandelbrot  (fractal, z),
		FractalProtocol::Burningship => get_dwell_burningship (fractal, z),
		FractalProtocol::Duquesne    => get_dwell_duquesne    (fractal, z),
//		FractalProtocol::Newton(c)   => get_dwell_newton      (fractal, z, c),
		_                            => panic!("ETF dwell protocol not implemented !"),
	}
}
