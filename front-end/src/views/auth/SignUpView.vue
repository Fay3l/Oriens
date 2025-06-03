<script setup lang="ts">
import { Form } from '@primevue/forms';
import { nextTick, ref } from 'vue';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { z } from 'zod';
import Message from 'primevue/message';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import { VueSimplePhone, type ParsedPhoneNumber } from 'vue-simple-phone'
import 'vue-simple-phone/themes/light.css' 
import OriensButton from '@/components/button/OriensButton.vue';
import { UserRegister } from '@/composables/auth';
import { useAuth } from '@/stores/useAuth';


const authStore = useAuth();
const number_phone = ref(<ParsedPhoneNumber | undefined>undefined);


const initialValues = ref<UserRegister>({
    firstname: '',
    lastname: '',
    email: '',
    address: '',
    postalcode: 0,
    number_phone: '',
    age: 0,
    city: '',
    username: '',
    password: '',
    role: '',
    experience: 0,
    badges: []
});



const resolver = zodResolver(
    z.object({
        city: z
            .string()
            .regex(/^[a-zA-Z\s]*$/, { message: 'City must contain only letters and spaces.' })
            .optional(),
        email: z
            .string()
            .email({ message: 'Invalid email address.' }),
        number_phone: z
            .string()
            .min(1, { message: "Invalid phone number" }),
        username: z.string().min(1, { message: 'Username is required.' }),
        lastname: z.string().min(1, { message: 'Lastname is required.' }),
        firstname: z.string().min(1, { message: 'Firstname is required.' }),
        password: z
            .string()
            .min(8, { message: 'Minimum 8 characters.' })
            .refine((value: any) => /[a-z]/.test(value), {
                message: 'Must have a lowercase letter.'
            })
            .refine((value: any) => /[A-Z]/.test(value), {
                message: 'Must have an uppercase letter.'
            })
            .refine((value: any) => /\d/.test(value), {
                message: 'Must have a number.'
            })
            .refine((value: any) => /[!;@#$%^&*(),.?":{}|<>]/.test(value), {
                message: 'Must have a special character.'
            })
    })
);

const onGoogleSubmit = (e: any) => {
    console.log(initialValues.value);
    nextTick(() => {
        window.open('/api/auth/google', '_self');
    });
    
};
</script>

<template>
    <div class="flex flex-row h-screen">
        <!-- Left Section -->
        <div class="flex justify-center items-center w-[50%] bg-orange">
            <img src="../../images/welcome_oriens.svg" alt="Welcome Oriens" class="w-full h-full object-cover" />
        </div>
        <!-- Right Section -->
        <div class="flex flex-col justify-center items-center w-1/2 p-10">
            <div class="flex flex-col gap-2 ">
                <h2 class="text-2xl font-bold">Rejoins-nous</h2>
                <p>
                    J'ai déjà un compte. <a href="/auth/login" class="text-orange font-bold">Je me connecte</a>
                </p>
            </div>
            <Form v-slot="$form" :initialValues="initialValues" :resolver="resolver" @submit="authStore.register(initialValues)"
                class="grid grid-cols-2 gap-5 w-full max-w-lg">
                <div class="flex flex-col gap-1">
                    <label for="firstname" class="font-bold">Prénom</label>
                    <InputText v-model="initialValues.firstname" name="firstname" type="text" placeholder="Prénom"
                        fluid />
                    <Message v-if="$form.firstname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.firstname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <label for="lastname" class="font-bold">Nom</label>
                    <InputText v-model="initialValues.lastname" name="lastname" type="text" placeholder="Nom" fluid />
                    <Message v-if="$form.lastname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.lastname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1 col-span-2">
                    <label for="email" class="font-bold">Email</label>
                    <InputText v-model="initialValues.email" name="email" type="email" placeholder="Email" fluid />
                    <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
                        $form.email.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1 col-span-2">
                    <label for="password" class="font-bold">Mot de passe</label>
                    <Password v-model="initialValues.password" name="password" placeholder="Mot de passe"
                        :feedback="false" toggleMask fluid />
                    <Message v-if="$form.password?.invalid" severity="error" size="small" variant="simple">{{
                        $form.password.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1 col-span-2">
                    <label for="number_phone" class="font-bold">Numéro de téléphone</label>
                    <VueSimplePhone region="FR" :countries="['US', 'CN', 'FR', 'SE', 'MX', 'JP']" v-model="number_phone" />
                    <Message v-if="$form.number_phone?.invalid" severity="error" size="small" variant="simple">{{
                        $form.number_phone.error.message }}</Message>
                </div>
                <div class="grid grid-cols-4 items-center col-span-2 ">
                    <div>
                        <div class="flex gap-1 items-center">
                            <input
                                class="appearance-none w-4 h-4 border-2 border-gray-300 rounded-md checked:bg-orange checked:border-orange focus:outline-none m-2"
                                v-model="initialValues.role" id="role1" type="radio" value="HighSchoolStudent" />
                            <label class="text-sm" for="role1">Lycéen/ne</label>
                        </div>
                    </div>
                    <div>
                        <div class="flex gap-1 items-center">
                            <input
                                class="appearance-none w-4 h-4 border-2 border-gray-300 rounded-md checked:bg-orange checked:border-orange focus:outline-none m-2"
                                v-model="initialValues.role" id="role2" type="radio" value="PostBacStudent" />
                            <label class="text-sm" for="role2">Étudiant/te post-bac</label>
                        </div>
                    </div>
                    <div>
                        <div class="flex gap-1 items-center">
                            <input
                                class="appearance-none w-4 h-4 border-2 border-gray-300 rounded-md checked:bg-orange checked:border-orange focus:outline-none"
                                v-model="initialValues.role" id="role3" type="radio" value="ParentStudent" />
                            <label class="text-sm" for="role3">Parent d'élève</label>
                        </div>
                    </div>
                    <div>
                        <div class="flex gap-1 items-center">
                            <input
                                class="appearance-none w-4 h-4 border-2 border-gray-300 rounded-md checked:bg-orange checked:border-orange focus:outline-none"
                                v-model="initialValues.role" id="role4" type="radio" value="Reorientation" />
                            <label class="text-sm" for="role4">En réorientation</label>
                        </div>
                    </div>
                </div>
                <Button @click="onGoogleSubmit"  severity="secondary"  class="col-span-2 mt-4">
                    <img src="../../images/Google_logo.svg" class="h-5 w-5" alt="Google Logo" />
                    <p>Se connecter avec Google</p>
                </Button>
                <div class="col-span-2">
                    <OriensButton class="w-full " size="small" label="S'inscrire"></OriensButton>
                </div>
                
            </Form>
        </div>
    </div>
</template>

<style lang="css" scoped>
.linegradient {
    background: linear-gradient(to bottom, #EE7213 0%, #F09A4E 100%);
}
</style>