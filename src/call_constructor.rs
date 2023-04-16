use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::Encode;
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
use crate::field_content::{Field, FieldContent};
use crate::messaging::CallConstructorEvent;
use crate::field_content::magic_unchecked_extrinsic::MagicUncheckedExtrinsic;
use crate::extensions::ExtensionContent;

pub struct CallConstructor {
    metadata: RuntimeMetadataV14,
    content: FieldContent,
    extensions: Vec<ExtensionContent>,
}

impl CallConstructor {
    pub fn new(metadata: RuntimeMetadataV14) -> Self {
        let extensions = metadata.extrinsic.signed_extensions.iter()
            .map(|a| {
                ExtensionContent::new(a, &metadata)
            }).collect();
        CallConstructor {
            metadata: metadata.clone(),
            content: FieldContent::MagicUncheckedExtrinsic(MagicUncheckedExtrinsic::new(&metadata)),
            extensions: extensions,
        }
    }

    pub fn get_cards(&self, callback_original: Callback<CallConstructorEvent>) -> Vec<Html> {
        self.content
            .get_cards(vec![0], callback_original, &self.metadata)
    }

    pub fn get_extension_cards(&self, callback_original: Callback<CallConstructorEvent>) -> Vec<Html> {
        let mut out = Vec::new();
        for (i, extension) in self.extensions.iter().enumerate() {
            let index = vec![1, i, 0];
            let callback = callback_original.clone();
            out.append(&mut extension.get_content_cards(index, callback, &self.metadata));
        }
        out
    }

    pub fn get_extension_additional_cards(&self, callback_original: Callback<CallConstructorEvent>) -> Vec<Html> {
        let mut out = Vec::new();
        for (i, extension) in self.extensions.iter().enumerate() {
            let index = vec![1, i, 1];
            let callback = callback_original.clone();
            out.append(&mut extension.get_additional_cards(index, callback, &self.metadata));
        }
        out
    }

    pub fn handle_event(&mut self, event: CallConstructorEvent) -> bool {
        let mut index_iter = event.index.iter();
        let mut field = match index_iter.next() {
            Some(0) => &mut self.content,
            Some(1) => match index_iter.next() {
                Some(j) => match self.extensions.get_mut(*j) {
                    Some(a) => match index_iter.next() {
                        Some(0) => a.content(),
                        Some(1) => a.additional(),
                        _ => return false,
                    },
                    None => return false,
                },
                None => return false,
            },
            _ => return false,
        };
        for id in index_iter {
            match field.get_child(id) {
                Some(a) => field = a,
                None => return false,
            }
        }
        field.handle_event(event.ccei, &self.metadata);
        true
    }

    pub fn encoded(&self) -> Vec<u8> {
        self.content.encoded(&self.metadata).encode()
    }

    pub fn encoded_call(&self) -> Vec<u8> {
        if let FieldContent::MagicUncheckedExtrinsic(extr) = &self.content {
            let a = extr.get_children();
            a[0].encoded(&self.metadata).encode()
        } else {
            Vec::new()
        }
    }

    pub fn encoded_extensions(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for extension in self.extensions.iter() {
            out.append(&mut (extension.encoded_content(&self.metadata)));
        }
        for extension in self.extensions.iter() {
            out.append(&mut extension.encoded_additional(&self.metadata));
        }
        out
    }
}
