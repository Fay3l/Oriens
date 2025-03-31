<script lang="ts" setup>
const props = defineProps({
    label: {
        type: String,
        default: ''
    }
});
</script>

<template>
    <button class="bg-orange text-white font-bold py-5 px-5 rounded-lg shadow-md hover:bg-orangelight hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">
        {{ props.label }}
    </button>
</template>