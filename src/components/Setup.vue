<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "../Pages";

const emit = defineEmits(["redirect"]);

const password = ref("");
const confirmPassword = ref("");
const err = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

async function setup() {
    err.value = await invoke("setup_master_password", {password: password.value, confirmPassword: confirmPassword.value});

    if (err.value) {
        confirmPassword.value = "";
    } else {
        emit("redirect", Pages.LOGIN);
    }
}
</script>

<template>
    <div class="container">
        <h1>Setup</h1>
        <div>Create a password for the password manager. It will be required every time you open the app.</div>

        <div>No method for password recovery exists.</div>

        <p class="error"> {{ err }} </p>

        <form @submit.prevent="setup">
            <input id="password-input" type="password" v-model="password" placeholder="Password" /> <br>
            <input id="confirm-password-input" type="password" v-model="confirmPassword" placeholder="Confirm Password" /> <br>
            <button type="submit">Create</button>
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