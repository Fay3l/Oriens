<script setup lang="ts">
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import IftaLabel from 'primevue/iftalabel';
import { ref } from 'vue';
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
        <div class="flex gap-6 font-bold lg:text-base sm:text-xs">
            <p>Se connecter</p>
            <p>Contact</p>
            <p>Découvrir</p>
        </div>
    </header>
    <div class="flex flex-row justify-around rounded-b-5xl  gap-5  sm:flex flex-row gap-1 rounded-b-none ">
        <div class="flex items-center sm:flex m-1 text-sm"> 
            <div class="flex flex-col gap-4">
                <div class=" font-bold flex flex-col gap-1 lg:text-5xl sm:text-xs">
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
                    <div >
                        <Button class="bg-orangelight border-none text-orange p-20 "  >FAIRE MON TEST D'ORIENTATION</Button>
                    </div>
                </div>
            </div>
        </div>

        <div class=" flex items-end">
            <img alt="" class="mt-5 w-full  " src="./../images/student1.svg">
        </div>
    </div>
</template>

<style lang="css" scoped></style>