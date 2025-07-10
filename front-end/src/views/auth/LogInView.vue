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
import { UserLogin } from '@/composables/auth';
import { useRouter } from 'vue-router';

const router = useRouter();
const authStore = useAuth();
const initialValues = ref<UserLogin>({
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

const forgotpassword = () =>{
    router.push({ name: 'forgot-password' });
}

const home = () => {
    router.push({ name: 'home' });
}

const onGoogleSubmit = (e: any) => {
    console.log(initialValues.value);
    nextTick(() => {
        window.open('/api/auth/google', '_self');
    });
    
};
</script>

<template>
    <div class="flex items-center justify-center h-screen ">
        <Toast></Toast>
        <Card>
            <template #title>
                <button @click="home"><img class="w-23 h-20 mx-auto"  src="../../images/logo.svg" alt="Logo" /></button>
                <h2 class="text-center mt-2">Login</h2>
            </template>
            <template #content>
                
                <Form v-slot="$form" :initialValues :resolver  >
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
                        <button @click="forgotpassword" class="text-orange">Mot de passe oublié ?</button>
                    </div>
                    <Button @click="onGoogleSubmit" severity="secondary"  class="w-full flex items-center justify-center gap-2 mt-4">
                        <img src="../../images/Google_logo.svg" class="h-5 w-5" alt="Goolge Logo" />
                        <p>Se connecter avec Google</p>
                    </Button>
                    <OriensButton class="mt-2 w-full items-center justify-center " size="small" label="Se Connecter"></OriensButton>
                    <div class="flex justify-center mt-4">
                        <p>Vous n'avez pas de compte ?</p>
                        <button @click="router.push({ name: 'signup' })" class="text-orange  ml-2">S'inscrire</button>
                    </div>
                </Form>
            </template>
        </Card>

    </div>
</template>


<style lang="css" scoped>

</style>