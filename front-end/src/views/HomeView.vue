<script setup lang="ts">
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import IftaLabel from 'primevue/iftalabel';
</script>



<template>
  <header class="border border-indigo-600 flex flex-row justify-around items-center">
    <img src="./../images/logo_blue.svg" class="m-2">
    <div class="flex gap-6">
      <p>Métiers</p>
      <p>Formations</p>
      <p>Nos services</p>
    </div>
    <div class="flex gap-6">
      <p>Se connecter</p>
      <p>Contact</p>
      <p>Découvrir</p>
    </div>
  </header>
  <div class="header flex flex-row justify-around items-center gap-5 ">
    <div class="flex flex-col gap-4">
      <div class="text-white font-bold flex flex-col gap-1 lg:text-6xl sm:text-xl"> 
        <p>UN ACCOMPAGNEMENT</p>
        <div class="flex flex-row items-center gap-2">
          <p class="surligne p-1">PERSONNALISÉ</p>
          <p>POUR</p>
        </div>
        <p>L'ORIENTATION DES</p>
        <div class="flex">
          <p class="surligne p-1">JEUNES</p>
        </div>
      </div>
      <div>
        <img src="./../images/lorem_ipsum.svg">
      </div>
      <div class="lg:flex flex-row gap-4 ">
        <IftaLabel>
          <InputText id="metier" v-model="value" placeholder="Ingénieur" variant="filled" />
          <label for="username">Métier, certification</label>
        </IftaLabel>
        <IftaLabel>
          <InputText id="lieu" v-model="value" placeholder="Paris" variant="filled" />
          <label for="username">Où ?</label>
        </IftaLabel>
        <Button icon="pi pi-search" severity="warn"></Button>
      </div>
    </div>
    <div>
      <img class="mt-3" src="./../images/student1.svg">
    </div>
  </div>

</template>



<style scoped lang="css">
.header {
  background-color: #2F4B98;
}
.surligne{
  background-color: #EE7213;
}
/* Ajoutez vos styles globaux ici */
</style>