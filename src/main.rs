use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use futures::{Stream, StreamExt};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::wasm_client::WasmClientBuilder;
use parity_scale_codec::Decode;
use scale_info::{form::PortableForm, Type, TypeDef, Variant};
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
    AttrValue,
};

mod field;
use field::FieldItem;

const ONE_SECOND: Duration = Duration::from_secs(1);

pub fn unhex(hex_input: &str) -> Vec<u8> {
    let hex_input_trimmed = {
        if let Some(hex_input_stripped) = hex_input.strip_prefix("0x") {
            hex_input_stripped
        } else {
            hex_input
        }
    };
    hex::decode(hex_input_trimmed).unwrap()
}

enum ClientStatus {
    NonInit,
    Init,
    Error,
}

enum Request {
    GetBlock,
}

struct App {
    input: UnboundedSender<Request>,
    last_error: Vec<String>,
    last_block: String,
    metadata: Vec<(u32, RuntimeMetadataV14)>,
    pallet: Option<usize>,
    call: Option<usize>,
}

enum Msg {
    NewBlock(String),
    PublishMetadata((u32, RuntimeMetadataV14)),
    SelectPallet(usize),
    SelectCall(usize),
    PublishError(String),
}

impl App {
    fn get_metadata(&self) -> Option<&RuntimeMetadataV14> {
        match self.metadata.last() {
            Some((_, a)) => Some(a),
            None => None,
        }
    }

    fn resolve_ty(&self, id: u32) -> Option<&Type<PortableForm>> {
        match self.get_metadata() {
            Some(a) => a.types.resolve(id),
            None => None,
        }
    }

    fn get_pallets(&self) -> Vec<PalletMetadata<PortableForm>> {
        match self.get_metadata() {
            Some(a) => a.pallets.clone(),
            None => Vec::new(),
        }
    }

