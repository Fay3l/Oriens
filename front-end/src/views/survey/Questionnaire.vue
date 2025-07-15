<script setup lang="ts">
import { ref } from 'vue';
import { questionnaire } from '../../composables/questionnaire';
import { useRouter } from 'vue-router';
import { useQuiz } from '@/stores/useQuiz';

const currentSection = ref(0);
const currentQuestion = ref(0);
const sections = questionnaire.value.sections;
const router = useRouter();
const QuizStore = useQuiz();

const selectedOption = ref(sections[currentSection.value].questions[currentQuestion.value].response || '');

function selectOption(option: string) {
    selectedOption.value = option;
    // Stocke la réponse dans la structure du questionnaire
    sections[currentSection.value].questions[currentQuestion.value].response = option;
}

function nextSection() {
    // Enregistre la réponse avant de passer à la suivante
    sections[currentSection.value].questions[currentQuestion.value].response = selectedOption.value;
    // Passe à la question suivante dans la section
    if (currentQuestion.value < sections[currentSection.value].questions.length - 1) {
        currentQuestion.value++;
        selectedOption.value = sections[currentSection.value].questions[currentQuestion.value].response || '';
    } else if (currentSection.value < sections.length - 1) {
        // Passe à la section suivante
        currentSection.value++;
        currentQuestion.value = 0;
        selectedOption.value = sections[currentSection.value].questions[currentQuestion.value].response || '';
    }
}

const finishQuiz = async () => {
    sections[currentSection.value].questions[currentQuestion.value].response = selectedOption.value;
    localStorage.setItem('questionnaireData', JSON.stringify(questionnaire.value));
    router.push({ name: 'result-quiz' });
};
</script>

