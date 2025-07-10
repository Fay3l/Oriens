<script setup lang="ts">
import { ref } from 'vue';
import OriensButton from './button/OriensButton.vue';
import { useRouter } from 'vue-router';
const value = ref('');
const router = useRouter();
const login = () => {
    router.push({ name: 'login' });
}
const quiz = () => {
    router.push({ name: 'start-quiz' });
}
const home = () => {
    router.push({ name: 'home' });
}
</script>

<template>
    <header class="border-b-2 p-1 lg:flex flex-wrap sm:  flex flex-row justify-around items-center">
        <button @click="home"><img src="./../images/logo.svg" alt="" class="m-2 lg:w-48 h-12 sm: w-full h-18"></button>
        <div class="flex gap-6 font-bold lg:text-base sm:text-xs">
            <p>Métiers</p>
            <p>Formations</p>
            <p>Nos services</p>
        </div>
        <div class="flex gap-6 flex-row items-center lg:text-base sm:text-xs">
            <button class="hover:font-bold" @click="login">Se connecter</button>
            <OriensButton size="small" @click="quiz" label="Quiz d'orientation"></OriensButton>
        </div>
    </header>

</template>

<style lang="css" scoped></style>