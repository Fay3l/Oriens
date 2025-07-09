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



</script>

<template>
    <div>
        <div class="flex flex-col items-center gap-2 justify-center h-screen">
            <Form v-slot="$form" :initialValues="initialValues" :resolver="resolver" @submit="authStore.forgotPassword(initialValues)">
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