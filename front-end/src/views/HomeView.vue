<script setup lang="ts">
import GroupCardJob from '@/components/jobs/GroupCardJob.vue';
import InfiniteScroll from '@/components/InfiniteScroll.vue';
import OriensButton from '@/components/button/OriensButton.vue';
import router from '@/router';

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
                        <OriensButton label="FAIRE MON TEST D'ORIENTATION" @click="router.push({name:'signup'})"></OriensButton>
                    </div>
                </div>
            </div>
        </div>

        <div class=" flex items-end infinite-scroll">
            <img alt="" class="mt-5 w-full hidden sm:visible" src="./../images/Image.svg">
        </div>
    </div>
   <InfiniteScroll></InfiniteScroll>
  <div class="flex flex-col items-center font-bold justify-center mt-20 lg:text-xl sm: flex flex-row items-center justify-center ">
    <p class="text-orange font-shadows lg:text-2xl">Découvrez Oriens</p>
    <p class="">ACCOMPAGNEMENT PERSONNALISÉ POUR</p>
    <p class="">LES JEUNES DE 12 À 18 ANS</p>
  </div>
  <div>
    <GroupCardJob class="grid grid-cols-4 gap-4 m-20" :page="1" :perpage="4"></GroupCardJob>
  </div>
<!-- Avis utilisateurs -->
<div class="bg-white py-12">
    <div class="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="bg-gray-50 rounded-xl p-6 shadow">
            <div class="flex items-center gap-2 mb-2">
                <img src="../images/avatar1.png" alt="avatar" class="w-8 h-8 rounded-full" />
                <span class="font-bold">Inès</span>
            </div>
            <div class="flex gap-1 text-yellow-400 mb-2">
                <span>★</span><span>★</span><span>★</span><span>★</span><span>★</span>
            </div>
            <p class="text-sm text-gray-700">
                Grâce à Oriens, j'ai très trouvé une formation qui me correspond vraiment ! Une fiche métier très nette, et des conseils de domaines que je ne connaissais pas du tout.
            </p>
        </div>
        <div class="bg-gray-50 rounded-xl p-6 shadow">
            <div class="flex items-center gap-2 mb-2">
                <img src="../images/avatar2.png" alt="avatar" class="w-8 h-8 rounded-full" />
                <span class="font-bold">Julien T.</span>
            </div>
            <div class="flex gap-1 text-yellow-400 mb-2">
                <span>★</span><span>★</span><span>★</span><span>★</span><span>★</span>
            </div>
            <p class="text-sm text-gray-700">
                Je ne pensais pas être réorienté complètement, mais grâce à ma première séance d'accompagnement, mon choix s'est affiné et j'ai pu rencontrer les bonnes écoles et pros.
            </p>
        </div>
        <div class="bg-gray-50 rounded-xl p-6 shadow">
            <div class="flex items-center gap-2 mb-2">
                <img src="../images/avatar3.png" alt="avatar" class="w-8 h-8 rounded-full" />
                <span class="font-bold">Samuel B.</span>
            </div>
            <div class="flex gap-1 text-yellow-400 mb-2">
                <span>★</span><span>★</span><span>★</span><span>★</span><span>★</span>
            </div>
            <p class="text-sm text-gray-700">
                Grâce à ce projet, j'ai enfin créé du sens. On apprend vraiment à mieux se connaître, et cela change mon choix d'études supérieures.
            </p>
        </div>
    </div>
</div>

<!-- Logos écoles/partenaires -->
<div class="flex flex-wrap justify-center items-center gap-8 py-10 bg-gray-50">
    <img src="../images/logo1.png" alt="logo1" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo2.png" alt="logo2" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo3.png" alt="logo3" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo4.png" alt="logo4" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo5.png" alt="logo5" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo6.png" alt="logo6" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo7.png" alt="logo7" class="h-12 opacity-60 grayscale" />
    <img src="../images/logo8.png" alt="logo8" class="h-12 opacity-60 grayscale" />
    <button class="bg-orange-500 text-white px-6 py-2 rounded-lg font-bold ml-4">+ Découvrez les écoles</button>
</div>

<!-- Call to action section -->
<div class="flex flex-col md:flex-row items-center justify-between bg-orange-400 py-12 px-8 rounded-xl my-16">
    <div class="flex-1 text-white">
        <h2 class="text-2xl md:text-3xl font-bold mb-4">Inscrivez-vous et découvrez<br>une méthode d’orientation nouvelle génération.</h2>
        <a href="#" class="underline font-semibold">DÉCOUVRIR LES MÉTIERS</a>
    </div>
    <div class="flex-1 flex justify-center mt-8 md:mt-0">
        <img src="../images/cta-girl.png" alt="étudiante" class="h-64 object-contain" />
    </div>
</div>

<!-- FAQ -->
<section class="max-w-5xl mx-auto my-20">
    <h3 class="text-2xl font-bold mb-8">
        Un accompagnement <span class="text-orange-500 font-shadows">personnalisé</span>
    </h3>
    <div class="grid md:grid-cols-2 gap-8">
        <div class="bg-gray-100 rounded-xl p-6">
            <h4 class="font-bold mb-2">Est-ce que l’accès à la plateforme est gratuit ?</h4>
            <p class="text-gray-700 text-sm">
                L’inscription et le test d’orientation de base sont gratuits et sans collecte de données. Des formules premium existent pour accéder plus rapidement à des accompagnements individuels et accéder à l’ensemble des ressources.
            </p>
        </div>
        <div class="bg-gray-100 rounded-xl p-6">
            <h4 class="font-bold mb-2">Puis-je télécharger ou imprimer les fiches métiers et formations ?</h4>
            <p class="text-gray-700 text-sm">
                Oui, toutes les fiches sont disponibles en version téléchargeable (PDF), vous pouvez consulter hors ligne ou imprimer selon les besoins.
            </p>
        </div>
    </div>
    <a href="#" class="block mt-4 text-sm underline text-gray-500">Voir toutes les questions</a>
</section>
</template>



<style scoped lang="css">
/* Ajoutez vos styles globaux ici */
</style>