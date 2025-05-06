<script setup>
import HeaderHomeView from '@/components/HeaderHomeView.vue';
</script>

<template>
    <div class="app-layout">
        <HeaderHomeView />
        <div class="content">
            <main class="main-content">
                <router-view />
            </main>
        </div>
    </div>
</template>