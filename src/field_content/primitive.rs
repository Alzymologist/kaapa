use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use num_bigint::{BigInt, BigUint, Sign};
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

use crate::field_content::{components::{render_input_field, render_name}, Field, FieldContent};
use crate::messaging::{CallConstructorEvent, CCEI};

#[derive(Debug)]
pub struct Primitive {
    name: Option<String>,
    compact: bool,
    input_mode: InputMode,
    content: PrimitiveContent,
}

#[derive(Debug)]
enum PrimitiveContent {
    Bool(bool),
    Char(char),
    Str(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(BigUint),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    I256(BigInt),
}

#[derive(Debug)]
enum InputMode {
    Dec,
    Hex,
    UTF,
}

impl Primitive {
    pub fn resolve(
        input: &TypeDefPrimitive,
        name: Option<&str>,
        compact: bool,
        metadata: &RuntimeMetadataV14,
    ) -> Self {
        let name = match name {
            Some(a) => Some(a.to_string()),
            None => None,
        };
        let content = match input {
            TypeDefPrimitive::Bool => PrimitiveContent::Bool(false),
            TypeDefPrimitive::Char => PrimitiveContent::Char('a'),
            TypeDefPrimitive::Str => PrimitiveContent::Str(String::new()),
            TypeDefPrimitive::U8 => PrimitiveContent::U8(0),
            TypeDefPrimitive::U16 => PrimitiveContent::U16(0),
            TypeDefPrimitive::U32 => PrimitiveContent::U32(0),
            TypeDefPrimitive::U64 => PrimitiveContent::U64(0),
            TypeDefPrimitive::U128 => PrimitiveContent::U128(0),
            TypeDefPrimitive::U256 => PrimitiveContent::U256(BigUint::new(vec![0])),
            TypeDefPrimitive::I8 => PrimitiveContent::I8(0),
            TypeDefPrimitive::I16 => PrimitiveContent::I16(0),
            TypeDefPrimitive::I32 => PrimitiveContent::I32(0),
            TypeDefPrimitive::I64 => PrimitiveContent::I64(0),
            TypeDefPrimitive::I128 => PrimitiveContent::I128(0),
            TypeDefPrimitive::I256 => PrimitiveContent::I256(BigInt::new(Sign::Plus, vec![0])),
        };
        Primitive {
            name: name,
            compact: compact,
            input_mode: InputMode::Hex,
            content: content,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &**x)
    }

    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        None
    }
}

impl Field for Primitive {
    fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool {
        match self.content {
            PrimitiveContent::Bool(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::Char(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::Str(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::U8(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::U16(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::U32(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::U64(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::U128(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::U256(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::I8(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::I16(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::I32(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::I64(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::I128(ref mut a) => {
                //a = ccei.message.into();
                true
            },
            PrimitiveContent::I256(ref mut a) => {
                //a = ccei.message.into();
                true
            },
        }
    }

    fn get_children(&self) -> &[FieldContent] {
        &[]
    }

    fn render_self(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
    metadata: &RuntimeMetadataV14,
    ) -> Html {
        html! {
            <>
                {render_name(&self.name)}
                {
                    match &self.content {
                        PrimitiveContent::Bool(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::Char(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::Str(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::U8(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::U16(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::U32(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::U64(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::U128(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::U256(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::I8(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::I16(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::I32(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::I64(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::I128(ref _a) => render_input_field(parent, callback_original),
                        PrimitiveContent::I256(ref _a) => render_input_field(parent, callback_original),
                    }
                }
            </>
        }
    }

    fn encoded_self(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        let mut out = Vec::new();
        match &self.content {
            PrimitiveContent::Bool(a) => out.push(if *a { 1 } else { 0 }),
            PrimitiveContent::Char(a) => out.extend_from_slice(&a.to_string().as_bytes()),
            PrimitiveContent::Str(a) => out.extend_from_slice(&a.as_bytes()),
            PrimitiveContent::U8(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::U16(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::U32(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::U64(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::U128(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::U256(a) => out.extend_from_slice(&a.to_bytes_be()),
            PrimitiveContent::I8(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::I16(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::I32(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::I64(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::I128(a) => out.extend_from_slice(&a.to_be_bytes()),
            PrimitiveContent::I256(a) => out.extend_from_slice(&a.to_bytes_be().1),

        }
        out        
    }
}
