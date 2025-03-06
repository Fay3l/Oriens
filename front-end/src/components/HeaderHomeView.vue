<script setup lang="ts">
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import IftaLabel from 'primevue/iftalabel';
import { ref } from 'vue';
const value = ref('');
</script>

<template>
    <header class=" lg:flex flex-wrap sm:  flex flex-row justify-around items-center">
        <img  src="./../images/logo_blue.svg" class="m-2 lg:w-48 h-12 sm: w-full h-5">
        <div class="flex gap-6 lg:text-base sm:text-xs">
            <p>Métiers</p>
            <p>Formations</p>
            <p>Nos services</p>
        </div>
        <div class="flex gap-6 lg:text-base sm:text-xs">
            <p>Se connecter</p>
            <p>Contact</p>
            <p>Découvrir</p>
        </div>
    </header>
    <div class="bg-blue flex flex-row justify-around rounded-b-3xl  gap-5 sm:flex flex-row gap-1 rounded-b-none ">
        <div class="flex items-center sm:flex m-1 text-sm"> 
            <div class="flex flex-col gap-4">
                <div class="text-white font-bold flex flex-col gap-1 lg:text-5xl sm:text-xs">
                    <p class="">UN ACCOMPAGNEMENT</p>
                    <div class="flex flex-row items-center gap-2 ">
                        <p class="bg-orange p-1">PERSONNALISÉ</p>
                        <p>POUR</p>
                    </div>
                    <p>L'ORIENTATION DES</p>
                    <div class="flex">
                        <p class="bg-orange p-1">JEUNES</p>
                    </div>
                </div>
                <div>
                    <img src="./../images/lorem_ipsum.svg">
                </div>
                <div class=" flex flex-row gap-4  ">
                    <IftaLabel class="sm: m-1">
                        <InputText id="metier" v-model="value" size="small" placeholder="Ingénieur" variant="filled" />
                        <label for="username">Métier, certification</label>
                    </IftaLabel>
                    <IftaLabel class="sm: m-1">
                        <InputText id="lieu" v-model="value" size="small" placeholder="Paris" variant="filled" />
                        <label for="username">Où ?</label>
                    </IftaLabel>
                    <Button icon="pi pi-search" class="sm: m-1" severity="warn"></Button>
                </div>
            </div>
        </div>

        <div class=" flex items-end">
            <img class="mt-5 w-full  " src="./../images/student1.svg">
        </div>
    </div>
</template>

<style lang="css" scoped></style>