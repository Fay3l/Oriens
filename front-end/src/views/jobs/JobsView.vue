<script setup lang="ts">
import OriensButton from '@/components/button/OriensButton.vue';
import { Jobs } from '@/composables/jobs';
import { IconField, InputIcon, InputText } from 'primevue';
import { ref, watch } from 'vue';
import GroupCardJob from '@/components/jobs/GroupCardJob.vue';
import { useJobs } from '@/stores/useJobs';

const page = ref(1);
const perPage = ref(16);
const jobs = ref<Jobs[]>([]);
const search = ref('');
const jobstore = useJobs();

const disabled = ref(false);
watch(page, newValue => {
    checkdisabled();
});


const nextpage = async () => {
    if (disabled.value) return;
    page.value++;
    console.log("Page: ", page.value);
    await jobstore.getJobs(page.value, perPage.value);
    jobs.value = jobstore.collection;
}
const previouspage = async () => {
    if (page.value <= 1) return;
    page.value--;
    console.log("Page: ", page.value);
    await jobstore.getJobs(page.value, perPage.value);
    jobs.value = jobstore.collection;

}
async function searchJobs(search: string): Promise<Jobs[]> {
    try {
        const response = await fetch(`http://localhost:3000/api/jobs/search?search=${search}`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('token')}`,
            },
        });
        const data: Jobs[] = await response.json();
        jobs.value = data;
        console.log('Jobs searched:', jobs.value);
        return data;
    } catch (error) {
        console.error('Error searching jobs:', error);
        return [];
    }
}
const checkdisabled = (): boolean => {
    if (page.value <= 1) {
        return true;
    }
    else {
        return false;
    }
}
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
        <IconField>
            <InputIcon>
                <i class="pi pi-search" />
            </InputIcon>
            <InputText v-model="search" @keydown="searchJobs(search)" placeholder="Peintre" />
        </IconField>
    </div>
    <div>
        <GroupCardJob class="grid grid-cols-1 justify-center gap-6 mr-40 ml-40 mb-5 md:grid-cols-4" :page="page"
            :jobs="jobs" :perpage="perPage"></GroupCardJob>
    </div>
    <div class="flex justify-center items-center gap-20 p-10 mt-10">
        <button @click="previouspage" :hidden="checkdisabled()">
            <span class="pi pi-angle-left"></span>
        </button>
        <button @click="nextpage">
            <span class="pi pi-angle-right "></span>
        </button>
    </div>


</template>
