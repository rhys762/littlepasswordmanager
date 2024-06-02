<script setup lang="ts">
import { ref } from "vue";
import Spinner from "./Spinner.vue";
import { useRouter } from 'vue-router';
import { Binding } from "../binds";

const emit = defineEmits(["toast"]);

const password = ref("");
const loading = ref(false);

const router = useRouter();

function login() {
    loading.value = true;

    Binding.login(password.value).then(e => {
        let toast = e as string;
        

        if (toast.length > 0) {
            loading.value = false;
            emit("toast", toast);
        } else {
            // get passwords, backend will decrypt and cache,
            // ready for mount.
            Binding.get_passwords("")
            .then(() => {
                router.push("/view");
            });
        }
    });
}

</script>

<template>
    <div class="container">
        <h1>Login</h1>

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