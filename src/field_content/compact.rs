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
pub struct Compact {
    name: Option<String>,
    value: Vec<u8>,
    max: usize,
}

impl Compact {
    pub fn resolve(
        input: &TypeDefCompact<PortableForm>,
        name: Option<&str>,
        metadata: &RuntimeMetadataV14,
    ) -> FieldContent {
        let name = match name {
            Some(a) => Some(a.to_string()),
            None => None,
        };
        let mut max = 0;
        if let Some(a) = metadata.types.resolve(input.type_param().id()) {
            let content = match a.type_def() {
                TypeDef::Primitive(b) => match b {
                    TypeDefPrimitive::U8 => max = 1,
                    _ => return FieldContent::Error(String::from("unhandled")),
                },
                TypeDef::Composite(b) => max = 32,
                TypeDef::Variant(b) => max = 32,
                TypeDef::Sequence(b) => max = 32,
                TypeDef::Array(b) => max = 32,
                TypeDef::Tuple(b) => max = 32,
                TypeDef::Compact(b) => max = 32,
                TypeDef::BitSequence(b) => max = 32,
            };
            FieldContent::Compact(
                Compact {
                    name: name,
                    value: vec![0],
                    max: max,
                }
            )
        } else {
            FieldContent::Error(format!("Type could not be resolved: {}", input.type_param().id()))
        }

    }

}

