<script lang="ts" setup>
import { Form } from '@primevue/forms';
import OriensButton from '@/components/button/OriensButton.vue';
import InputText from 'primevue/inputtext';
import Message from 'primevue/message';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { z } from 'zod';
import { ForgotPassword } from '@/composables/auth';
import { ref } from 'vue';
import { useAuth } from '@/stores/useAuth';
import Toast from 'primevue/toast';
import { useToast } from "primevue/usetoast";
import { useRouter } from 'vue-router';
const toast = useToast();
const router = useRouter();
const authStore = useAuth();
const initialValues = ref<ForgotPassword>({
    email: '',
})

const resolver = zodResolver(
    z.object({
        email: z
            .string()
            .email({ message: 'Invalid email address.' }),

    })
);

const  forgotPassword = async() => {
    if(await authStore.forgotPassword(initialValues.value)){
        toast.add({ severity: 'success', summary: 'Success', detail: 'Email sent successfully!', life: 3000 });
    } else {
        toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to send email.', life: 3000 });
    }
}
</script>

<template>
    <div class="flex flex-col ">
        <Toast position="bottom-center">
            <template #message="slotProps">
                <div class="flex flex-col gap-3 items-start flex-auto">
                    <div class="font-medium text-xl my-4">{{ slotProps.message.summary }}</div>
                    <div class="font-bold text-lg">{{ slotProps.message.detail }}</div>
                </div>
            </template>
        </Toast>
        
        <div class="flex flex-col items-center gap-5 justify-center h-screen">
            <button class="flex items-center justify-center mb-4" @click="router.push({ name: 'home' })">
                <img  src="./../../images/logo.svg" alt="" class=" lg:w-48 h-18 sm: w-full h-18">

            </button>
            
            <Form v-slot="$form" :initialValues="initialValues" :resolver="resolver" @submit="forgotPassword">
                <div class="flex flex-col gap-2 mb-2">
                    <label for="email" class="font-bold">Email</label>
                    <InputText v-model="initialValues.email" name="email" type="email" placeholder="Email" fluid />
                    <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{ $form.email.error.message }}</Message>
                </div>
                <OriensButton label="Envoyer le lien de réinitialisation" type="submit"></OriensButton>
            </Form>
        </div>  
    </div>
    </template>