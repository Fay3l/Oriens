<script setup lang="ts">
import CardJob from './CardJob.vue';
</script>

<template>
    <div cl>
        <div v-for="index in 4" :key="index">
            <div>
                <CardJob />
            </div>
        </div>
    </div>

</template>

<style lang="css" scoped></style>