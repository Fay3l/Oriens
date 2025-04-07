<script setup lang="ts">
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import IftaLabel from 'primevue/iftalabel';
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
    <div class="flex flex-row justify-around items-center rounded-b-5xl gap-10 m-3 xs:flex-col gap-6 rounded-b-none items-center m-5 ">
        <div class="flex flex-col items-center lg:text-3xl gap-3 sm:text-2xl gap-3 xs:text-lg"> 
            <div class="flex flex-col gap-6">
                <div class=" font-bold flex flex-col gap-5 ">
                    <p class="font-jakarta">Un accompagnement</p>
                    <div class="flex flex-row items-center gap-2">
                        <p class="text-orange font-shadows ">personnalisé</p>
                        <p class="font-jakarta">pour</p>
                    </div>
                    <p class="font-jakarta">l'orientation des</p>
                    <div class="flex">
                        <p class="text-orange font-shadows">jeunes</p>
                    </div>
                </div>
                <div>
                    <img alt="" src="./../images/lorem_ipsum.svg">
                </div>
                <div class=" flex flex-col gap-4  ">
                    <div>
                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p> 
                        <p>Vehicula massa in enim luctus. Rutrum arcu.</p>
                    </div>
                    <div class="text-base" >
                        <OriensButton label="FAIRE MON TEST D'ORIENTATION"></OriensButton>
                    </div>
                </div>
            </div>
        </div>

        <div class=" flex items-end">
            <img alt="" class="mt-5 w-full hidden sm:visible" src="./../images/Image.svg">
        </div>
    </div>
</template>

<style lang="css" scoped></style>