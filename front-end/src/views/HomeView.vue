<script setup lang="ts">
import GroupCardJob from '@/components/GroupCardJob.vue';
import { onMounted,ref } from 'vue';
import type { Jobs } from '@/composables/jobs';
import OriensButton from '@/components/button/OriensButton.vue';
import { useJobs } from '@/stores/useJobs';

const jobsStore = useJobs();
const jobs = ref<Jobs[]>([]);
onMounted(() => {
  jobsStore.getJobs(1,4)
});
</script>



<template>
  <div class="flex flex-row justify-around items-center rounded-b-5xl gap-10 m-3 xs:flex-col gap-6 rounded-b-none items-center m-5 ">
        <div class="flex flex-col items-center lg:text-3xl gap-3 sm:text-2xl gap-3 xs:text-lg"> 
            <div class="flex flex-col gap-6">
                <div class=" font-bold flex flex-col gap-5 ">
                    <p class="font-jakarta">Un accompagnement</p>
                    <div class="flex flex-row items-center gap-2">
                        <p class="text-orange font-shadows ">personnalisé</p>
                        <p class="font-jakarta">pour</p>
                    </div>
                    <p class="font-jakarta">l'orientation des</p>
                    <div class="flex">
                        <p class="text-orange font-shadows">jeunes</p>
                    </div>
                </div>
                <div>
                    <img alt="" src="./../images/lorem_ipsum.svg">
                </div>
                <div class=" flex flex-col gap-4  ">
                    <div>
                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p> 
                        <p>Vehicula massa in enim luctus. Rutrum arcu.</p>
                    </div>
                    <div class="text-base" >
                        <OriensButton label="FAIRE MON TEST D'ORIENTATION"></OriensButton>
                    </div>
                </div>
            </div>
        </div>

        <div class=" flex items-end">
            <img alt="" class="mt-5 w-full hidden sm:visible" src="./../images/Image.svg">
        </div>
    </div>
  <img alt="" class="lg:mt-10" src="../images/ecoles.png">
  <div class="flex flex-col items-center font-bold justify-center mt-20 lg:text-xl sm: flex flex-row items-center justify-center ">
    <p class="text-orange font-shadows lg:text-2xl">Découvrez Oriens</p>
    <p class="">ACCOMPAGNEMENT PERSONNALISÉ POUR</p>
    <p class="">LES JEUNES DE 12 À 18 ANS</p>
  </div>
  <div>
    <GroupCardJob class="grid grid-cols-4 gap-4 m-20" :jobs="jobsStore.collection"></GroupCardJob>
  </div>
</template>



<style scoped lang="css">
/* Ajoutez vos styles globaux ici */
</style>