use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.get_element_by_id("canvas").unwrap();
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    context.arc(75.0, 75.0, 50.0, 0.0, f64::consts::TAU).unwrap();
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();
    context.move_to(65.0, 65.0);
    context.arc(60.0, 65.0, 5.0, 0.0, f64::consts::TAU).unwrap();
    context.move_to(95.0, 65.0);
    context.arc(90.0, 65.0, 5.0, 0.0, f64::consts::TAU).unwrap();
    context.stroke();

    Ok(())
}
