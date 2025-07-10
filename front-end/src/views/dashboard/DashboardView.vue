<script setup lang="ts">
import Divider from 'primevue/divider';
import OriensButton from '@/components/button/OriensButton.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
import { onMounted, ref } from 'vue';
import { useUsers } from '@/stores/useUsers';

const usersStore = useUsers();
const page = ref('vue');
const handlePageChange = (newPage: string) => {
    page.value = newPage;
    console.log(page.value);
};
onMounted(async () => {
    // Initial setup or data fetching can be done here
    console.log('DashboardView mounted');
    await usersStore.getUser();
    console.log('User data:', usersStore.users);

}); 
</script>

<template>
    <div class="m-10">
        <div>
            <div class="mb-10 text-3xl font-bold">Mon espace Oriens</div>
            <div class="flex items-center  gap-4">
                <button @click="handlePageChange('vue')" class="p-2 hover:shadow-lg rounded-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">Vue d'ensemble</button>
                <button @click="handlePageChange('profil')" class="p-2 hover:shadow-lg rounded-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">Mon profil</button>
                <button @click="handlePageChange('favoris')" class="p-2 hover:shadow-lg rounded-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">Mes favoris</button>
            </div>
            <Divider></Divider>
        </div>
        <div v-if="page === 'vue'">
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
            <div class="mt-12 flex flex-col ">
                <p class="font-bold text-xl text-center">Mes rendez-vous orientation</p>
                <div>
                    <div class="border rounded-lg bg-[#FFF6EE] h-80 w-[15%] ">

                    </div>
                </div>
            </div>
        </div>
        <div v-if="page === 'profil'" class="">
            <p>Profil</p>
            <p>{{ usersStore.users }}</p>
        </div>
        <div v-else class=""></div>
    </div>
</template>