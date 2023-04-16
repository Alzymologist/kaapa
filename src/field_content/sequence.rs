use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::Encode;
use scale_info::{
    form::PortableForm, Type, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact,
    TypeDefComposite, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, Variant,
};
use serde_json::value::Value;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement, WebSocket};
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

use crate::field_content::{Field, FieldContent};
use crate::messaging::{CallConstructorEvent, CCEI};

#[derive(Debug)]
pub struct Sequence {
    name: Option<String>,
    compact: bool,
    content: SequenceType,
}

#[derive(Debug)]
enum SequenceType {
    PrimitiveU8(SequenceU8),
    Composite(CompositeContent),
}

#[derive(Debug)]
pub struct SequenceU8 {
    content: Vec<u8>,
    input_mode: StringInputMode,
}

#[derive(Debug)]
pub struct CompositeContent {
    content: Vec<FieldContent>,
}

impl SequenceU8 {
    pub fn new() -> Self {
        SequenceU8 {
            content: Vec::new(),
            input_mode: StringInputMode::UTF,
        }
    }

    pub fn show(&self) -> String {
        match self.input_mode {
            StringInputMode::UTF => if let Ok(a) = String::from_utf8(self.content.clone()) { a } else { "error".to_string() },
            StringInputMode::Hex => hex::encode(&self.content),
        }
    }
}

#[derive(Debug)]
enum StringInputMode {
    Hex,
    UTF,
}

impl CompositeContent {
    pub fn new() -> Self {
        CompositeContent {
            content: Vec::new(),
        }
    }
}

impl Sequence {
    pub fn resolve(
        input: &TypeDefSequence<PortableForm>,
        name: Option<&str>,
        compact: bool, // TODO this should be useless, as sequences are always compact and inner
                       // values take care of themselves
        metadata: &RuntimeMetadataV14,
    ) -> Self {
        let name = match name {
            Some(a) => Some(a.to_string()),
            None => None,
        };
        if let Some(a) = metadata.types.resolve(input.type_param().id()) {
            let content = match a.type_def() {
                TypeDef::Primitive(b) => match b {
                    _/*TypeDefPrimitive::U8*/ => SequenceType::PrimitiveU8(SequenceU8::new()),
                },
                TypeDef::Composite(b) => SequenceType::Composite(CompositeContent::new()),
                TypeDef::Variant(b) => SequenceType::Composite(CompositeContent::new()),
                TypeDef::Sequence(b) => SequenceType::Composite(CompositeContent::new()),
                TypeDef::Array(b) => SequenceType::Composite(CompositeContent::new()),
                TypeDef::Tuple(b) => SequenceType::Composite(CompositeContent::new()),
                TypeDef::Compact(b) => SequenceType::Composite(CompositeContent::new()),
                TypeDef::BitSequence(b) => SequenceType::Composite(CompositeContent::new()),
            };
            Sequence {
                name: name,
                compact: compact,
                content: content,
            }
        } else {
            panic!("lol");
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &**x)
    }

    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        //self.fields.get_mut(*id)
        None
    }

}

impl Field for Sequence {
    fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool {
        match self.content {
            SequenceType::PrimitiveU8(ref mut a) => match ccei.id {
                0 => {
                    a.content = ccei.message.into();
                    true
                },
                _ => return false,
            },
            SequenceType::Composite(_) => false,
        }
    }

    fn get_children(&self) -> &[FieldContent] {
        match self.content {
            SequenceType::Composite(ref a) => &a.content,
            SequenceType::PrimitiveU8(_) => &[],
        }
    }

    fn render_self(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
    metadata: &RuntimeMetadataV14,
    ) -> Html {
        html! {
            <>
                {
                    if let Some(ref a) = self.name {
                        html! { <p>{a}</p> }
                    } else {
                        html! {}
                    }
                }
                {
                    match self.content {
                        SequenceType::Composite(ref a) => html! { <p>{"please add rendering"}</p> },
                        SequenceType::PrimitiveU8(ref a) => {
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
                            let value = a.show().clone();
                            html! {
                                <>
                                    <input oninput={oninput}/>
                                </>
                            }
                        },
                    }
                }
            </>
        }
    }

    fn encoded_self(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        let mut out = Vec::new();
        match self.content {
            SequenceType::PrimitiveU8(ref a) => {
                out = a.content.encode();
            },
            SequenceType::Composite(ref _a) => {},
        }
        out        
    }
}
