import './assets/main.css'
import 'primeicons/primeicons.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice';
import Noir from './presets/Noir.ts';

const app = createApp(App)
const pinia = createPinia()
app.use(pinia)
app.use(ToastService);
app.use(PrimeVue, {
    ripple: true ,
    inputVariant: "filled",
    theme: {
        preset: Noir,
        options: {
            prefix: 'p',
            darkModeSelector: '.p-dark',
            cssLayer: false,
        }
    }
});

app.use(router)

app.mount('#app')
