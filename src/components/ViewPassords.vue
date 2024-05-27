<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Pages } from "../Pages";

const emit = defineEmits(["redirect", "toast"]);

interface Password {
    domain: string,
    password: string
}

const passwords = ref([] as Password[]);
const filter = ref("");

// retrieve passwords
function get_passwords() {
    invoke("get_passwords", {
        filter: filter.value
    }).then(p => {
        passwords.value = p as Password[];
    })
}

function copy(text: string) {
    navigator.clipboard.writeText(text)
    .then(() => {
        emit("toast", "Copied to clipboard");
    });
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

        <div class="filter">
            <span class="material-symbols-outlined search">search</span>
            <input v-model="filter" placeholder="filter" @input="get_passwords"/>
        </div>

        <div class="card" v-for="password in passwords">
            <div class="domain"> {{ password.domain }}</div>

            <div class="spacer"></div>

            <div class="password"> {{ password.password }}</div>
            <span @click="copy(password.password)" class="copy material-symbols-outlined" title="Copy">content_copy</span>
            <span class="material-symbols-outlined edit" title="Edit">edit</span>

        </div>
    </div>
</template>

<style scoped>

hr {
    margin: 5px;
}

button {
    width: fit-content;
}

.container {
    display: flex;
    align-items: center;
    justify-content: start;
}

.filter {
    display: flex;
    background-color: white;
    border-radius: 20px;
    margin-top: 10px;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    padding: 3px;
    align-items: center;
    width: fit-content;
}

.filter input {
    border: 0px;
    box-shadow: none;
}

.search {
    color: #777;
}

.card {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    width: 100%;
    max-width: 900px;
    margin-top: 10px;

    display: flex;
    align-items: center;
    justify-content: center;
}

.card .domain {
    /* display: inline-block; */
    margin-right: 40px;
    margin-right: auto;
}

.card .spacer {
    /* display: inline-block; */
    width: 20px;
}

.card .password {
    /* display: inline-block; */
    /* margin-left: auto; */
    color: transparent;
    background-color: #eeeeee;
    border-radius: 4px;
    padding-right: 10px;
    padding-left: 10px;
}


.copy {
    color: #777;
    margin-left: 10px;
    cursor: pointer;
}

.edit {
    color: #777;
    margin-left: 10px;
    cursor: pointer;
}

.card .password:hover {
    color: black;
    background-color: transparent;
}

</style>