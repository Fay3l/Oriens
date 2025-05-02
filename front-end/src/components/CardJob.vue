<script setup lang="ts">
import { Jobs } from '@/composables/jobs';
import Card from 'primevue/card';
import Button from 'primevue/button';

const props = defineProps({
    job:{
        type: Jobs,
        default: () => ({
            nom_metier: '',
            competences: '',
            identifiant: '',
            acces_metier: '',
        })
    } 
});
</script>

<template>
    <Card class="border">
        <template #content>
            <div class="flex flex-col gap-4 items-center">
                <div>
                    <img class=" lg:w-29 sm:w-30 h-26 " alt="" src="./../images/jobs_card.svg">
                </div>
                <div class="flex flex-col items-center gap-1" >
                    <div class="flex mb-2">
                        <p class="text-orange text-base font-bold ">{{ props.job.nom_metier }}</p>
                    </div>
                    <p class="font-bold text-base text-black">UX review presentations<i class="pi pi-arrow-up-right ml-3"></i></p>
                    {{ props.job.competences }}
                    <Button icon="pi pi-heart" variant="text" rounded aria-label="Favorite" />
                </div>
                
            </div>
        </template>
    </Card>
</template>

<style lang="css" scoped>

</style>