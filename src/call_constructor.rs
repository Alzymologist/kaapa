use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use futures::{Stream, StreamExt};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::wasm_client::WasmClientBuilder;
use parity_scale_codec::Decode;
use scale_info::{
    form::PortableForm, Type, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact,
    TypeDefComposite, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, Variant,
};
use serde_json::value::Value;
use std::time::Duration;
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

use crate::field_content::pallet_call::PalletCall;
use crate::field_content::FieldContent;
use crate::messaging::CallConstructorEvent;

pub struct CallConstructor {
    metadata: RuntimeMetadataV14,
    content: FieldContent,
}

impl CallConstructor {
    pub fn new(metadata: RuntimeMetadataV14) -> Self {
        CallConstructor {
            metadata: metadata,
            content: FieldContent::PalletCall(PalletCall::new()),
        }
    }

    pub fn get_cards(&self, callback_original: Callback<CallConstructorEvent>) -> Vec<Html> {
        self.content
            .get_cards(Vec::new(), callback_original, &self.metadata)
    }

    pub fn handle_event(&mut self, event: CallConstructorEvent) -> bool {
        let mut field = &mut self.content;
        for id in event.index.iter() {
            match field.get_child(id) {
                Some(a) => field = a,
                None => return false,
            }
        }
        field.handle_event(event.ccei, &self.metadata);
        true
    }

    pub fn encoded(&self) -> Vec<u8> {
        self.content.encoded(&self.metadata)
    }
}
