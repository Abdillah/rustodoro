import { element, createApp } from 'deku';
import App from './component/App';

let store = new wasm.Store;

let container: HTMLElement | null = document.body.querySelector('#app');
if (container != null) {
    createApp(container, store.dispatch)(
        <App />
    );
} else {
    console.error("Did you forget to add #app container?")
}
