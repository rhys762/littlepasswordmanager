<script setup lang=ts>
import { computed } from "vue";

const props = defineProps({
    text: String,
    show: Boolean
});

const opacity = computed(() => {
    return props.show ? 1 : 0;
})

</script>

<template>
    <div class="toast" :class="{pulse: props.show}">{{ props.text }}</div>
</template>

<style>

.toast {
    transition: opacity 0.1s ease-in-out;
    opacity: v-bind(opacity);
    background-color: #FFF;
    border-radius: 20px;
    width: fit-content;
    padding: 20px;
    position: fixed;
    bottom: 30px;
    margin-left: auto;
    margin-right: auto;
    left: 0;
    right: 0;
    border: 1px solid #aaa;
}

.pulse {
    animation: pulse 0.3s ease-in-out forwards;
}

@keyframes pulse {
    0% { 
        box-shadow: 0 0 0 0 rgba(255, 0, 0, 1);
    }
    80% {
        box-shadow: 0 0 0 4px rgba(255, 0, 0, 0.6);
    }
    100% {
        box-shadow: 0 0 0 2px rgba(255, 0, 0, 0.0);
    }
}

</style>