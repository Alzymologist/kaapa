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

use crate::field_content::{Field, FieldContent};
use crate::messaging::{CallConstructorEvent, CCEI};

#[derive(Debug)]
pub struct Composite {
    name: Option<String>,
    compact: bool, // TODO: this is useless probably
    fields: Vec<FieldContent>,
}

impl Composite {
    pub fn resolve(
        input: &TypeDefComposite<PortableForm>,
        name: Option<&str>,
        compact: bool,
        metadata: &RuntimeMetadataV14,
    ) -> Self {
        let name = match name {
            Some(a) => Some(a.to_string()),
            None => None,
        };
        Composite {
            name: name,
            compact: compact,
            fields: input
                .fields()
                .iter()
                .map(|a| {
                    FieldContent::new(
                        metadata.types.resolve(a.ty().id()),
                        a.name().map(|x| &**x),
                        false,
                        metadata,
                    )
                })
                .collect(),
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &**x)
    }

    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        self.fields.get_mut(*id)
    }

}

impl Field for Composite {
    fn handle_event(&mut self, _ccei: CCEI, _metadata: &RuntimeMetadataV14) -> bool {
        false
    }

    fn get_children(&self) -> &[FieldContent] {
        &self.fields
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
        Vec::new()
    }
}
