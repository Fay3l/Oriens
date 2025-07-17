<script setup lang="ts">
import { onMounted, ref } from 'vue';
import OriensButton from './button/OriensButton.vue';
import { useRouter } from 'vue-router';
import { useUsers } from '@/stores/useUsers';
const userStore = useUsers();
const router = useRouter();
const id = localStorage.getItem('id');
const login = () => {
    router.push({ name: 'login' });
}
const quiz = () => {
    router.push({ name: 'start-quiz' });
}
const home = () => {
    router.push({ name: 'home' });
}
const dashboard = () => {
    router.push({ name: 'dashboard' });
}
const jobs = () => {
    router.push({ name: 'jobs' });
}
onMounted(async () => {
    await userStore.getUser();
    
});
const animationbutton = "p-2 rounded-lg hover:shadow-xl transform hover:-translate-y-1 transition-all duration-300 ease-in-out"
</script>

<template>
    <header class="border-b-2 p-1 lg:flex flex-wrap sm:  flex flex-row justify-around items-center">
        <button @click="home"><img src="./../images/logo.svg" alt="" class="m-2 lg:w-48 h-12 sm: w-full h-18"></button>
        <div class="flex gap-6 font-bold lg:text-base sm:text-xs">
            <button :class="animationbutton" @click="jobs">Métiers</button>
            <button :class="animationbutton">Formations</button>
            <button :class="animationbutton">Nos services</button>
        </div>
        <div class="flex gap-6 flex-row items-center lg:text-base sm:text-xs">
            <div v-if="id">
                <button class="hover:font-bold" @click="dashboard">Bonjour {{ userStore.users.firstname }}</button>
            </div>
            <div v-else>
                <button class="hover:font-bold"  @click="login">Se connecter</button>
            </div>
            <OriensButton size="small" @click="quiz" label="Quiz d'orientation"></OriensButton>
        </div>
    </header>

</template>

<style lang="css" scoped></style>