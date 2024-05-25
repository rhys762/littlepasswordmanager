<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "../Pages";

const emit = defineEmits(["redirect"]);

const domain = ref("");
const password = ref("");

function back() {
    emit("redirect", Pages.VIEW_PASSWORDS);
}

async function create_password() {
    console.log("calling create_password");

    if (domain.value.length == 0) {
        return;
    }

    await invoke("create_password", {
        domain: domain.value,
        password: password.value
    });

    emit("redirect", Pages.VIEW_PASSWORDS);
}

async function generate_password() {
    console.log("calling generate_password");
    password.value = await invoke("generate_password");
}

</script>

<template>
    <button @click="back">Back</button>
    <div class="container">
        <!-- <form @submit.prevent="create_password"> -->
            <input v-model="domain" placeholder="Domain">
            <input v-model="password" placeholder="password"> <button @click="generate_password">Generate</button>
            <button type="submit" @click="create_password">Create</button>
        <!-- </form> -->
    </div>
</template>

<style scoped>

</style>
