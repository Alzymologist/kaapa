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
pub struct VariantContent {
    name: Option<String>,
    compact: bool, //useless, maybe remove? variant is always compact.
    variants: Vec<Variant<PortableForm>>,
    selected: Option<Variant<PortableForm>>,
    fields: Vec<FieldContent>,
}

impl VariantContent {
    pub fn resolve(input: &TypeDefVariant<PortableForm>, name: Option<&str>, compact: bool, metadata: &RuntimeMetadataV14) -> Self {
        let name = match name {
            Some(a) => Some(a.to_string()),
            None => None,
        };
        VariantContent {
            name: name,
            compact: compact,
            variants: input.variants().to_vec(),
            selected: None,
            fields: Vec::new(),
        }
    }

    pub fn set_variant(&mut self, index: u8, metadata: &RuntimeMetadataV14) {
        for variant in self.variants.iter() {
            if variant.index() == index {
                self.selected = Some(variant.clone());
                self.fields = variant.fields.iter().map(|a| {FieldContent::new(metadata.types.resolve(a.ty().id()), a.name().map(|x| &**x), false, metadata)}).collect();
                break;
            }
        }
    }

    pub fn name(&self) -> String {
        match &self.name {
            Some(a) => a.to_string(),
            None => String::new(),
        }
    }

    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        self.fields.get_mut(*id)
    }
}

impl Field for VariantContent {
    fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool {
        if let Ok(index) = ccei.message.parse::<u8>() {
            match ccei.id {
                0 => self.set_variant(index, metadata),
                _ => return false,
            }
            true
        } else {
            false
        }
    }

    fn get_children(&self) -> &[FieldContent] {
        &self.fields
    }

    fn render_self(&self, parent: Vec<usize>, callback_original: Callback<CallConstructorEvent>, metadata: &RuntimeMetadataV14,) -> Html {
        let select_callback = Callback::from(move |e: Event| {
            let target: EventTarget = e.target().unwrap();
            let item = target.unchecked_into::<HtmlSelectElement>().value();
            callback_original.emit(CallConstructorEvent {
                index: parent.clone(),
                ccei: CCEI {
                    id: 0,
                    message: item,
                },
            });
        });

        html! {
            <>
                <p>{self.name()}</p>
                <select onchange = {select_callback}>
                    <option disabled = true selected = {self.selected.is_none()} >{ "-" }</option>
                    {
                        self.variants.iter().enumerate().map(|(n, p)| {
                            html!{<option value = {p.index.to_string()}>{p.name.to_string()}</option>}
                        }).collect::<Html>()
                    }
                </select>
                {
                    if let Some(ref a) = self.selected {
                        html! {<p>{a.docs().join("\n")}</p> }
                    } else {
                        html! {<p>{"select variant"}</p> }
                    }
                }
            </>
        }
    }

    fn encoded_self(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(ref a) = self.selected {
            out.push(a.index);
        }
        out
    }
}

