<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter, useRoute } from 'vue-router';
import { Binding } from "../binds";

const domain = ref("");
const password = ref("");
const shown = ref(false);
const domains = ref([] as string[]);

const update = computed(() => domains.value.includes(domain.value));
const confirming_update = ref(false);

const router = useRouter();
const route = useRoute();

const input_type = computed(() => {
    if (shown.value) {
        return "text";
    }
    return "password";
})

function back() {
    router.push("/view");
}

async function create_password() {
    console.log("calling create_password");

    if (domain.value.length == 0) {
        return;
    }

    Binding.create_password(domain.value, password.value);

    router.push("/view");
}

async function generate_password() {
    console.log("calling generate_password");
    password.value = await Binding.generate_password();
}

function toggle_shown() {
    shown.value = !shown.value;
}

function get_passwords() {
    Binding.get_passwords("")
    .then(p => {
        domains.value = p.map(p => p.domain);
    });
}

function toggle_update() {
    confirming_update.value = !confirming_update.value;
}

onMounted(() => {
    get_passwords();

    const dom = route.params.domain;
    if (dom == undefined) {
        return;
    }

    domain.value = decodeURI(dom as string);
});

</script>

<template>
    <div class="container">
        <button class="back" @click="back">Back</button>

        <form class="create" v-if="!confirming_update">
            <input v-model="domain" placeholder="Domain" class="domain">

            <div class="password_line">
                <div class="password_wrap">
                    <input class="password" v-model="password" placeholder="password" :type="input_type">
                    <span class="material-symbols-outlined shown" @click="toggle_shown" title="Show">visibility</span>
                </div>
                <button @click="generate_password" class="generate" type="button">Generate</button>
            </div>

            <button v-if="update" type="submit" @click="toggle_update" class="update_button">Update {{ domain }}</button>
            <button v-else type="submit" @click="create_password">Create</button>
        </form>
        <form class="update" v-else>
            <div class="warning">This will overwrite the password for {{ domain }}, and cannot be undone.</div>
            <button class="yes" @click="create_password">Yes</button>
            <button class="cancel" @click="toggle_update">Cancel</button>
        </form>
    </div>
</template>

<style scoped>
.back {
    width: fit-content;
    margin-bottom: 10px;
}

.container {
    align-items: center;
}

.domain {
    margin-bottom: 10px;
}

.password_line {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 10px;
}

.update_button {
    background-color: orange;
}

.password_wrap {
    display: flex;
    align-items: center;
    background-color: white;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    border-radius: 8px;
    margin-right: 10px;
}

.password {
    box-shadow: none;
}

.shown {
    cursor: pointer;
    color: #777;
    margin-right: 10px;
}

.warning {
    margin-bottom: 10px;
}

button.yes {
    margin-right: 10px;
}

button.cancel {
    background-color: red;
    color: white;
}

</style>
