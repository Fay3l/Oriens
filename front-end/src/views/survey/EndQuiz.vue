<template>
    <div class="bg-[#FFFCFA] min-h-screen w-full overflow-x-hidden pb-12">
        <!-- Titre principal -->
        <div class="pt-12 text-center">
            <h1 class="text-4xl md:text-5xl font-bold leading-tight text-black">
                Voilà le chemin qui <span class="block font-handwriting text-[#EE7213] text-3xl md:text-4xl mt-1">te
                    ressemble</span>
            </h1>
            <div class="mt-4 text-gray-600 text-sm max-w-2xl mx-auto">
                Voici une proposition personnalisée selon tes réponses. Prends ton temps pour l’explorer.<br>
                Tes résultats se sont automatiquement enregistrés dans ton tableau de bord que tu retrouveras sur ton
                compte.
            </div>
        </div>

        <!-- Carte profil principal -->
        <div class="flex justify-center mt-10">
            <div
                class="bg-gradient-to-r from-[#EE7213] to-[#F09A4E] rounded-xl px-10 py-7 text-center max-w-2xl w-full">
                <div class="text-white text-lg font-bold tracking-wide mb-2">LE CRÉATIF</div>
                <div class="text-white text-sm">Tu aimes créer, innover, et travailler dans un cadre libre.<br>Tu es
                    fait pour des formations qui stimulent ton imagination et te laissent de l’autonomie.</div>
            </div>
        </div>



        <!-- Boutons fin -->
        <div class="flex flex-col items-center mt-12 gap-2">
            <button
                class="px-10 py-3 bg-gradient-to-r from-[#EE7213] to-[#F09A4E] text-white rounded-lg text-lg font-semibold shadow hover:brightness-110 transition-all">
                Découvrir mon profil
            </button>
            <button class="mt-2 text-[#EE7213] text-sm font-semibold hover:underline bg-transparent">Recommencer le
                quiz</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
const content = route.state?.content || [];
console.log('Content from state:', content);
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Pacifico&display=swap');

.font-handwriting {
    font-family: 'Pacifico', cursive;
}
</style>