    fn get_pallet(&self) -> Option<&PalletMetadata<PortableForm>> {
        match self.get_metadata() {
            Some(a) => match self.pallet {
                Some(n) => match a.pallets.get(n) {
                    Some(b) => Some(&b),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }

    fn get_calls(&self) -> Vec<Variant<PortableForm>> {
        match self.get_pallet() {
            Some(a) => match a.calls {
                Some(ref calls_def) => {
                    let id = calls_def.ty.id();
                    match self.resolve_ty(id) {
                        Some(type_struct) => match type_struct.type_def() {
                            TypeDef::Variant(calls) => calls.variants().to_vec(),
                            _ => Vec::new(),
                        },
                        None => Vec::new(),
                    }
                }
                None => Vec::new(),
            },
            None => Vec::new(),
        }
    }

    fn get_call(&self) -> Option<Variant<PortableForm>> {
        match self.call {
            Some(n) => self.get_calls().get(n).cloned(),
            None => None,
        }
    }

    fn get_fields(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self.get_call() {
            Some(call) => {
                for call_field in call.fields() {
                    self.resolve_ty(call_field.ty().id());
                }
            },
            None => {},
        };
        out
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let new_block = ctx.link().callback(Msg::NewBlock);
        let (tx, mut rx) = yew::platform::pinned::mpsc::unbounded::<Request>();
        let publish_error = ctx.link().callback(Msg::PublishError);
        let publish_metadata = ctx.link().callback(Msg::PublishMetadata);
        let pe = publish_error.clone();

        //All communication with node is here
        spawn_local(async move {
            let client = WasmClientBuilder::default()
                .build("ws://127.0.0.1:9944")
                //.build("wss://westend-rpc.polkadot.io:443")
                .await
                .unwrap();
            let test: Value = client
                .request("chain_getBlockHash", rpc_params![])
                .await
                .unwrap();
            let mut last_block = String::new();
            let mut metadata_value: Value = client
                .request("state_getMetadata", rpc_params![])
                .await
                .unwrap();
            let metadata = if let Value::String(hex_meta) = metadata_value {
                let meta = unhex(&hex_meta);
                if !meta.starts_with(&[109, 101, 116, 97]) {
                    publish_error.emit(String::from("Wrong start"));
                    panic!();
                }
                match RuntimeMetadata::decode(&mut &meta[4..]) {
                    Ok(RuntimeMetadata::V14(out)) => out,
                    Ok(_) => panic!(),
                    Err(_) => panic!(),
                }
            } else {
                panic!();
            };
            publish_metadata.emit((0, metadata));
            while let Some(request) = rx.next().await {
                match request {
                    Request::GetBlock => {
                        match client.request("chain_getBlockHash", rpc_params![]).await {
                            Ok(Value::String(a)) => {
                                if last_block != a {
                                    last_block = a.clone();
                                    new_block.emit(a);
                                }
                            }
                            Ok(_) => publish_error.emit("invalid block format".to_string()),
                            Err(e) => publish_error.emit(format!("{e}\n")),
                        }
                    }
                }
            }
        });

        let block_requester = tx.clone();
        let publish_error = ctx.link().callback(Msg::PublishError);
        //Block number fetcher
        spawn_local(async move {
            loop {
                match block_requester.send_now(Request::GetBlock) {
                    Ok(_) => (),
                    Err(e) => publish_error.emit(e.to_string()),
                };
                sleep(ONE_SECOND).await
            }
        });

        Self {
            input: tx,
            last_error: Vec::new(),
            last_block: "none".to_string(),
            metadata: Vec::new(),
            pallet: None,
            call: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewBlock(a) => {
                self.last_block = a;
                true
            }
            Msg::PublishMetadata(a) => {
                self.metadata.push(a);
                true
            }
            Msg::SelectPallet(n) => {
                self.pallet = Some(n);
                self.call = None;
                true
            }
            Msg::SelectCall(n) => {
                self.call = Some(n);
                true
            }
            Msg::PublishError(e) => {
                self.last_error.push(e);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let errors = self.last_error.clone();
        let pallets = self.get_pallets();
        /*match self.metadata.last() {
            Some((_, a)) => {
                a.pallets.clone()//.iter().map(|p| p.name.to_string()).collect()
            },
            None => Vec::new()
        };*/

        let pallet_select_callback_event = ctx.link().callback(Msg::SelectPallet);

        let pallet_select_callback = Callback::from(move |e: Event| {
            let target: EventTarget = e.target().unwrap();
            let item = target.unchecked_into::<HtmlSelectElement>().value();
            pallet_select_callback_event.emit(item.parse::<usize>().unwrap());
        });

        html! {
            <div>
                <p>{&self.last_block}</p>
                <p>{"ERRORS:"}</p>
                <ul class = "item-list">
                    { errors.iter().collect::<Html>() }
                </ul>
                
                //Select pallet
                
                <select onchange = {pallet_select_callback}>
                    <option disabled = true selected = {self.pallet.is_none()} >{ "-" }</option>
                    {
                        pallets.iter().enumerate().map(|(n, p)| {
                            html!{<option value = {n.to_string()}>{p.name.to_string()}</option>}
                        }).collect::<Html>()
                    }
                </select>

                //Select call
                
                {
                    if self.pallet.is_some() {
                        let calls = self.get_calls();
                        let call_select_callback_event = ctx.link().callback(Msg::SelectCall);
                        let call_select_callback = Callback::from(move |e: Event| {
                            let target: EventTarget = e.target().unwrap();
                            let item = target.unchecked_into::<HtmlSelectElement>().value();
                            call_select_callback_event.emit(item.parse::<usize>().unwrap());
                        });

                        html! {
                            <>
                                <select onchange = {call_select_callback}>
                                    <option disabled = true selected = {self.call.is_none()} >{ "-" }</option>
                                    {
                                        calls.iter().enumerate().map(|(n, p)| {
                                            html!{<option value = {n.to_string()}>{p.name.to_string()}</option>}
                                        }).collect::<Html>()
                                    }
                                </select>

                                //Call parameters

                                {
                                    if let Some(this_call) = self.get_call() {
                                        html! {
                                            <>
                                                <p>{this_call.docs().join("\n")}</p>
                                                <ul class = "item-list">
                                                {
                                                    for this_call.fields().iter().map(|a| html!{
                                                        <>
                                                            <p>{a.name()}</p>
                                                            {
                                                                if let Some(field) =  self.resolve_ty(a.ty().id()) {
                                                                    html!{
                                                                        <>
                                                                            //<FieldItem content = {poeben}/>
                                                                            <p>{field.docs().join("\n")}</p>
                                                                            {
                                                                                match field.type_def() {
                                                                                TypeDef::Composite(type_def_composite) => {
                                                                                    if let Some(b) = type_def_composite.fields().first() {
                                                                                        if let Some(t_field) = self.resolve_ty(b.ty().id()) {
                                                                                            html!{
                                                                                                <>
                                                                                                    <p>{"composite"}</p>
                                                                                                    <p>{format!("{:?}", t_field.type_def())}</p>
                                                                                                </>
                                                                                            }
                                                                                        } else {html!{<p>{"lol"}</p>}}
                                                                                    } else {html!{<p>{"notfound"}</p>}}
                                                                                },
                                                                                _ => html!{<p>{format!("{:?}", field.type_def())}</p>},
                                                                                }
                                                                            }
                                                                        </>
                                                                    }
                                                                } else {
                                                                    html!{<p>{"Field "}</p>}
                                                                }
                                                            }
                                                        </>
                                                    })
                                                }
                                                </ul>
                                            </>
                                        }
                                    } else {
                                        html!{<p>{"Select call"}</p>}
                                    }
                                }
                            </>
                        }   
                    } else {
                        html! {<p>{"Select palet"}</p>}
                    }
                }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
