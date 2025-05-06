<script setup lang="ts">
import OriensButton from '@/components/button/OriensButton.vue';
import { IconField, InputIcon, InputText } from 'primevue';
</script>
<template>
    <div class="flex flex-col gap-4 items-center justify-center ">
        <div class="flex flex-col items-center justify-center text-center">
            <p class="font-bold text-2xl">Des fiches formation</p>
            <p class="text-2xl"><span class="text-orange font-shadows">personnalisées </span><span
                    class="font-bold">pour ton </span><span class="text-orange font-shadows">orientation</span></p>
        </div>
        <div class="flex flex-col items-center justify-center text-center m-2">
            <p class="text-base">Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>
            <p class="text-base">Vehicula massa in enim luctus. Rutrum arcu.</p>
        </div>

    </div>
    <div class="grid grid-cols-5 gap-4 m-5 ">
        <img alt="" src="./../../images/Oriens _ New/image 4.svg">
        <img alt="" src="./../../images/Oriens _ New/Image.svg">
        <img alt="" src="./../../images/Oriens _ New/Image-3.svg">
        <img alt="" src="./../../images/Oriens _ New/Image-2.svg">
        <img alt="" src="./../../images/Oriens _ New/Image-1.svg">
    </div>
    <div class="flex flex-col items-center justify-center text-center m-10 gap-4">
        <OriensButton label="DÉCOUVRIR LES MÉTIERS"></OriensButton>
        <IconField>
            <InputIcon>
                <i class="pi pi-search" />
            </InputIcon>
            <InputText placeholder="Search" />
        </IconField>
    </div>
    <div class="grid grid-cols-4 gap-4 m-5">
        

    </div>
    

</template>
