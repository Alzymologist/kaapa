use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};

use scale_info::{
    form::PortableForm, Type, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact,
    TypeDefComposite, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, Variant,
};

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement, WebSocket};
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

pub mod pallet_call;
use pallet_call::PalletCall;

pub mod composite;
use composite::Composite;

pub mod sequence;
use sequence::Sequence;

use crate::messaging::{CallConstructorEvent, CCEI};

#[derive(Debug)]
pub enum FieldContent {
    PalletCall(PalletCall),
    TypeDefVariant(TypeDefVariant<PortableForm>),
    Composite(Composite),
    Sequence(Sequence),
    TypeDefArray(TypeDefArray<PortableForm>),
    TypeDefTuple(TypeDefTuple<PortableForm>),
    TypeDefPrimitive(TypeDefPrimitive),
    TypeDefCompact(TypeDefCompact<PortableForm>),
    TypeDefBitSequence(TypeDefBitSequence<PortableForm>),
    Stub(Type<PortableForm>),
    Error(String),
}

impl FieldContent {
    pub fn new(
        input: Option<&Type<PortableForm>>,
        name: Option<&str>,
        metadata: &RuntimeMetadataV14,
    ) -> Self {
        match input {
            Some(a) => match a.type_def() {
                TypeDef::Variant(b) => FieldContent::TypeDefVariant(b.to_owned()),
                TypeDef::Composite(ref b) => {
                    FieldContent::Composite(Composite::resolve(b, name, metadata))
                }
                TypeDef::Sequence(b) => {
                    FieldContent::Sequence(Sequence::resolve(b, name, metadata))
                }
                TypeDef::Array(b) => FieldContent::TypeDefArray(b.to_owned()),
                TypeDef::Tuple(b) => FieldContent::TypeDefTuple(b.to_owned()),
                TypeDef::Primitive(b) => FieldContent::TypeDefPrimitive(b.to_owned()),
                TypeDef::Compact(b) => FieldContent::TypeDefCompact(b.to_owned()),
                TypeDef::BitSequence(b) => FieldContent::TypeDefBitSequence(b.to_owned()),
            },
            None => FieldContent::Error("type not found".to_owned()),
        }
    }

    pub fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool {
        match self {
            FieldContent::PalletCall(ref mut a) => a.handle_event(ccei, metadata),
            FieldContent::Stub(_) => false,
            FieldContent::Error(_) => false,
            _ => false,
        }
    }

    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        match self {
            FieldContent::PalletCall(a) => a.get_child(id),
            FieldContent::Stub(_) => None,
            FieldContent::Error(_) => None,
            _ => None,
        }
    }

    pub fn get_cards(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
        metadata: &RuntimeMetadataV14,
    ) -> Vec<Html> {
        match self {
            FieldContent::PalletCall(ref a) => a.render(parent, callback_original, metadata),
            FieldContent::Composite(ref a) => a.render(parent, callback_original, metadata),
            FieldContent::Sequence(ref a) => a.render(parent, callback_original, metadata),
            _ => vec![html! {<p>{format!("unhandled: {:?}", self)}</p>}], //TODO
        }
    }
}


pub trait Field {
    fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool;

    fn get_children(&self) -> &[FieldContent];

    fn render_self(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
        metadata: &RuntimeMetadataV14,
    ) -> Html;
    
    fn render_children(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
        metadata: &RuntimeMetadataV14,
    ) -> Vec<Html> {
        let mut out = Vec::new();
        for (index, child) in self.get_children().iter().enumerate() {
            let mut address = parent.clone();
            address.push(index);
            out.append(&mut child.get_cards(address, callback_original.clone(), metadata));
        }
        out
    }

    fn render(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
        metadata: &RuntimeMetadataV14,
    ) -> Vec<Html> {
        let mut out = Vec::new();
        let callback = callback_original.clone();
        out.push(self.render_self(parent.clone(), callback, metadata));
        out.append(&mut self.render_children(parent, callback_original, metadata));
        out
    }

}

