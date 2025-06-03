<script setup lang="ts">
import { Jobs } from '@/composables/jobs';
import CardJob from './CardJob.vue';
import { onMounted } from 'vue';
import { useJobs } from '@/stores/useJobs';
const jobsStore = useJobs();
const props = defineProps({
    class: {
        type: String,
        default:''
    },
    page: {
        type: Number,
        default: 1
    },
    perpage: {
        type: Number,
        default: 16
    },
});
onMounted(()=>{
    jobsStore.getJobs(props.page, props.perpage)
})
</script>

<template>
    <div :class="props.class" >
        <div v-for="job in jobsStore.collection" :key="job.identifiant">
            <div>
                <CardJob :job="job" />
            </div>
        </div>
    </div>

</template>

<style lang="css" scoped></style>