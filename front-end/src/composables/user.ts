export class User {
    id: string;
    email: string;
    firstname: string;
    lastname: string;
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
        firstname: string;
        lastname: string;
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
        this.firstname = options.firstname;
        this.lastname = options.lastname;
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

export class UserQuiz {
    id: string;
    adjectif: string;
    description: string;
    formations: string[];
    metiers: string[];
    softskills: string[];

    constructor(options: {
        id: string;
        adjectif: string;
        description: string;
        formations: string[];
        metiers: string[];
        softskills: string[];
    }) {
        this.id = options.id;
        this.adjectif = options.adjectif;
        this.description = options.description;
        this.formations = options.formations;
        this.metiers = options.metiers;
        this.softskills = options.softskills;
    }
}