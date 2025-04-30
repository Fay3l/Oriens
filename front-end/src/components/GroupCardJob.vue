<script setup lang="ts">
import { Jobs } from '@/composables/jobs';
import CardJob from './CardJob.vue';
const props = defineProps({
    class: {
        type: String,
        default:''
    },
    
    job:{
        type: Jobs,
        default: () => ({
            identifiant: "",
            nom_metier: '',
            acces_metier: "Pas d'accès au metier ",
            competences:"",
        })
    } 
});
</script>

<template>
    <div :class="props.class" >
        <div v-for="index in 2" :key="index">
            <div>
                <CardJob :job="props.job" />
            </div>
        </div>
    </div>

</template>

<style lang="css" scoped></style>