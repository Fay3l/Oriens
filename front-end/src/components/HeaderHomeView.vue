<script setup lang="ts">
import { ref } from 'vue';
import OriensButton from './button/OriensButton.vue';
import { useRouter } from 'vue-router';
const value = ref('');
const router = useRouter();
const login = ()=>{
    router.push({ name: 'login' });
}
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
            <button class="hover:font-bold" @click="login">Se connecter</button>
            <OriensButton size="small" label="Quiz d'orientation"></OriensButton>
        </div>
    </header>
    
</template>

<style lang="css" scoped></style>