<script setup lang="ts">
import { Card, Button, InputNumber } from 'primevue';
import { questionnaire } from '../composables/questionnaire';
import { ref } from 'vue';
const currentSection = ref(0);
const sections = [
    ...questionnaire.value.sections
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
                                <input type="radio" class="m-1"  :name="'question' + index" :value="option" @change="(event:any) =>question.response = option" />
                                <label>{{ option }}</label>
                            </div>
                            <div v-else>
                                <InputNumber size="small" type="text" :name="'question' + index" @input="(event:any) => { question.response = event.target.value }"  />
                            </div>
                        </template>
                    </Card>
                </div>
            </template>
        </Card>
        <div class="flex justify-center mt-3 gap-5">
            <Button v-if="currentSection > 0" @click="prevSection">Précédent</Button>
            <Button v-if="currentSection < sections.length - 1" @click="nextSection">Suivant</Button>
            <Button v-if="currentSection >= sections.length - 1" @click="()=>{console.log(questionnaire)}">Submit</Button>
        </div>
    </div>
</template>