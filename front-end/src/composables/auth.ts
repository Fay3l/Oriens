import { ref } from 'vue';
import { type UUID } from 'vue-uuid';
export const token = ref<string>('');
export const isAuthenticated = ref<boolean>(false);
export const user = ref<UUID>();

export class UserRegister{
    firstname: string;
    lastname: string;
    email: string;
    address: string;
    postalcode: number;
    number_phone: string;
    age: number;
    city: string;
    username: string;
    password: string;
    role: string;
    experience: number;
    badges: []

    constructor(firstname: string, lastname: string, email: string, address: string, postalcode: number, number_phone: string, age: number, city: string, username: string, password: string, role: string, experience: number, badges: []) {
        this.firstname = firstname;
        this.lastname = lastname;
        this.email = email;
        this.address = address;
        this.postalcode = postalcode;
        this.number_phone = number_phone;
        this.age = age;
        this.city = city;
        this.username = username;
        this.password = password;
        this.role = role;
        this.experience = experience;
        this.badges = badges;
    }
}
export class UserLogin {
    firstname: string;
    lastname: string;
    email: string;
    password: string;

    constructor(email: string, password: string,firstname: string, lastname: string) {
        this.email = email;
        this.password = password;
        this.firstname = firstname;
        this.lastname = lastname;
    }
}

export class ForgotPassword {
    email: string;

    constructor(email: string) {
        this.email = email;
    }
}   