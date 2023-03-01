use yew::prelude::*;
use web_sys::{EventTarget, HtmlSelectElement, WebSocket};
use yew::{AttrValue, events, platform::{pinned::mpsc::UnboundedSender, spawn_local, time::{interval, sleep}}};
use std::time::Duration;
use futures::{Stream, StreamExt};
use jsonrpsee::wasm_client::WasmClientBuilder;
use jsonrpsee::{rpc_params};
use serde_json::value::Value;
use jsonrpsee::core::client::ClientT;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataV14, PalletMetadata};
use parity_scale_codec::Decode;
use wasm_bindgen::JsCast;
use scale_info::{TypeDef, Type, Variant};

pub struct FieldItem {
    value: u8,
}

pub enum Msg {
    Modify(u8),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: Type,
}

impl Component for FieldItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {value: 0}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <p>{"lol"}</p>
        }
    }
}
