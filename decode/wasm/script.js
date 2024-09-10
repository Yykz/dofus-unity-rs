import init, { decode } from './pkg/wasm.js';

(async () => {
    await init();
    document.getElementById('decodeButton').addEventListener('click', function() {
        const protocol = document.getElementById('protocolSelect').value;
        const inputText = document.getElementById('inputText').value;

        document.getElementById('outputText').innerText = decode(protocol, inputText);
    });
})();