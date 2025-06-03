<script setup lang="ts">
import Divider from 'primevue/divider';
import OriensButton from '@/components/button/OriensButton.vue';
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
            <div class="grid grid-cols-2 gap-6 ">
                <div class="bg-white border rounded-xl"></div>
                <div>
                    <div>
                        <OriensButton label="Mes favoris" size="small"></OriensButton>
                        <OriensButton label="Mes recommandations" size="small"></OriensButton>
                    </div>

                </div>

            </div>
        </div>
    </div>
</template>