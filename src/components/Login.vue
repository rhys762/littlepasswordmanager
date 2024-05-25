<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "../Pages";

const emit = defineEmits(["redirect"]);

const password = ref("");
const err = ref("");

async function login() {
    err.value = await invoke("login", {password: password.value});

    if (err.value) {
        password.value = "";
    } else {
        emit("redirect", Pages.VIEW_PASSWORDS);
    }
}

</script>

<template>
    <div class="container">
        <h1>Login</h1>

        <p class="error"> {{ err }} </p>

        <form @submit.prevent="login">
            <label for="password-input">Password</label> <input id="password-input" v-model="password" /> <br>
            <button type="submit">Login</button>
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