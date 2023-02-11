use std::{rc::Rc, sync::Mutex, collections::HashMap};
use wasm_bindgen::prelude::*;
use web_sys::{ window as sys_window};
use wasm_bindgen::{JsCast, JsValue};
use serde::Deserialize;

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>
}
#[derive(Deserialize)]
struct Cell {
    frame: Rect
}

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16
}


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
                    sender.send(Ok(())).unwrap();
                }
        });

        let error_callback = Closure::once(move |err| {
            if let Some(error) = error.lock().ok()
                .and_then(|mut opt| opt.take()) {
                    error.send(Err(err)).unwrap();
                }
        });
        

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        image.set_src("./assets/resized/rhb/Idle (1).png");

        receiver.await.unwrap().unwrap();

        context.draw_image_with_html_image_element(&image, 0.0, 0.0).unwrap();

        let json = fetch_json("./assets/sprite_sheets/rhb.json").await.unwrap();

        let sheet:Sheet = serde_wasm_bindgen::from_value(json).unwrap();


        /////////////////////////////////////////////////////////////////////////
        let (sender, receiver ) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        
        let sender  = Rc::new(Mutex::new(Some(sender)));
        let error = Rc::clone(&sender);

        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            if let Some(sender) = sender.lock().ok()
                .and_then(|mut opt| opt.take()) {
                    sender.send(Ok(())).unwrap();
                }
        });

        let error_callback = Closure::once(move |err| {
            if let Some(error) = error.lock().ok()
                .and_then(|mut opt| opt.take()) {
                    error.send(Err(err)).unwrap();
                }
        });
        

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        image.set_src("./assets/sprite_sheets/rhb.png");

        receiver.await.unwrap().unwrap();

        let sprite = sheet.frames.get("Run (1).png").expect("Cell not found");

        context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &image, 
            sprite.frame.x.into(), 
            sprite.frame.y.into(), 
            sprite.frame.w.into(), 
            sprite.frame.h.into(), 
            300.0, 
            300.0, 
            sprite.frame.w.into(), 
            sprite.frame.h.into()).unwrap();

    });
    
    Ok(())
}

async fn fetch_json(json_path:&str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    let response = wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_str(json_path)).await?;

    let response_value: web_sys::Response = response.dyn_into()?;

    wasm_bindgen_futures::JsFuture::from(response_value.json()?).await
}