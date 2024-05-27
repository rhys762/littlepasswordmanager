<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Setup from "./components/Setup.vue";
import Login from "./components/Login.vue";
import ViewPassords from "./components/ViewPassords.vue";
import CreatePassword from "./components/CreatePassword.vue";
import Toast from "./components/Toast.vue";

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "./Pages";

let page = ref(Pages.NONE);

let show_toast = ref(false);
let toast_text = ref("");

invoke("get_master_hash", {}).then((s) => {
    const str = s as string;
    console.log("got master hash", str);

    if (str.length) {
        page.value = Pages.LOGIN;
    } else {
        page.value = Pages.SETUP;
    }
});

let toast_id = 0;
function next_toast_id() {
    toast_id += 1;
    return toast_id;
}

function redirect(p: Pages) {
    page.value = p;
}

function toast(str: string) {
    const id = next_toast_id();
    show_toast.value = false;
    setTimeout(() => {
        toast_text.value = str;
        show_toast.value = true;
        setTimeout(() => {
            if (toast_id == id) {
                show_toast.value = false;
            }
        }, 5000);
    }, 100);
}

</script>

<template>
    <div class="container">
        <Setup v-if="page == Pages.SETUP" v-on:redirect="redirect" v-on:toast="toast"></Setup>
        <Login v-if="page == Pages.LOGIN" v-on:redirect="redirect" v-on:toast="toast"></Login>
        <ViewPassords v-if="page == Pages.VIEW_PASSWORDS" v-on:redirect="redirect" v-on:toast="toast"></ViewPassords>
        <CreatePassword v-if="page == Pages.CREATE_PASSWORD" v-on:redirect="redirect" v-on:toast="toast"></CreatePassword>
    </div>
    <Toast :text="toast_text" :show="show_toast"></Toast>
</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
