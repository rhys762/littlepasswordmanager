<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "../Pages";

const emit = defineEmits(["redirect"]);

interface Password {
    domain: string,
    password: string
}

const passwords = ref([] as Password[]);
const err = ref("");

// retrieve passwords
async function get_passwords() {
    passwords.value = await invoke("get_passwords", {});
}

onMounted(() => {
    console.log("mounted.");
    get_passwords();
});

</script>

<template>
    <div class="container">
        <h1>Passwords</h1>
        <button @click="emit('redirect', Pages.CREATE_PASSWORD)">Add new Passord</button>
        
        <hr>

        <div v-for="password in passwords">
            <p> {{ password.domain }}</p>
            <p> {{ password.password }}</p>
        </div>
    </div>
</template>

<style scoped>

hr {
    margin: 5px;
}

</style>