<script setup lang="ts">
import Divider from 'primevue/divider';
import OriensButton from '@/components/button/OriensButton.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
import { ref } from 'vue';

const page = ref('');
const handlePageChange = (newPage: string) => {
    page.value = newPage;
    console.log(page.value);
};
</script>

<template>
    <div class="m-10">
        <div>
            <div class="mb-10 text-3xl font-bold">Mon espace Oriens</div>
            <div class="flex items-center  gap-4">
                <button @click="handlePageChange('Vue d)" class="p-2 hover:shadow-lg rounded-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">Vue d'ensemble</button>
                <button class="p-2 hover:shadow-lg rounded-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">Mon profil</button>
                <button class="p-2 hover:shadow-lg rounded-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">Mes favoris</button>
            </div>
            <Divider></Divider>
        </div>
        <div>
            <div class=""></div>
            <div class="grid grid-cols-3 gap-6">
                <div class="bg-white border rounded-xl"></div>
                <div class="col-span-2">
                    <div class="flex flex-col gap-2">
                        <div class="flex gap-2 items-center">
                            <OriensButton label="Mes favoris" ></OriensButton>
                            <OriensButton label="Mes recommandations"  color="white"></OriensButton>
                        </div>
                        <div >
                            <GroupCardJob :page="1" :perpage="3" class="grid grid-cols-3 gap-4 items-center justify-center"></GroupCardJob>
                        </div>
                    </div>
                </div>
            </div>
            <div class="mt-10 flex justify-center">
                <p class="font-bold text-xl">Mes rendez-vous orientation</p>
            </div>
        </div>
    </div>
</template>