importScripts('./pkg/wasmparser.js');

const { parse_events2, parse_chat_messages, parse_skins } = wasm_bindgen;

// We compiled with `--target no-modules`, which does not create a module. The generated bindings
// can be loaded in web workers in all modern browsers.

async function run_in_worker() {
    // Load the Wasm file by awaiting the Promise returned by `wasm_bindgen`
    await wasm_bindgen('./pkg/wasmparser_bg.wasm');
    console.log("worker.js has loaded Wasm file → Functions defined with Rust are now available");
}

run_in_worker();


onmessage = async function (e) {
    console.log("onmessage inside worker.js runs");
    console.log("event name:" + e.data.event_name);
    
    let result = parse_events2(
        e.data.file,
        e.data.event_name,
    );
    

    /*
    let result = parse_skins(
        e.data.file,
    );
    */

    postMessage(result);
};
