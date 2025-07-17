<script setup lang="ts">
import { Jobs } from '@/composables/jobs';
import Card from 'primevue/card';
import Dialog from 'primevue/dialog';
import Button from 'primevue/button';
import { useRouter } from 'vue-router';
import { ref } from 'vue';

const router = useRouter()
const state = ref(false)
const iconheart = ref('pi pi-heart')
const dialogvisible = ref(false)
const props = defineProps({
    job: {
        type: {} as Jobs,
        default: () => ({
            nom_metier: '',
            competences: '',
            identifiant: '',
            acces_metier: '',
        })
    }
});

// Function to strip HTML tags from a string
function stripHtmlTags(input: string): string {
    const div = document.createElement('div');
    div.innerHTML = input;
    return div.textContent || div.innerText || '';
}

// Function to truncate text to a specific length
function truncateText(input: string, length: number): string {
    return input.length > length ? input.substring(0, length) + '...' : input;
}

function handleClick() {
    dialogvisible.value = true
}
function handleClick2() {
    state.value = !state.value
    iconheart.value = state.value ? 'pi pi-heart-fill' : 'pi pi-heart'
}
</script>

<template>

    <Card
        class="border hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">
        <template #content>
            <div class="flex flex-col gap-4 items-center justify-center"> 
                <button @click="handleClick()">
                    <div class="flex justify-center items-center mb-2">
                        <img class=" max-sm:w-[120px] max-sm:h-[120px] max-md:w-[240px] max-md:h-[240px]  " alt="" src="./../../images/jobs_card.svg">
                    </div>

                    <div class="flex flex-col items-center gap-1">
                        <div class="flex mb-2">
                            <p class="text-orange text-base font-bold ">{{ props.job.nom_metier }}</p>
                        </div>
                        <p class="font-bold text-base text-black">UX review presentations<i
                                class="pi pi-arrow-up-right ml-3"></i></p>
                        <!-- Display truncated competences without HTML tags -->
                        <div class="flex flex-wrap items-center justify-center gap-1">
                            <p class="text-sm text-gray-600 ">{{ truncateText(stripHtmlTags(props.job.competences), 120)
                                }}</p>
                        </div>
                    </div>
                </button>
                <Button @click="handleClick2" :icon="iconheart" variant="text" class="!bg-red" rounded
                    aria-label="Favorite" />
            </div>
        </template>
    </Card>
    <Dialog v-model:visible="dialogvisible" modal :header="props.job.nom_metier" :style="{ width: '50rem' }"
        :breakpoints="{ '1199px': '75vw', '575px': '90vw' }">
        <div class="flex flex-col gap-4 items-center ">
            <div>
                <img class=" lg:w-29 sm:w-30 h-26 " alt="" src="./../../images/jobs_card.svg">
            </div>
            <div class="flex flex-col items-center gap-1">
                <!-- Display competences without HTML tags -->
                <p v-html="props.job.competences"></p>
            </div>
        </div>
    </Dialog>
</template>

<style lang="css" scoped></style>