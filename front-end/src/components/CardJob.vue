<script setup lang="ts">
import { Jobs } from '@/composables/jobs';
import Card from 'primevue/card';
import Button from 'primevue/button';
import { useRouter } from 'vue-router';
import { ref } from 'vue';

const router = useRouter()
const state = ref(false)
const iconheart = ref('pi pi-heart')
const props = defineProps({
    job: {
        type: Jobs,
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
    router.push({
        name:"login"
    })
}
function handleClick2() {
    state.value = !state.value
    iconheart.value = state.value ? 'pi pi-heart-fill' : 'pi pi-heart'
}
</script>

<template>
    
        <Card class="border">
            <template #content>
                <div class="flex flex-col gap-4 items-center ">
                    <div>
                        <img class=" lg:w-29 sm:w-30 h-26 " alt="" src="./../images/jobs_card.svg">
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
                        <Button @click="handleClick2" :icon="iconheart" variant="text" class="!color-red" rounded aria-label="Favorite" />
                    </div>
                </div>
            </template>
        </Card>
</template>

<style lang="css" scoped></style>