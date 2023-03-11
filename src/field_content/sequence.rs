use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
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
}

#[derive(Debug)]
pub struct CompositeContent {
    content: Vec<FieldContent>,
}

impl SequenceU8 {
    pub fn new() -> Self {
        SequenceU8 {
            content: Vec::new(),
        }
    }
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
                content: content,
            }
        } else {
            panic!("lol");
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &**x)
    }
}

impl Field for Sequence {
    fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool {
        /*
        if let Ok(index) = ccei.message.parse::<u8>() {
            match ccei.id {
                0 => self.set_pallet(index, metadata),
                1 => self.set_call(index, metadata),
                _ => return false,
            }
            true
        } else {
            false
        }
        */
        false
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
                        SequenceType::PrimitiveU8(_) => {
                            html! {
                                <input/>
                            }
                        },
                    }
                }
            </>
        }
    }
}
