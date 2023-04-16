use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement, WebSocket};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::{
    events,
    platform::{
        pinned::mpsc::UnboundedSender,
        spawn_local,
        time::{interval, sleep},
    },
    AttrValue, Callback,
};

use crate::messaging::{CallConstructorEvent, CCEI};

pub fn render_input_field(parent: Vec<usize>, callback_original: Callback<CallConstructorEvent>) -> Html {
    let callback = callback_original.clone();
    let parent1 = parent.clone();
    let oninput = Callback::from(move |e: InputEvent| {
        let new_value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        callback.emit(CallConstructorEvent {
            index: parent.clone(),
            ccei: CCEI {
                id: 0,
                message: new_value,
            }
        })
    });
    html! {
        <>
            <input oninput={oninput}/>
        </>
    }   
}


pub fn render_name(name: &Option<String>) -> Html {
    if let Some(ref a) = name {
                        html! { <p>{a}</p> }
                    } else {
                        html! {}
                    }

}
