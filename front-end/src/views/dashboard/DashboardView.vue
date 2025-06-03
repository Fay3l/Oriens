<script setup lang="ts">
import Divider from 'primevue/divider';
import OriensButton from '@/components/button/OriensButton.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
</script>

<template>
    <div class="m-10">
        <div>
            <div class="mb-10 text-3xl font-bold">Mon espace Oriens</div>
            <div class="flex items-center  gap-4">
                <button>Vue d'ensemble</button>
                <button>Mon profil</button>
                <button>Mes favoris</button>
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