<template>
    <div class="relative min-h-screen bg-[#FFFCFA] flex flex-col">
        <!-- Décorations de fond -->
        <div class="absolute left-[0%] top-[20%] w-[25%] h-full opacity-10 select-none pointer-events-none z-0">
            <svg xmlns="http://www.w3.org/2000/svg" width="609" height="752" viewBox="0 0 609 752" fill="none">
                <g>
                    <path
                        d="M-104.753 523.154C-112.933 496.713 -146.24 429.222 -93.3831 428.017C-15.7514 426.222 61.5795 409.103 132.649 377.651C154.876 367.831 177.895 355.369 189.244 333.892C205.768 302.72 191.96 264.123 173.634 233.893C137.888 175.247 86.2916 128.808 33.1669 86.2207C3.71683 62.6526 -15.7591 58.5238 14.573 28.6027C24.408 18.82 49.3882 -9.48636 65.4255 3.30794C141.934 64.2438 217.252 127.751 267.201 213.201C290.883 253.681 308.401 301.923 295.54 346.96C284.145 386.793 251.024 416.728 215.772 438.54C120.704 497.445 4.40845 512.597 -104.716 522.954L-104.753 523.154Z"
                        fill="#E1620F" />
                    <path
                        d="M-28.084 312.783C10.153 286.283 19.7145 233.871 -6.72769 195.716C-33.1699 157.562 -85.6027 148.113 -123.84 174.612C-162.077 201.111 -171.638 253.524 -145.196 291.678C-118.754 329.833 -66.3209 339.282 -28.084 312.783Z"
                        fill="#E1620F" />
                    <path
                        d="M61.0224 762.359C82.9083 779.302 134.409 834.184 154.095 785.115C183.032 713.056 226.211 646.658 280.613 591.155C297.611 573.791 317.362 556.613 341.456 553.527C376.445 548.999 407.738 575.478 429.609 603.252C471.971 657.312 497.339 721.928 518.564 786.623C530.292 822.473 527.32 842.159 565.985 824.263C578.598 818.488 613.873 805.037 607.524 785.528C577.329 692.496 544.308 599.676 481.835 522.906C452.246 486.519 413.225 453.179 366.54 449.409C325.241 446.094 285.584 466.597 252.783 491.946C164.251 560.282 109.233 663.854 61.2223 762.395L61.0224 762.359Z"
                        fill="#E1620F" />
                    <path
                        d="M380.594 902.484C418.831 875.984 428.392 823.572 401.95 785.417C375.508 747.262 323.075 737.814 284.838 764.313C246.601 790.812 237.039 843.224 263.482 881.379C289.924 919.534 342.357 928.983 380.594 902.484Z"
                        fill="#E1620F" />
                </g>
            </svg>
        </div>
        <div
            class="absolute right-0  bottom-[40%] w-[20%] h-full opacity-10 select-none pointer-events-none z-0 flex items-end justify-end">
            <svg xmlns="http://www.w3.org/2000/svg" width="402" height="671" viewBox="0 0 402 671" fill="none">
                <g>
                    <path
                        d="M762.348 442.944C779.03 420.859 833.296 368.71 783.997 349.608C711.599 321.53 644.693 279.142 588.547 225.403C570.982 208.613 553.571 189.068 550.199 165.011C545.256 130.079 571.362 98.4737 598.873 76.2742C652.427 33.2729 716.737 7.13945 781.175 -14.8525C816.883 -27.0057 836.603 -24.2678 818.249 -62.7182C812.325 -75.2616 798.455 -110.374 779.023 -103.794C686.356 -72.4954 593.936 -38.3745 517.912 25.0064C481.88 55.0254 449.006 94.4392 445.791 141.166C442.966 182.501 463.939 221.912 489.676 254.409C559.058 342.124 663.277 395.907 762.381 442.744L762.348 442.944Z"
                        fill="#E1620F" />
                    <path
                        d="M761.76 219.039C734.808 181.119 743.633 128.578 781.471 101.684C819.309 74.7906 871.832 83.7288 898.783 121.648C925.735 159.568 916.909 212.109 879.071 239.003C841.234 265.896 788.711 256.958 761.76 219.039Z"
                        fill="#E1620F" />
                    <path
                        d="M525.129 611.549C498.787 620.044 431.696 654.15 429.864 601.311C427.146 523.706 409.11 446.584 376.816 375.893C366.733 353.784 353.998 330.915 332.388 319.822C301.022 303.67 262.591 317.934 232.581 336.619C174.365 373.059 128.542 425.203 86.5882 478.83C63.3715 508.558 59.4746 528.081 29.1952 498.107C19.2964 488.389 -9.30477 463.747 3.29811 447.559C63.3209 370.331 125.928 294.265 210.779 243.304C250.975 219.143 299.005 201.053 344.192 213.378C384.158 224.299 414.484 257.062 436.714 292.053C496.743 386.415 513.276 502.521 524.929 611.515L525.129 611.549Z"
                        fill="#E1620F" />
                    <path
                        d="M176.955 634.692C150.004 596.773 158.829 544.232 196.667 517.338C234.505 490.444 287.027 499.383 313.979 537.302C340.93 575.222 332.105 627.763 294.267 654.657C256.429 681.55 203.907 672.612 176.955 634.692Z"
                        fill="#E1620F" />
                </g>
            </svg>
        </div>
        <!-- Breadcrumb & titre -->
        <div class="z-10 pt-4 pl-6 text-xs text-gray-600">Quiz &gt; Étude post-bac</div>
        <div class="z-10 px-6 pt-2">
            <h2 class="font-bold text-lg md:text-xl">Quiz étude post-bac – Trouve ta voie avec nos quiz d’orientation
            </h2>
            <div class="text-xs text-gray-500 mt-1">L’orientation après le bac peut être un vrai casse-tête.<br>Pas de
                panique, nos quiz interactifs t’aident à y voir plus clair en quelques minutes !</div>
        </div>
        <!-- Carte question -->
        <div class="flex flex-1 items-center justify-center z-10">
            <div
                class="w-full max-w-2xl rounded-2xl bg-gradient-to-b from-[#EE7213] to-[#F09A4E] shadow-lg px-6 py-8 relative">
                <!-- Badge question -->
                <div class="absolute -top-6 left-1/2 -translate-x-1/2">
                    <div class="bg-white border-2 border-[#EE7213] rounded-t-xl rounded-b-lg px-8 py-2 font-semibold text-black shadow text-base"
                        style="transform: rotate(-2deg);">
                        Question {{ currentSection * 1 + 1 }}/7
                    </div>
                </div>
                <!-- Question -->
                <div class="mt-8 text-center text-white text-xl font-semibold mb-8">
                    {{ sections[currentSection].questions[currentQuestion].question }}
                </div>
                <div class="grid grid-cols-2 gap-4 mb-8">
                    <button v-for="(option, idx) in sections[currentSection].questions[currentQuestion].options"
                        :key="idx"
                        :class="['rounded-lg border border-white py-3 font-semibold text-base transition-all',
                            selectedOption === option ? 'bg-white text-[#EE7213]' : 'bg-transparent text-white hover:bg-white hover:text-[#EE7213]']"
                        @click="selectOption(option)">
                        {{ option }}
                    </button>
                </div>
                <!-- Bouton suivant -->
                <div
                    v-if="currentSection == sections.length - 1 && currentQuestion == sections[currentSection].questions.length - 1">
                    <button
                        class="w-full py-3 rounded-lg bg-[#F6CBA3] text-[#A97A50] font-semibold text-base mt-2 transition-all hover:brightness-105"
                        @click="finishQuiz">
                        Terminer le quiz
                    </button>
                </div>
                <div v-else>
                    <button
                        class="w-full py-3 rounded-lg bg-[#F6CBA3] text-[#A97A50] font-semibold text-base mt-2 transition-all hover:brightness-105"
                        @click="nextSection">
                        Question suivante
                    </button>
                </div>

            </div>
        </div>
    </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Pacifico&display=swap');

.font-handwriting {
    font-family: 'Pacifico', cursive;
}
</style>