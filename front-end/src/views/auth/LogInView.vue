<script lang="ts" setup>
import { nextTick, ref } from 'vue';
import { Form } from '@primevue/forms';
import Button from 'primevue/button';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { z } from 'zod';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import Card from 'primevue/card';
import OriensButton from '@/components/button/OriensButton.vue';
import { useAuth } from '@/stores/useAuth';

const authStore = useAuth();
const initialValues = ref({
    email: '',
    firstname: '',
    lastname: '',
    password: ''
})

const resolver = zodResolver(
    z.object({
        email: z
            .string()
            .email({ message: 'Invalid email address.' }),
        password: z
            .string()
    })
);

const handleLogin = () => {
    console.log(initialValues.value);
    fetch('/api/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(initialValues.value)
    })
        .then(response => response.json())
        .then(data => {
            localStorage.setItem('token', data.token);
            console.log('Success:', data);
        })
        .catch((error) => {
            console.error('Error:', error);
        });

};
const onGoogleSubmit = (e: any) => {
    console.log(initialValues.value);
    nextTick(() => {
        window.open('/api/auth/google', '_self');
    });
    
};
</script>

<template>
    <div class="flex items-center justify-center h-screen ">
        <Card>
            <template #title>
                <img class="w-23 h-20 mx-auto" src="../../images/logo.svg" alt="Logo" />
                <h2 class="text-center mt-2">Login</h2>
            </template>
            <template #content>
                
                <Form v-slot="$form" :initialValues :resolver @submit="handleLogin" >
                    <div class="flex flex-col gap-2 mt-4">
                        <label for="email" class="font-bold">Email</label>
                        <InputText v-model="initialValues.email" name="email" type="email" placeholder="Email" fluid />
                        <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
                            $form.email.error.message }}</Message>
                    </div>
                    <div class="flex flex-col gap-2 mt-4">
                        <label for="password" class="font-bold">Mot de passe</label>
                        <Password v-model="initialValues.password" name="password" placeholder="Mot de passe"
                            :feedback="false" toggleMask fluid />
                        <Message v-if="$form.password?.invalid" severity="error" size="small" variant="simple">{{
                            $form.password.error.message }}</Message>
                    </div>
                    <Button @click="onGoogleSubmit" severity="secondary"  class="w-full flex items-center justify-center gap-2 mt-4">
                        <img src="../../images/Google_logo.svg" class="h-5 w-5" alt="Goolge Logo" />
                        <p>Se connecter avec Google</p>
                    </Button>
                    <OriensButton class="mt-2 w-full items-center justify-center " size="small" label="Se Connecter"></OriensButton>
                </Form>
            </template>
        </Card>

    </div>
</template>


<style lang="css" scoped>

</style>