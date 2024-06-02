<script setup lang="ts">

import { ref, onMounted } from "vue";
import { useRouter, useRoute } from 'vue-router';
import { Binding } from "../binds";

const domain = ref("");

const router = useRouter();
const route = useRoute();

onMounted(() => {
    const dom = route.params.domain;
    domain.value = decodeURI(dom as string);
});

function back() {
    router.push("/view");
}

function delete_domain() {
    Binding.delete(domain.value).then(() => {
        router.push("/view");
    });
}

</script>

<template>
    <div class="container">
        <div class="warning">This will permenently delete {{ domain }}!</div>

        <div class="button_row">
            <button class="back" @click="back">Back</button>
            <button class="delete_domain" @click="delete_domain">Delete {{ domain }}</button>
        </div>
    </div>
</template>

<style scoped>

.warning {
    margin-bottom: 10px;
}

.button_row {
    display: flex;
    justify-content: center;
    align-items: center;
}

.back {
    margin-right: 10px;
}

.delete_domain {
    color: white;
    background-color: red;
    
}

</style>