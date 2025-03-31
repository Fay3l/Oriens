<script lang="ts" setup>
const props = defineProps({
    label: {
        type: String,
        default: ''
    }
});
</script>

<template>
    <button
        class="linegradient text-white py-5 px-5 rounded-lg shadow-md hover:bg-orangelight hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">
        {{ props.label }}
    </button>
</template>

<style scoped lang="css">
    .linegradient{
        background: linear-gradient(to bottom , #EE7213 0%, #F09A4E 100%);
    }
</style>