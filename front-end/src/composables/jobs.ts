import { computed, onMounted, ref } from "vue";

export class Jobs {
    identifiant: string;
    nom_metier: string;
    acces_metier: string;
    competences: string;
    constructor(identifiant: string, nom_metier: string, acces_metier: string, competences: string) {
        this.identifiant = identifiant;
        this.nom_metier = nom_metier;
        this.acces_metier = acces_metier;
        this.competences = competences;
    }
}

export async function jobs(page: number, per_page: number):Promise<Jobs[]>{
    const res =  await fetch(`http://localhost:3000/api/jobs?page=${page}&per_page=${per_page}`)
    .then((response) => response.json())
    .then((data:Promise<Jobs[]>) => {
        return data.then(res=>res); 
    })
    .catch((error) => [] as Jobs[]);
    return res;
} 

