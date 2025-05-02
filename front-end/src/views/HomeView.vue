<script setup lang="ts">
import HeaderHomeView from '@/components/HeaderHomeView.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
import { onMounted,ref } from 'vue';
import type { Jobs } from '@/composables/jobs';

const jobs = ref<Jobs[]>([]);
onMounted(() => {
  fetch('http://localhost:3000/api/jobs?page=1&per_page=2')
    .then((response) => response.json())
    .then((data) => {
      jobs.value = data;
      console.log('Jobs fetched:', jobs.value);
    })
    .catch((error) => console.error('Error fetching jobs:', error));
});
</script>



<template>
  <header>
    <HeaderHomeView></HeaderHomeView>
  </header>
  <img alt="" class="lg:mt-10" src="../images/ecoles.png">
  <div class="flex flex-col items-center font-bold justify-center mt-20 lg:text-xl sm: flex flex-row items-center justify-center ">
    <p class="text-orange font-shadows lg:text-2xl">Découvrez Oriens</p>
    <p class="">ACCOMPAGNEMENT PERSONNALISÉ POUR</p>
    <p class="">LES JEUNES DE 12 À 18 ANS</p>
  </div>
  <div>
    <GroupCardJob class="flex flex-col items-center gap-2 m-5" :jobs="jobs" ></GroupCardJob>
  </div>
</template>



<style scoped lang="css">
/* Ajoutez vos styles globaux ici */
</style>