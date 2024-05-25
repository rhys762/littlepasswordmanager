<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Setup from "./components/Setup.vue";
import Login from "./components/Login.vue";
import ViewPassords from "./components/ViewPassords.vue";
import CreatePassword from "./components/CreatePassword.vue";

import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "./Pages";

let page = ref(Pages.NONE);

invoke("get_master_hash", {}).then((s) => {
    const str = s as string;
    console.log("got master hash", str);

    if (str.length) {
        page.value = Pages.LOGIN;
    } else {
        page.value = Pages.SETUP;
    }
})

function redirect(p: Pages) {
    page.value = p;
}

</script>

<template>
    <div class="container">
        <Setup v-if="page == Pages.SETUP" v-on:redirect="redirect"></Setup>
        <Login v-if="page == Pages.LOGIN" v-on:redirect="redirect"></Login>
        <ViewPassords v-if="page == Pages.VIEW_PASSWORDS" v-on:redirect="redirect"></ViewPassords>
        <CreatePassword v-if="page == Pages.CREATE_PASSWORD" v-on:redirect="redirect"></CreatePassword>
    </div>
</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
