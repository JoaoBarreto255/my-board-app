import { createApp } from "vue";

import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/dist/js/bootstrap.bundle.min.js';
import './assets/app.css';
import { library } from "@fortawesome/fontawesome-svg-core";
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import App from "./App.vue";

library.add(faBars);
const app = createApp(App);
app.component('font-awesome-icon', FontAwesomeIcon);

app.mount("#app");
