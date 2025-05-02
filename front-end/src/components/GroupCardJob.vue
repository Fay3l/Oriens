<script setup lang="ts">
import { Jobs } from '@/composables/jobs';
import CardJob from './CardJob.vue';
const props = defineProps({
    class: {
        type: String,
        default:''
    },
    
    jobs:{
        type: Array<Jobs>,
        default: () => [],
    } 
});
console.log('props', props.jobs);
</script>

<template>
    <div :class="props.class" >
        <div v-for="job in props.jobs" :key="job.identifiant">
            <div>
                <CardJob :job="job" />
            </div>
        </div>
    </div>

</template>

<style lang="css" scoped></style>