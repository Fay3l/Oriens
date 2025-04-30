<script setup lang="ts">
import HeaderHomeView from '@/components/HeaderHomeView.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
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
    <GroupCardJob class="flex flex-col items-center gap-2 m-2"></GroupCardJob>
  </div>
</template>



<style scoped lang="css">
/* Ajoutez vos styles globaux ici */
</style>