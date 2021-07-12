
use crate::Event;
use crate::Complex;
use crate::Fractal;
use crate::Float;

use crate::Keycode;



pub fn handle_key_press(e: Event, fractal: &mut Fractal) -> bool
{
	let increment:Float = 0.1;

	match e
	{
		Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { return true; }

		Event::KeyDown { keycode: Some(Keycode::Right)   , .. } => { println!("Right     "); fractal.anchor += Complex { re: fractal.zoom * increment, im:                       0. }; }
		Event::KeyDown { keycode: Some(Keycode::Left)    , .. } => { println!("Left      "); fractal.anchor -= Complex { re: fractal.zoom * increment, im:                       0. }; }
		Event::KeyDown { keycode: Some(Keycode::Up)      , .. } => { println!("Up        "); fractal.anchor += Complex { re:                       0., im: fractal.zoom * increment }; }
		Event::KeyDown { keycode: Some(Keycode::Down)    , .. } => { println!("Down      "); fractal.anchor -= Complex { re:                       0., im: fractal.zoom * increment }; }
		Event::KeyDown { keycode: Some(Keycode::PageUp)  , .. } => { println!("PageUp    "); fractal.zoom *= increment ; }
		Event::KeyDown { keycode: Some(Keycode::PageDown), .. } => { println!("PageDown  "); fractal.zoom /= increment ; }
		_ => {}
	}
	return false;
}