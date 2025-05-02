
<script setup lang="ts">
import JitsiMeetJS from './../../lib-jitsi-meet.min.js';
JitsiMeetJS.init();
</script>

<template>
    <div class="flex flex-col items-center justify-center h-screen">
        <h1 class="text-2xl font-bold mb-4">Jitsi Meet</h1>
        <JitsiMeeting></JitsiMeeting>
    </div>
</template>

<style scoped lang="css">

</style>