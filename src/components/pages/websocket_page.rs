use {
    crate::{
        components::pages::dashboard_page::get_ring_values, integrations::ring::types::RingValues,
    },
    js_sys::Reflect,
    leptos::*,
    leptos_reactive::create_effect,
    leptos_use::{core::ConnectionReadyState, use_websocket, UseWebsocketReturn},
    serde_json::json,
    wasm_bindgen::{closure::Closure, JsValue},
    web_sys::RtcPeerConnection,
};

#[component]
pub fn WebSocketPage() -> impl IntoView {
    let ring_values = create_resource(|| (), |_| get_ring_values());

    view! {
        <h1>"WebSocket"</h1>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                ring_values
                    .get()
                    .map(|ring_values| {
                        ring_values
                            .map(|ring_values| {
                                view! { <WebSocketComponent ring_values=ring_values/> }
                            })
                    })
            }}

        </Suspense>
    }
}

#[component]
fn WebSocketComponent(ring_values: RingValues) -> impl IntoView {
    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket(&ring_values.ws_url);

    let send_message = move |_| {
        send("Hello, world!");
    };

    // let send_byte_message = move |_| {
    //     send_bytes(b"Hello, world!\r\n".to_vec());
    // };

    create_effect(move |_| {
        if ready_state.get() == ConnectionReadyState::Open {
            let send_bytes = send_bytes.clone();
            let pc = RtcPeerConnection::new().unwrap();
            let create_offer_callback = Closure::wrap(Box::new(move |offer: JsValue| {
                let sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))
                    .unwrap()
                    .as_string()
                    .unwrap();
                send_bytes(
                    serde_json::to_vec(&json!({
                        "method": "live_view",
                        "dialog_id": "333333",
                        "body": {
                            "doorbot_id": "",
                            "stream_options": { "audio_enabled": true, "video_enabled": true },
                            "sdp": sdp,
                        }
                    }))
                    .unwrap(),
                );
            }) as Box<dyn FnMut(JsValue)>);
            let _ = pc.create_offer().then(&create_offer_callback);
            create_offer_callback.forget();
        }
    });
    let status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
    };

    let close_connection = move |_| {
        close();
    };

    view! {
        <div>
            <p>"status: " {status}</p>

            <button on:click=send_message disabled=move || !connected()>
                "Send"
            </button>
            // <button on:click=send_byte_message disabled=move || !connected()>
            // "Send bytes"
            // </button>
            <button on:click=open_connection disabled=connected>
                "Open"
            </button>
            <button on:click=close_connection disabled=move || !connected()>
                "Close"
            </button>

            <h2>"Receive message: "</h2>
            <pre style="text-wrap: wrap; word-break: break-all;">
                {move || format!("{:?}", message.get())}
            </pre>
            <p>"Receive byte message: " {move || format!("{:?}", message_bytes.get())}</p>
        </div>
    }
}
