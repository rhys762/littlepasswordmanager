<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Toast from "./components/Toast.vue";

import { ref, onMounted } from "vue";
import { useRouter } from 'vue-router';

import { Binding } from "./binds";

let show_toast = ref(false);
let toast_text = ref("");
const router = useRouter();

onMounted(() => {
    Binding.get_master_hash().then((s) => {
        console.log("have remounted app.");
        const str = s as string;
        console.log("got master hash", str);

        if (str.length) {
            router.push("/login");
            console.log("routing to login");
        } else {
            router.push("/setup");
            console.log("routing to setup");
        }
    });
});

let toast_id = 0;
function next_toast_id() {
    toast_id += 1;
    return toast_id;
}

function toast(str: string) {
    const id = next_toast_id();
    show_toast.value = false;
    setTimeout(() => {
        toast_text.value = str;
        show_toast.value = true;
        setTimeout(() => {
            if (toast_id == id) {
                show_toast.value = false;
            }
        }, 5000);
    }, 100);
}

// router

</script>

<template>
    <div class="container">
        <RouterView v-on:toast="toast" />
    </div>
    <Toast :text="toast_text" :show="show_toast"></Toast>
</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
