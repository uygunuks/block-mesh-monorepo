// A static import is required in b/g scripts because they are executed in their own env
// not connected to the content scripts where wasm is loaded automatically
import initWasmModule, {task_poller} from './wasm/blockmesh_ext.js';

console.log("Background script started");

// console.log(await chrome.permissions.getAll());

// run the wasm initializer before calling wasm methods
// the initializer is generated by wasm_pack
(async () => {
    await initWasmModule();
    // setInterval(async () => {
    //     await task_poller().then(onSuccess, onError);
    // }, 5_000);
})();

// A placeholder for OnSuccess in .then
function onSuccess(message) {
    console.log(`Background::Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
    console.error(`Background::Promise error: ${error}`);
}

// A placeholder for OnError in .then
function onErrorWithLog(error) {
    console.error(`Background::Promise error: ${error}`);
}

// Popup button handler
// Fetches the data from Spotify using the creds extracted earlier
chrome.runtime.onMessage.addListener(async (request, sender, sendResponse) => {
    console.log(`Popup message received: ${JSON.stringify(request)}, ${JSON.stringify(sender)}`);
    return true;
    // chrome.runtime.sendMessage("Missing how many tracks to add param. It's a bug.").then(onSuccess, onError);
});