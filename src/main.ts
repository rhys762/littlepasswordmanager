import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createRouter, createMemoryHistory } from "vue-router";

import Setup from "./components/Setup.vue";
import Login from "./components/Login.vue";
import ViewPassords from "./components/ViewPassords.vue";
import CreatePassword from "./components/CreatePassword.vue";
import Delete from "./components/Delete.vue";

const routes = [
    {
        path: "/view",
        component: ViewPassords
    },
    {
        path: "/setup",
        component: Setup
    },
    {
        path: "/login",
        component: Login
    },
    {
        path: "/create_password/:domain",
        component: CreatePassword
    },
    {
        path: "/create_password",
        component: CreatePassword
    },
    {
        path: "/delete/:domain",
        component: Delete
    }
];

export const router = createRouter({
    history: createMemoryHistory(),
    routes
});

const app = createApp(App);
app.use(router);
app.mount("#app");
