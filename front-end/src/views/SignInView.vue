<script setup lang="ts">
import { Form } from '@primevue/forms';
import { ref } from 'vue';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { z } from 'zod';
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
    number_phone: 0,
    age: 0,
    city: '',
    username: '',
    password: '',
    experience:0,
    badges:[]
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
            .number()
            .min(1000000000, { message: 'Phone number must be at least 10 digits.' })
            .max(9999999999, { message: 'Phone number must be at most 10 digits.' }),
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
    // fetch('/api/login', {
    //     method: 'POST',
    //     headers: {
    //         'Content-Type': 'application/json'
    //     },
    //     body: JSON.stringify(initialValues.value)
    // })
    // .then(response => response.json())
    // .then(data => {
    //     console.log('Success:', data);
    // })
    // .catch((error) => {
    //     console.error('Error:', error);
    // });
    console.log(initialValues.value);
};
</script>

<template>
    <div class="flex flex-col mt-20 gap-5 items-center ">
        <div>
            <img src="../images/logo.svg" class="w-70 h-20">
        </div>
        <div>
            <Form v-slot="$form" :initialValues :resolver @submit="onFormSubmit"
                class="flex flex-col gap-4 w-full sm:w-60">
                <div class="flex flex-col gap-1">
                    <InputText v-model="initialValues.firstname" name="firstname" type="text" placeholder="Prénom" fluid />
                    <Message v-if="$form.firstname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.firstname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <InputText v-model="initialValues.lastname" name="lastname" type="text" placeholder="Nom de famille" fluid />
                    <Message v-if="$form.lastname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.lastname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <InputText v-model="initialValues.username" name="username" type="text" placeholder="Username" fluid />
                    <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">{{
                        $form.username.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <InputNumber v-model="initialValues.age" inputId="age" :maxFractionDigits="3" fluid />
                </div>
                <div class="flex flex-col gap-1">
                    <InputText v-model="initialValues.email" name="email" type="email" placeholder="Email" fluid />
                    <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
                        $form.email.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <InputText v-model="initialValues.address" name="address" type="text" placeholder="Adresse" fluid />
                    <Message v-if="$form.firstname?.invalid" severity="error" size="small" variant="simple">{{
                        $form.firstname.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <InputNumber v-model="initialValues.postalcode" inputId="postalcode" :maxFractionDigits="5" fluid />
                </div>
                <div class="flex flex-col gap-1">
                    <InputText v-model="initialValues.city" name="city" type="text" placeholder="Ville" fluid />
                    <Message v-if="$form.city?.invalid" severity="error" size="small" variant="simple">{{
                        $form.city.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <InputNumber v-model="initialValues.number_phone" inputId="numberphone" :maxFractionDigits="10" fluid />
                    <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">{{
                        $form.username.error.message }}</Message>
                </div>
                <div class="flex flex-col gap-1">
                    <Password v-model="initialValues.password" name="password" placeholder="Mot de passe" :feedback="false" toggleMask fluid />
                    <Message v-if="$form.password?.invalid" severity="error" size="small" variant="simple">
                        <ul class="flex flex-col gap-1">
                            <li v-for="(error, index) in $form.password.errors" :key="index">{{ error.message }}</li>
                        </ul>
                    </Message>
                </div>
                <Button type="submit" severity="secondary" label="Confirmer" />
            </Form>
        </div>

    </div>
</template>

<style lang="css" scoped></style>