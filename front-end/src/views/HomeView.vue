<script setup lang="ts">
import HeaderHomeView from '@/components/HeaderHomeView.vue';
import GroupCardJob from '@/components/GroupCardJob.vue';
</script>



<template>
  <HeaderHomeView></HeaderHomeView>
  <div class="flex flex-col items-center justify-center mt-20 sm: flex flex-row items-center justify-center ">
    <p class="font-rock-salt text-orange">Découvrez Oriens</p>
    <p class="font-bold text-blue">ACCOMPAGNEMENT PERSONNALISÉ POUR</p>
    <p class="font-bold text-blue">LES JEUNES DE 12 À 18 ANS</p>
  </div>
  <GroupCardJob></GroupCardJob>
</template>



<style scoped lang="css">

/* Ajoutez vos styles globaux ici */
</style>