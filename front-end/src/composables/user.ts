export class User {
    id: string;
    email: string;
    firstName: string;
    lastName: string;
    role: string;
    badges: string;
    experience: string;
    username: string;
    city: string;
    number_phone: string;
    age: number;
    address: string;
    postalcode: string;
    constructor(options: {
        id: string;
        email: string;
        firstName: string;
        lastName: string;
        role: string;
        badges: string;
        experience: string;
        username: string;
        city: string;
        number_phone: string;
        age: number;
        address: string;
        postalcode: string;
    }) {
        this.id = options.id;
        this.email = options.email;
        this.firstName = options.firstName;
        this.lastName = options.lastName;
        this.role = options.role;
        this.badges = options.badges;
        this.experience = options.experience;
        this.username = options.username;
        this.city = options.city;
        this.number_phone = options.number_phone;
        this.age = options.age;
        this.address = options.address;
        this.postalcode = options.postalcode;
    }

}