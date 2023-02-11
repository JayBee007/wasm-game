use std::{rc::Rc, sync::Mutex};

use wasm_bindgen::prelude::*;
use web_sys::{ window as sys_window};
use wasm_bindgen::{JsCast, JsValue};


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

    wasm_bindgen_futures::spawn_local(async move {
 
        let (sender, receiver ) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        
        let sender  = Rc::new(Mutex::new(Some(sender)));
        let error = Rc::clone(&sender);

        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            if let Some(sender) = sender.lock().ok()
                .and_then(|mut opt| opt.take()) {
                    sender.send(Ok(()));
                }
        });

        let error_callback = Closure::once(move |err| {
            if let Some(error) = error.lock().ok()
                .and_then(|mut opt| opt.take()) {
                    error.send(Err(err));
                }
        });
        

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        image.set_src("./assets/resized/rhb/Idle (1).png");

        receiver.await.unwrap();

        context.draw_image_with_html_image_element(&image, 0.0, 0.0).unwrap();
    });
    
    Ok(())
}