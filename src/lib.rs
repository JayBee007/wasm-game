use wasm_bindgen::prelude::*;
use web_sys::{ window as sys_window};
use wasm_bindgen::{JsCast, JsValue};
use rand::{thread_rng, Rng};

struct Point(f64, f64);
struct Color(u8,u8,u8);


fn midpoint(point1: &Point, point2: &Point) -> Point {
    Point((point1.0 + point2.0) / 2.0, (point1.1 + point2.1) / 2.0)
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [&Point;3], color: &Color) {
    let [top, left, right] = points;

    let color_string = format!("rgb({},{},{})", color.0, color.1,color.2);

    context.set_fill_style(&JsValue::from_str(&color_string));
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.fill();
}

fn sierpinski(
        context: &web_sys::CanvasRenderingContext2d, 
        points: [&Point;3], 
        depth: u8, 
        color: &Color)
    {
        draw_triangle(&context, points, color);
    
        let [top, left, right] = points;
        let depth = depth - 1;

        if depth > 0 {
            let mut rng = thread_rng();

            let next_color = Color(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );

            let left_middle = midpoint(&top,&left);
            let right_middle = midpoint(&top,&right);
            let bottom_middle = midpoint(&left,&right);

            sierpinski(&context, [top, &left_middle, &right_middle], depth, &next_color);
            sierpinski(&context, [&left_middle, left, &bottom_middle], depth, &next_color);
            sierpinski(&context, [&right_middle, &bottom_middle, right], depth, &next_color);
        }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = sys_window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

        sierpinski(
            &context, 
            [&Point(300.0,0.0), &Point(0.0,600.0), &Point(600.0, 600.0)], 
            5, 
            &Color(0,255,0));

    
    
    Ok(())
}