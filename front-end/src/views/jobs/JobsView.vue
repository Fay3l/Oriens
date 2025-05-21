<script setup lang="ts">
import OriensButton from '@/components/button/OriensButton.vue';
import { Jobs } from '@/composables/jobs';
import { IconField, InputIcon, InputText } from 'primevue';
import { computed, onMounted, ref } from 'vue';
import { useFetch } from '@vueuse/core'
import GroupCardJob from '@/components/GroupCardJob.vue';

const page = ref(1);
const perPage = ref(16);
const url = ref(`http://localhost:3000/api/jobs?page=${page.value}&per_page=${perPage.value}`);

const search = ref('');



function searchJobs() {
    console.log("Search: ",search.value);
    console.log(localStorage.getItem('token'))
    console.log("Url: ",url.value);
    fetch(`http://localhost:3000/api/jobs/search?search=${search.value}`,{
        method:'GET',
        headers: {
            'Authorization': 'Bearer ' + localStorage.getItem('token'),
        },
    })
        .then((response) => response.json())
        .then((data: Jobs[]) => {
            jobs.value = data;
        })
        .catch((error) => {
            console.log('Error fetching jobs:', error);
            return [] as Jobs[]});
}

const jobs = ref<Jobs[]>([]);
onMounted(()=>{
    fetch(url.value)
        .then((response) => response.json())
        .then((data: Jobs[]) => {
            jobs.value = data;
        })
        .catch((error) => [] as Jobs[]);
})
</script>
<template>
    <div class="flex flex-col gap-4 items-center justify-center mt-4 gap-2 ">
        <div class="flex flex-col items-center justify-center text-center">
            <p class="font-bold text-5xl">Des fiches formation</p>
            <p class="text-5xl"><span class="text-orange font-shadows">personnalisées </span><span
                    class="font-bold">pour ton </span><span class="text-orange font-shadows">orientation</span></p>
        </div>
        <div class="flex flex-col items-center justify-center text-center m-2">
            <p class="text-xl">Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>
            <p class="text-xl">Vehicula massa in enim luctus. Rutrum arcu.</p>
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
            <InputText v-model="search" @keydown="searchJobs()" placeholder="Search" />
        </IconField>
        <InputText v-model="page" placeholder="page" />
        <InputText v-model="perPage" placeholder="per page" />
    </div>
    <div>
        <GroupCardJob class="grid grid-cols-4 gap-6 mr-40 ml-40 mb-5" :jobs="jobs"></GroupCardJob>
    </div>


</template>
