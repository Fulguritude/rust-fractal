extern crate sdl2;

use std::ops;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub trait Null
{
    const NULL: Self;
}

type Float = f64;

#[derive(Copy, Clone)]
enum Scalar
{
	Real          (Float),
	Complex       {x: Float, y: Float},
	Split         {x: Float, y: Float},
	Dual          {x: Float, y: Float},
//	Complex_Polar {r: Float, t: Float},
//	Split_Polar   {r: Float, t: Float},
//	Dual_Polar    {r: Float, t: Float},
}

impl Null for crate::Scalar   { const NULL: Self = Self::Real(0.0); }

impl ops::Add<Scalar> for Scalar
{
	type Output = Option<Scalar>;

	fn add(self, _rhs:Scalar) -> Option<Scalar>
	{
		match (self, _rhs)
		{
			(Scalar::Real    (n           ), Scalar::Real    (m           )) => Some(Scalar::Real    (   n  + m             )),
			(Scalar::Real    (n           ), Scalar::Complex {   x,     y }) => Some(Scalar::Complex {x: n  + x,  y: y      }),
			(Scalar::Real    (n           ), Scalar::Split   {   x,     y }) => Some(Scalar::Split   {x: n  + x,  y: y      }),
			(Scalar::Real    (n           ), Scalar::Dual    {   x,     y }) => Some(Scalar::Dual    {x: n  + x,  y: y      }),
			(Scalar::Complex {   x,     y }, Scalar::Real    (n           )) => Some(Scalar::Complex {x: n  + x,  y: y      }),
			(Scalar::Split   {   x,     y }, Scalar::Real    (n           )) => Some(Scalar::Split   {x: n  + x,  y: y      }),
			(Scalar::Dual    {   x,     y }, Scalar::Real    (n           )) => Some(Scalar::Dual    {x: n  + x,  y: y      }),
			(Scalar::Complex {x: x1, y: y1}, Scalar::Complex {x: x2, y: y2}) => Some(Scalar::Complex {x: x1 + x2, y: y1 + y2}),
			(Scalar::Split   {x: x1, y: y1}, Scalar::Split   {x: x2, y: y2}) => Some(Scalar::Split   {x: x1 + x2, y: y1 + y2}),
			(Scalar::Dual    {x: x1, y: y1}, Scalar::Dual    {x: x2, y: y2}) => Some(Scalar::Dual    {x: x1 + x2, y: y1 + y2}),
			_ => None
		}
	}
}

/*
type Dimension = u32;

enum AlgebraicModule
{
	ModuleReal    (Dimension),
	ModuleComplex (Dimension),
	ModuleSplit   (Dimension),
	ModuleDual    (Dimension),
}
*/
// Ask how to make a struct that needs a trait thats a combination of other traits
/*
struct Monoid <T>(T);
struct Group  <T>(T);
struct Ring   <T>(T);
struct Field  <T>(T);
*/

struct Polynomial<T = Scalar>
{
	coefs: Vec<T>,
}

impl Null for Polynomial
{
    const NULL: Self = Self{coefs: vec![]};
}

impl ops::Index<usize> for Polynomial
{
	type Output = Scalar;

	fn index(&self, index: usize) -> &Scalar
	{
		&self.coefs[index]
    }
}

impl<T: Null> Polynomial<T>
{
	fn new(coefs:Vec<T>) -> Polynomial<T>
	{
		let mut result:Polynomial<T>;
		if coefs.len() == 0
		{
			return Self::null();
		}
		result.coefs = coefs;
		result
	}

	fn null() -> Polynomial<T>
	{
		Self::new(vec![])
	}

	fn len(&self) -> usize
	{
		self.coefs.len()
	}

	fn degree(&self) -> usize
	{
		if self.len() > 0
		{
			self.len() - 1
		}
		else
		{
			0
		}
	}


	fn eval(&self, value:T) -> T
	{
		let mut result:T;
		if self.len() == 0
		{
			return T::NULL
		}
		result = self.coefs[self.degree()];
		let max_index:i32 = (self.degree() as i32) - 1;
		for i in max_index..-1
		{
			result = value * result;
			result += self[i];
		}
		result
	}
}

//impl<T> ops::Add<Polynomial<T>> for Polynomial<T>
//	where T: Scalar
impl ops::Add<Polynomial> for Polynomial
{
	type Output = Polynomial;

	fn add(self, _rhs: Polynomial) -> Polynomial
	{
		let poly_min:&Polynomial;
		let poly_max:&Polynomial;
		if self.degree() <= _rhs.degree()
		{
			poly_min = &self;
			poly_max = &_rhs;
		}
		else
		{	
			poly_max = &self;
			poly_min = &_rhs;
		}
//		slice_min = &self.coefs[0..degree_min] + &_rhs.coefs[0..degree_min];
		let mut result:Polynomial = Polynomial::NULL;
		for i in 0..poly_min.degree()
		{
			let maybe_sum = poly_min.coefs[i] + poly_max.coefs[i];
			match maybe_sum
			{
				Some(z) => result.coefs.push(z),
				None => result.coefs.push(Scalar::Real(0.0))
			}
		}
		for i in poly_min.degree()..poly_max.degree()
		{
			result.coefs.push(poly_max.coefs[i])
		}

		result
	}
}

/*
pub trait GeometricAlgebra<T: AlgebraicModule>
{
	//Construct a
	fn new(z: T) -> Self;
}
*/


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

	println!("lolol");

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop
    {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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