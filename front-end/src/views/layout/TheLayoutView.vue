<script setup>
import HeaderHomeView from '@/components/layout/HeaderHomeView.vue';
import Footer from '@/components/layout/Footer.vue';
</script>

<template>
    <div class="app-layout">
        <HeaderHomeView />
        <div class="content">
            <main class="main-content">
                <router-view />
            </main>
        </div>
        <footer>
            <Footer></Footer>
        </footer>
    </div>
</template>