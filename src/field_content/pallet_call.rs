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
pub struct PalletCall {
    pallet: Option<PalletMetadata<PortableForm>>,
    call: Option<Variant<PortableForm>>,
    fields: Vec<FieldContent>,
}

impl PalletCall {
    pub fn new() -> Self {
        PalletCall {
            pallet: None,
            call: None,
            fields: Vec::new(),
        }
    }

    pub fn get_pallets(&self, metadata: &RuntimeMetadataV14) -> Vec<PalletMetadata<PortableForm>> {
        metadata.pallets.clone()
    }

    pub fn set_pallet(&mut self, n: u8, metadata: &RuntimeMetadataV14) {
        self.pallet = None;
        self.call = None;
        self.fields = Vec::new();
        for pallet in metadata.pallets.iter() {
            if pallet.index == n {
                self.pallet = Some(pallet.to_owned());
                break;
            }
        }
    }

    pub fn get_calls(&self, metadata: &RuntimeMetadataV14) -> Vec<Variant<PortableForm>> {
        match self.try_get_calls(metadata) {
            Some(a) => a.to_vec(),
            None => Vec::new(),
        }
    }

    fn try_get_calls<'a>(
        &self,
        metadata: &'a RuntimeMetadataV14,
    ) -> Option<&'a [Variant<PortableForm>]> {
        if let Some(ref pallet) = self.pallet {
            if let Some(ref calls_def) = pallet.calls {
                let id = calls_def.ty.id();
                if let Some(type_enum) = metadata.types.resolve(id) {
                    if let TypeDef::Variant(calls) = type_enum.type_def() {
                        return Some(calls.variants());
                    }
                }
            }
        }
        return None;
    }

    pub fn set_call(&mut self, n: u8, metadata: &RuntimeMetadataV14) {
        self.call = None;
        self.fields = Vec::new();
        if let Some(calls) = self.try_get_calls(metadata) {
            for call in calls.iter() {
                if call.index == n {
                    self.call = Some(call.to_owned());
                    self.fields = call
                        .fields
                        .iter()
                        .map(|a| {
                            FieldContent::new(
                                metadata.types.resolve(a.ty().id()),
                                a.name().map(|x| &**x),
                                false,
                                metadata,
                            )
                        })
                        .collect();
                    break;
                }
            }
        }
    }

    
    pub fn get_child(&mut self, id: &usize) -> Option<&mut FieldContent> {
        self.fields.get_mut(*id)
    }
}

impl Field for PalletCall {
    fn handle_event(&mut self, ccei: CCEI, metadata: &RuntimeMetadataV14) -> bool {
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
        let index = parent.clone();
        let callback = callback_original.clone();
        let pallet_select_callback = Callback::from(move |e: Event| {
            let target: EventTarget = e.target().unwrap();
            let item = target.unchecked_into::<HtmlSelectElement>().value();
            callback.emit(CallConstructorEvent {
                index: index.clone(),
                ccei: CCEI {
                    id: 0,
                    message: item,
                },
            });
        });

        let index = parent.clone();
        let callback = callback_original.clone();
        let call_select_callback = Callback::from(move |e: Event| {
            let target: EventTarget = e.target().unwrap();
            let item = target.unchecked_into::<HtmlSelectElement>().value();
            callback.emit(CallConstructorEvent {
                index: index.clone(),
                ccei: CCEI {
                    id: 1,
                    message: item,
                },
            });
        });

        html! {
            <>
                <select onchange = {pallet_select_callback}>
                    <option disabled = true selected = {self.pallet.is_none()} >{ "-" }</option>
                    {
                        self.get_pallets(metadata).iter().enumerate().map(|(n, p)| {
                            html!{<option value = {n.to_string()}>{p.name.to_string()}</option>}
                        }).collect::<Html>()
                    }
                </select>
                {
                    if self.pallet.is_some() {
                        html! {
                            <>
                                <select onchange = {call_select_callback}>
                                    <option disabled = true selected = {self.call.is_none()} >{ "-" }</option>
                                    {
                                        self.get_calls(metadata).iter().enumerate().map(|(n, p)| {
                                            html!{<option value = {n.to_string()}>{p.name.to_string()}</option>}
                                        }).collect::<Html>()
                                    }
                                </select>

                                {
                                    if let Some(ref call) = self.call {
                                        html! { <p>{call.docs().join("\n")}</p> }
                                    } else {
                                        html! { <p>{"select call"}</p> }
                                    }
                                }
                            </>
                        }
                    } else {
                        html! {
                            <p> {"select call"} </p>
                        }
                    }
                }
            </>
        }
    }

    fn encoded_self(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(ref a) = self.pallet {
            if let Some(ref b) = self.call {
                out.push(a.index);
                out.push(b.index);
            }
        }
        out
    }
}
