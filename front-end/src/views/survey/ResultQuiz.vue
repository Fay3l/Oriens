<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-[#FFFCFA] relative overflow-hidden">
    <!-- Décorations de fond -->
    <div class="absolute left-0 top-0 w-1/2 h-full opacity-10 select-none pointer-events-none">
      <svg viewBox="0 0 400 400" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-full">
        <path d="M200 350 Q100 200 200 50 Q300 200 200 350 Z" fill="#EE7213" />
      </svg>
    </div>
    <div class="absolute right-0 top-0 w-1/2 h-full opacity-10 select-none pointer-events-none">
      <svg viewBox="0 0 400 400" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-full">
        <circle cx="320" cy="320" r="80" fill="#EE7213" />
      </svg>
    </div>
    <!-- Contenu principal -->
    <div class="z-10 flex flex-col items-center w-full">
      <h1 class="text-3xl md:text-4xl font-bold text-center leading-tight text-black mb-2">
        Quiz orientation <span class="block font-handwriting text-[#EE7213] text-2xl md:text-3xl mt-1">terminé</span>
      </h1>
      <div class="mt-2 text-center text-gray-500 text-xs md:text-sm mb-6">
        Bravo, vous avez répondu aux questions !
      </div>
      <div v-if="loading" class="text-orange-500 font-semibold my-8">Analyse de vos réponses...</div>
      <div v-else-if="error" class="text-red-500 font-semibold my-8">{{ error }}</div>
      <div v-else-if="quizStore.collection" class="w-full max-w-2xl mt-6">
        <h2 class="text-xl font-bold text-center mb-4">Métiers qui pourraient vous correspondre :</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div v-for="(m, idx) in quizStore.collection.metiers" :key="idx"
            class="bg-white rounded-xl shadow p-6 flex flex-col gap-2">
            <div class="font-bold text-base text-[#EE7213]">{{ m }}</div>
          </div>
        </div>
      </div>
      <button v-if="!loading" @click="dashboard"
        class="mt-8 px-8 py-3 bg-gradient-to-r from-[#EE7213] to-[#F09A4E] text-white rounded-lg text-base font-semibold shadow hover:brightness-110 transition-all">
        Découvrir mon profil
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useQuiz } from '@/stores/useQuiz';
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const quizStore = useQuiz(); 
const loading = ref(true);
const error = ref('');

const dashboard = () => {
    router.push({ name: 'dashboard' });
}

onMounted(async () => {
  loading.value = true;
  error.value = '';
  try {
    const data = localStorage.getItem('questionnaireData');
    console.log('Data from localStorage:', data);
    const res = await quizStore.getResponseQuiz(data ? JSON.parse(data) : {});
    localStorage.setItem('quizResults', JSON.stringify(res));

    if (!res || res.error) {
      error.value = res?.error || "Erreur lors de la récupération des résultats.";
    }
  } catch (e: any) {
    error.value = e.message || "Erreur inconnue";
  } finally {
    localStorage.removeItem('questionnaireData')
    loading.value = false;
  }
});


</script>