import { createApp } from "vue";
import App from "~/App.vue";
import router from "~/router";
import "virtual:windi.css";
import "~/state";
import "~/state/shortcuts";

createApp(App).use(router).mount("#app");
