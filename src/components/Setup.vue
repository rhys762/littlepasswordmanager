<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from 'vue-router';
import { Binding } from "../binds";

const emit = defineEmits(["toast"]);

const password = ref("");
const confirmPassword = ref("");

const router = useRouter();

async function setup() {
    const error = await Binding.setup_master_password(password.value, confirmPassword.value);

    if (error) {
        confirmPassword.value = "";
        emit("toast", error);
    } else {
        router.push("/login");
    }
}
</script>

<template>
    <div class="container">
        <h1>Setup</h1>
        <div>Create a password for the password manager. It will be required every time you open the app.</div>

        <div>No method for password recovery exists.</div>

        <form @submit.prevent="setup">
            <input id="password-input" type="password" v-model="password" placeholder="Password" /> <br>
            <input id="confirm-password-input" type="password" v-model="confirmPassword" placeholder="Confirm Password" /> <br>
            <button type="submit">Create</button>
        </form>
    </div>
</template>

<style scoped>

form * {
    margin: 10px;
}

</style>