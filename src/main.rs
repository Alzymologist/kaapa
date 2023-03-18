use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use futures::{Stream, StreamExt};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::wasm_client::WasmClientBuilder;
use parity_scale_codec::{Decode, Encode};
use scale_info::{form::PortableForm, Type, TypeDef, Variant};
use serde_json::value::Value;
use sp_core::H256;
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

mod field_content;
mod messaging;
use messaging::CallConstructorEvent;

mod call_constructor;
use call_constructor::CallConstructor;

mod extensions;

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
    metadata: Vec<RuntimeMetadataV14>,
    construction: Option<CallConstructor>,
    genesis_hash: Option<H256>
}

enum Msg {
    NewBlock(String),
    PublishMetadata(RuntimeMetadataV14),
    PublishError(String),
    PublishGenesisHash(H256),
    CallConstructorEvent(CallConstructorEvent),
}

impl App {
    fn get_metadata(&self) -> Option<&RuntimeMetadataV14> {
        self.metadata.last()
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
            publish_metadata.emit(metadata);
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
            genesis_hash: None,
            construction: None::<CallConstructor>,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewBlock(a) => {
                self.last_block = a;
                true
            },
            Msg::PublishMetadata(a) => {
                self.metadata.push(a.clone());
                self.construction = Some(CallConstructor::new(a));
                true
            },
            Msg::PublishGenesisHash(a) => {
                self.genesis_hash = Some(a);
                true
            },
            Msg::PublishError(e) => {
                self.last_error.push(e);
                true
            },
            Msg::CallConstructorEvent(a) => match self.construction {
                Some(ref mut b) => b.handle_event(a),
                None => false,
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cards = match self.construction {
            Some(ref a) => a.get_cards(ctx.link().callback(Msg::CallConstructorEvent)),
            None => Vec::new(),
        };

        let extension_cards = match self.construction {
            Some(ref a) => a.get_extension_cards(ctx.link().callback(Msg::CallConstructorEvent)),
            None => Vec::new(),
        };

        let extension_additional_cards = match self.construction {
            Some(ref a) => a.get_extension_additional_cards(ctx.link().callback(Msg::CallConstructorEvent)),
            None => Vec::new(),
        };


        let encoded = match self.construction {
            Some(ref a) => a.encoded(),
            None => Vec::new(),
        };
        let encoded_call = match self.construction {
            Some(ref a) => a.encoded_call(),
            None => Vec::new(),
        };


        let mut parsed_cards = Vec::new();
        if let Some(metadata) = self.metadata.last() {
            if let Ok(a) = &substrate_parser::MarkedData::mark(&encoded_call) {
                if let Ok(parsed) = substrate_parser::decode_as_call(a, metadata) {
                    parsed_cards = parsed.card(0, &substrate_parser::ShortSpecs{base58prefix: 0, decimals: 1, name: "lolname".to_string(), unit: "huyunit".to_string()});
                }
            }
        }
    
        let mut parsed_ext_cards = Vec::new();
        if let Some(metadata) = self.metadata.last() {
            if let Ok(a) = &substrate_parser::MarkedData::mark(&encoded) {
                if let Ok(parsed) = substrate_parser::unchecked_extrinsic::decode_as_unchecked_extrinsic(&encoded, metadata) {
                    parsed_ext_cards = match parsed {
                        substrate_parser::unchecked_extrinsic::UncheckedExtrinsic::Signed{address: _, signature: _, extra: _, call: a} => a.card(0, &substrate_parser::ShortSpecs{base58prefix: 0, decimals: 1, name: "lolname".to_string(), unit: "huyunit".to_string()}),
                        substrate_parser::unchecked_extrinsic::UncheckedExtrinsic::Unsigned{call: a} => a.card(0, &substrate_parser::ShortSpecs{base58prefix: 0, decimals: 1, name: "lolname".to_string(), unit: "huyunit".to_string()}),
                    }
                }
            }
        }

        let errors = self.last_error.clone();
                    let queried = 6;

        html! {
            <div>
                <p>{&self.last_block}</p>
                <p>{"ERRORS:"}</p>
                <ul class = "item-list">
                    { errors.iter().collect::<Html>() }
                </ul>

                <p>{"CALL:"}</p>
                <ul class = "item-list">
                    { cards }
                </ul>
                <p>{"EXTENSIONS:"}</p>
                <ul class = "item-list">
                    { extension_cards }
                </ul>
                <p>{"EXTENSIONS ADDITIONAL:"}</p>
                <ul class = "item-list">
                    { extension_additional_cards }
                </ul>
                <p>{"did you mean this?"}</p>
                <p>{hex::encode(encoded.clone())}</p>
                <p>{"AS CALL"}</p>
                <ul class = "item-list">
                    { parsed_cards.iter().map(|a| html!{<p>{format!("{}\n", a.show())}</p>}).collect::<Html>() }
                </ul>
                <p>{"AS EXTRINSIC"}</p>
                <ul class = "item-list">
                    { parsed_ext_cards.iter().map(|a| html!{<p>{format!("{}\n", a.show())}</p>}).collect::<Html>() }
                </ul>

                <p>{"DEBUG AREA"}</p>
                {
                    if let Some(metadata) = self.metadata.last() {
                html!{
                    <>
                        <p>{format!("{:?}", metadata.extrinsic)}</p>
                        <p>{format!("type {} is {:?}", queried, metadata.types.resolve(queried))}</p>
                    </>}
                    } else {html!{}}
                }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
