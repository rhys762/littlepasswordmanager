<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "../Pages";
import Spinner from "./Spinner.vue";
const emit = defineEmits(["redirect"]);

const password = ref("");
const err = ref("");
const loading = ref(false);

function login() {
    loading.value = true;

    invoke("login", {password: password.value}).then(e => {
        err.value = e as string;
        

        if (err.value) {
            loading.value = false;
            password.value = "";
        } else {
            // get passwords, backend will decrypt and cache,
            // ready for mount.
            invoke("get_passwords", {
                filter: ""
            }).then(() => {
                emit("redirect", Pages.VIEW_PASSWORDS);
            });
        }
    });
}

</script>

<template>
    <div class="container">
        <h1>Login</h1>

        <p class="error"> {{ err }} </p>

        <form @submit.prevent="login">
            <input type="password" id="password-input" v-model="password" placeholder="password" /> <br>
            <Spinner v-if="loading"></Spinner>
            <button v-else type="submit">Login</button>
        </form>
    </div>
</template>

<style scoped>

.error {
    color: red;
}

form * {
    margin: 10px;
}

</style>