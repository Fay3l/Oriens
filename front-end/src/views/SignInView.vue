<script setup lang="ts">
import { Form } from '@primevue/forms';
import { ref } from 'vue';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { z } from 'zod';
import InputMask from 'primevue/inputmask';
import Message from 'primevue/message';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import InputNumber from 'primevue/inputnumber';

const initialValues = ref({
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

const onFormSubmit = (e: any) => {
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
            console.log('Success:', data);
        })
        .catch((error) => {
            console.error('Error:', error);
        });

};
</script>

<template>
    <div class=" ">
        <div>
            <img src="../images/logo.svg" alt="logo" class="w-70 h-20">
        </div>
        <div >
            <Form v-slot="$form" :initialValues :resolver @submit="onFormSubmit"
                class="grid grid-cols-2 gap-4">
                <div class="flex flex-col  gap-1">
                    <IftaLabel>
                        <label for="firstname" class="font-bold">Prénom</label>
                        <InputText v-model="initialValues.firstname" name="firstname" type="text" placeholder="Prénom"
                            fluid />
                    </IftaLabel>
                    <Message v-if="$form.firstname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.firstname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="lastname" class="font-bold">Nom de famille</label>
                        <InputText v-model="initialValues.lastname" name="lastname" type="text"
                            placeholder="Nom de famille" fluid />
                    </IftaLabel>
                    <Message v-if="$form.lastname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.lastname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="username" class="font-bold" >Username</label>
                        <InputText v-model="initialValues.username" name="username" type="text" placeholder="Username"
                        fluid />
                    </IftaLabel>
                    <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">{{
                        $form.username.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="age" class="font-bold">Age</label>
                        <InputNumber v-model="initialValues.age" inputId="age" :maxFractionDigits="3" fluid />
                    </IftaLabel>                    
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="email" class="font-bold">Email</label>
                        <InputText v-model="initialValues.email" name="email" type="email" placeholder="Email" fluid />
                    </IftaLabel>
                    <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
                        $form.email.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="address" class="font-bold">Adresse</label>
                        <InputText v-model="initialValues.address" name="address" type="text" placeholder="Adresse"
                            fluid />
                    </IftaLabel>
                    <Message v-if="$form.firstname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.firstname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="postalcode" class="font-bold"> Code postal</label>
                        <InputNumber v-model="initialValues.postalcode" inputId="postalcode" :maxFractionDigits="5"
                            fluid />
                    </IftaLabel>

                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="city" class="font-bold">Ville</label>
                        <InputText v-model="initialValues.city" name="city" type="text" placeholder="Ville" fluid />
                    </IftaLabel>
                    <Message v-if="$form.city?.invalid" severity="error" size="small" variant="simple">{{
                        $form.city.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="phone" class="font-bold">Phone</label>
                        <InputMask id="phone" v-model="initialValues.number_phone" mask="(999) 999-9999"
                            placeholder="(999) 999-9999" fluid />
                    </IftaLabel>
                </div>
                <div class="flex flex-col gap-1">
                    <IftaLabel>
                        <label for="password" class="font-bold">Mot de passe</label>
                    </IftaLabel>
                    <Password v-model="initialValues.password" name="password" placeholder="Mot de passe"
                        :feedback="false" toggleMask fluid />
                        <Message v-if="$form.password?.invalid" severity="error" size="small" variant="simple">{{
                        $form.password.error.message }}</Message>
                </div>
                <Button type="submit" severity="secondary" label="Confirmer" />
            </Form>
        </div>

    </div>
</template>

<style lang="css" scoped></style>