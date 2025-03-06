<script setup lang="ts">
import Card from 'primevue/card';
</script>

<template>
    <Card class="border">
        <template #content>
            <div class="flex flex-row gap-4 items-center">
                <div class="flex flex-col gap-1" >
                    <div class="flex mb-6">
                        <p class="text-white font-bold bg-orange p-1  ">OUVRIER</p>
                    </div>
                    <p class="font-bold text-xl text-black">Lorem ipsum dolor sit amet lorem</p>
                    <p>Lorem ipsum dolor sit amet lorem</p>
                </div>
                <div>
                    <img class="shadow-img" src="./../images/image_pack.png">
                </div>
            </div>
        </template>
    </Card>
</template>

<style lang="css" scoped>
.shadow-img{
    border-radius: 15px;
    box-shadow: 10px 10px #EE7213;
}
</style>