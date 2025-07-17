<template>
    <div class="bg-[#EE721308] flex flex-col items-center justify-center">
        <div class=" max-w-7xl m-10 grid grid-cols-1 md:grid-cols-4 gap-20 ">
            <!-- Logo & Socials -->
            <div class="flex flex-col items-start space-y-4">
                <img src="./../../images/logo.svg" alt="Oriens Logo" class="h-8 " />
                <div class="opacity-100 flex space-x-4">
                    <a href="#"><i class="fab fa-facebook-f"></i></a>
                    <a href="#"><i class="fab fa-linkedin-in"></i></a>
                    <a href="#"><i class="fab fa-instagram"></i></a>
                    <a href="#"><i class="fab fa-tiktok"></i></a>
                </div>
            </div>

            <!-- Je suis -->
            <div>
                <h3 class="text-orange-600 font-semibold mb-2">Je suis</h3>
                <ul class="space-y-1">
                    <li><a href="#" class="hover:underline">Lycéen</a></li>
                    <li><a href="#" class="hover:underline">Parent d’élève</a></li>
                    <li><a href="#" class="hover:underline">Étudiant post-bac</a></li>
                    <li><a href="#" class="hover:underline">Établissement</a></li>
                </ul>
            </div>

            <!-- Nos offres -->
            <div>
                <h3 class="text-orange-600 font-semibold mb-2">Nos offres</h3>
                <ul class="space-y-1">
                    <li><a href="#" class="hover:underline">Nos packs</a></li>
                    <li><a href="#" class="hover:underline">Options</a></li>
                    <li><a href="#" class="hover:underline">Tarifs écoles</a></li>
                </ul>
            </div>

            <!-- Liens utiles -->
            <div>
                <h3 class="text-orange-600 font-semibold mb-2">Liens utiles</h3>
                <ul class="space-y-1">
                    <li><a href="#" class="hover:underline">Répondre au Quiz</a></li>
                    <li><a href="#" class="hover:underline">Formulaire de contact</a></li>
                    <li><a href="#" class="hover:underline">Mon dashboard</a></li>
                </ul>
            </div>
        </div>

        <!-- Bottom bar -->
        <div
            class="border-t mt-10 pt-4 text-sm gap-8 text-gray-500 flex flex-col md:flex-row justify-between items-center max-w-7xl mx-auto px-4 space-y-2 md:space-y-0">
            <p>© 2025 Oriens. Tous droits réservés</p>
            <div class="space-x-4">
                <a href="#" class="hover:underline">Mentions Légales</a>
                <span>|</span>
                <a href="#" class="hover:underline">Politique sur les cookies</a>
            </div>
        </div>
    </div>

</template>