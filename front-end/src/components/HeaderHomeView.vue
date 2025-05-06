<script setup lang="ts">
import { ref } from 'vue';
import OriensButton from './button/OriensButton.vue';
const value = ref('');
</script>

<template>
    <header class=" lg:flex flex-wrap sm:  flex flex-row justify-around items-center">
        <img  src="./../images/logo.svg" alt="" class="m-2 lg:w-48 h-12 sm: w-full h-18">
        <div class="flex gap-6 font-bold lg:text-base sm:text-xs">
            <p>Métiers</p>
            <p>Formations</p>
            <p>Nos services</p>
        </div>
        <div class="flex gap-6 flex-row items-center lg:text-base sm:text-xs">
            <p>Se connecter</p>
            <OriensButton size="small" label="Quiz d'orientation"></OriensButton>
        </div>
    </header>
    
</template>

<style lang="css" scoped></style>