<script setup lang="ts">
import { Card, Button } from 'primevue';
import { questionnaire } from '../composables/questionnaire';
import { ref } from 'vue';
const currentSection = ref(0);
const sections = [
    questionnaire.value.section1,
    questionnaire.value.section2,
    questionnaire.value.section3,
    questionnaire.value.section4,
    questionnaire.value.section5,
    questionnaire.value.section6,
    questionnaire.value.section7
];

const responses = ref<{ [key: string]: string }>({});
const form = ref<{ [key: string]: { [key: string]: string } }>({});

const nextSection = () => {
    if (currentSection.value < sections.length - 1) {
        currentSection.value++;
    }
};

const prevSection = () => {
    if (currentSection.value > 0) {
        currentSection.value--;
    }
};

</script>

<template>
    <div class="flex flex-col items-center justify-center min-h-screen">
        <Card class="w-full max-w-2xl">
            <template #title>{{ sections[currentSection].title }} {{ currentSection }}</template>
            <template #content>
                <div v-for="(question, index) in sections[currentSection.valueOf()].questions" :key="index">
                    <Card class="m-5">
                        <template #title>{{ question.question }}</template>
                        <template #content>
                            <div v-if="question.options.length > 0" v-for="(option, idx) in question.options" :key="idx"
                                class="m-1">
                                <input type="radio" class="m-1" :name="'question' + index" :value="option" @change="(event:any) =>responses[question.question] = option" />
                                <label>{{ option }}</label>
                            </div>
                            <div v-else>
                                <input type="text" :name="'question' + index" @input="(event:any) => { responses[question.question] = event.target.value }"  />
                            </div>
                        </template>
                    </Card>
                </div>
            </template>
        </Card>
        <div class="flex justify-center mt-3 gap-5">
            <Button v-if="currentSection > 0" @click="prevSection">Précédent</Button>
            <Button v-if="currentSection < sections.length - 1" @click="nextSection">Suivant</Button>
            <Button v-if="currentSection >= sections.length - 1" @click="()=>{console.log(responses);}">Submit</Button>
        </div>
    </div>
</template>