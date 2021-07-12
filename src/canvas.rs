
use crate::Fractal;
use crate::Float;
use crate::Complex;
use crate::Point;
use crate::TextureCreator;
use crate::Texture;
use crate::Canvas;
use crate::Window;
use crate::WindowContext;
use crate::Result;
use crate::WINDOW_H;
use crate::WINDOW_W;
use crate::WINDOW_PITCH;


use crate::render_fractal;



//use crate::COMPLEX_PLANE_STEP;
//use crate::COMPLEX_PLANE_CENTER;
/*
t_complex		get_complex_from_point(t_fractol *frac, t_s32 x, t_s32 y)
{
	t_complex		result;
	static t_float	inv_ren_w = 1. / REN_W;
	static t_float	inv_ren_h = 1. / REN_H;

	result = frac->anchor;
	result.re += frac->zoom * (x - REN_W / 2) * inv_ren_w;
	result.im += frac->zoom * (y - REN_H / 2) * inv_ren_h;
	return (result);
}
*/

pub fn get_complex_value_from_pixel
(
	fractal: &Fractal<Float>,
	point:   Point,
)
-> Complex<Float>
{
	let inv_render_w:Float = 1. / (fractal.render_w as Float);
	let inv_render_h:Float = 1. / (fractal.render_h as Float);

	let rel_x:Float = (point.x as i32 - fractal.render_w as i32 / 2) as Float;
	let rel_y:Float = (point.y as i32 - fractal.render_h as i32 / 2) as Float;

	let offset_from_center =
		Complex
		{
			re: fractal.zoom * rel_x * inv_render_w,
			im: fractal.zoom * rel_y * inv_render_h,
		}
	;

	return fractal.anchor + offset_from_center;
}


pub fn draw_fractal(canvas:&mut Canvas<Window>, fractal:&Fractal) -> Result<()>
{
println!("Rendering fractal...");
	let fractal_data:Vec<u8> = render_fractal(fractal);
	let texture_creator:TextureCreator<WindowContext> = canvas.texture_creator();
	let mut texture:Texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), WINDOW_W as u32, WINDOW_H as u32)?;
	texture.update(None, &fractal_data, WINDOW_PITCH)?;
	canvas.copy(&texture, None, None)?;
	Ok(())
}
