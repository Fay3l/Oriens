<script lang="ts" setup>
const props = defineProps({
    label: {
        type: String,
        default: ''
    },
    size: {
        type: String,
        default: ''
    },
    class: {
        type: String,
        default: ''
    },
    color: {
        type: String,
        default: 'orange'
    }
});
</script>

<template>
    <div v-if="props.color === 'white'">
        <div v-if="size === 'small'">
            <button
                :class="`bg-white border border-orange text-orange py-2 px-2 rounded-lg shadow-md hover:bg-white hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105 ${props.class}`">
                {{ props.label }}
            </button>
        </div>
        <div v-else>
            <button
                :class="`bg-white border border-orange text-orange py-5 px-5 rounded-lg shadow-md hover:bg-white hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105 ${props.class}`">
                {{ props.label }}
            </button>
        </div>
    </div>
    <div v-else>
        <div v-if="size === 'small'">
            <button
                :class="`linegradient text-white py-2 px-2 rounded-lg shadow-md hover:bg-orangelight hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105 ${props.class}`">
                {{ props.label }}
            </button>
        </div>
        <div v-else>
            <button
                :class="`linegradient text-white py-5 px-5 rounded-lg shadow-md hover:bg-orangelight hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105 ${props.class}`">
                {{ props.label }}
            </button>
        </div>
    </div>
</template>

<style scoped lang="css">
.linegradient {
    background: linear-gradient(to bottom, #EE7213 0%, #F09A4E 100%);
}
</style>