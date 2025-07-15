<script setup lang="ts">
import Divider from 'primevue/divider';
import OriensButton from '@/components/button/OriensButton.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
import InputText from 'primevue/inputtext';
import { onMounted, ref } from 'vue';
import { useUsers } from '@/stores/useUsers';
import { User } from '@/composables/user';
import Button from 'primevue/button';
import ParentDashboard from '@/components/ParentDashboard.vue';
const usersStore = useUsers();

const page = ref('vue');
const edit=ref(false);
const edituser = ref<User>({} as User);
const handlePageChange = (newPage: string) => {
    page.value = newPage;
    console.log(page.value);
};
onMounted(async () => {
    // Initial setup or data fetching can be done here
    await usersStore.getUser();
    console.log('User data:', usersStore.users);
    edituser.value = usersStore?.users ;

}); 

</script>

<template>
    <div v-if="edituser.role === 'ParentStudent'">
        <ParentDashboard></ParentDashboard>
    </div>
    <div v-else class="m-10">
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
            <div class="flex flex-row flex-end gap-5 m-2">
                <Button icon="pi pi-check" variant="text" severity="success" rounded aria-label="Filter" />
                <Button icon="pi pi-times" severity="danger" variant="text" rounded aria-label="Cancel" :v-show="edit" />
                <Button icon="pi pi-user-edit" class="!bg-orange" aria-label="Notification" />
                

            </div>
            <div class="flex flex-col gap-4">
                <div class="flex flex-col gap-2">
                    <label for="firstname">Prénom</label>
                    <InputText id="firstname" v-model="edituser.firstName" :disabled="edit" />
                </div>
                <div class="flex flex-col gap-2">
                    <label for="lastname">Nom</label>
                    <InputText id="lastname" v-model="edituser.lastName" />
                </div>
                <div class="flex flex-col gap-2">
                    <label for="email">Email</label>
                    <InputText id="email" v-model="edituser.email" />
                </div>
                <div class="flex flex-col gap-2">
                    <label for="address">Adresse</label>
                    <InputText id="address" v-model="edituser.address" />
                </div>
                <div class="flex flex-col gap-2">
                    <label for="city">Ville</label>
                    <InputText id="city" v-model="edituser.city" />
                </div>
                <div class="flex flex-col gap-2">
                    <label for="postalcode">Code postal</label>
                    <InputText id="postalcode" v-model="edituser.postalcode" />
                </div>

            </div>
        </div>
        <div v-else class="">
            <p>Mes favoris</p>
        </div>
    </div>
</template>