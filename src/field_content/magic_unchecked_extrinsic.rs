use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::Encode;
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

use crate::field_content::{Field, FieldContent};
use crate::messaging::{CallConstructorEvent, CCEI};

#[derive(Debug)]
struct Signature {
    address: Box<FieldContent>,
    signature: Box<FieldContent>,
    extra: Box<FieldContent>,
}

#[derive(Debug)]
pub struct MagicUncheckedExtrinsic {
    signature: Option<Signature>,
    fields: Vec<FieldContent>,
}

impl MagicUncheckedExtrinsic {
    pub fn new(
        metadata: &RuntimeMetadataV14,
    ) -> Self {
        let mut call = FieldContent::Error(String::from("call extrinsic parameter not found"));
        if let Some(a) = metadata.types.resolve(metadata.extrinsic.ty.id()) {
            for tp in a.type_params().iter() {
                if tp.name() == "Call" {
                    if let Some(b) = tp.ty() {
                        call = FieldContent::new(
                            metadata.types.resolve(b.id()),
                            Some("Call"),
                            false,
                            metadata
                        );
                        break;
                    }
                }
            }
        }
        MagicUncheckedExtrinsic {
            signature: None,
            fields: vec![call],
        }
    }

    pub fn name(&self) -> Option<&str> {
        Some("Extrinsic")
    }

    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        self.fields.get_mut(*id)
    }
}

impl Field for MagicUncheckedExtrinsic {
    fn handle_event(&mut self, _ccei: CCEI, _metadata: &RuntimeMetadataV14) -> bool {
        false // TODO this should have plenty
    }

    fn get_children(&self) -> &[FieldContent] {
        &self.fields // TODO
    }


    fn render_self(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
        metadata: &RuntimeMetadataV14,
    ) -> Html {
        match self.name() {
                Some(name) => html! {<p>{format!("{}", name)}</p>},
                None => html! {},
            }
    }
    
    fn encoded_self(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(metadata.extrinsic.version + if self.signature.is_some() {0x80} else {0});
        out
    }
}
