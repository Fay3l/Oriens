<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-[#FFFCFA] relative overflow-hidden">
    <!-- Décorations de fond -->
    <!-- Contenu principal -->
    <div class="z-10 flex flex-col items-center">
      <h1 class="text-4xl md:text-5xl font-bold text-center leading-tight text-black">
        Découvrez votre <span class="relative align-middle">
          <span class="font-jakarta text-orange text-3xl md:text-4xl align-middle " style=" position: relative; top: -0.2em;">personnalité</span>
        </span>
        <br />en réalisant ce <span class="font-jakarta text-orange text-3xl md:text-4xl align-middle">quiz</span>
      </h1>
      <div class="mt-2 text-center text-gray-500 text-xs md:text-sm">
        Temps estimé : 3-5 minutes<br />
        Répondez aux 7 questions et découvrez vos résultats
      </div>
      <button class="mt-8 px-8 py-3 bg-gradient-to-r from-[#EE7213] to-[#F09A4E] text-white rounded-lg text-lg font-semibold shadow hover:brightness-110 transition-all">
        Je commence mon quiz
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
// ...
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Pacifico&display=swap');
.font-handwriting {
  font-family: 'Pacifico', cursive;
  letter-spacing: 0.5px;
}
</style>
