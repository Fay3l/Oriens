import { computed } from "vue";

export class Jobs{
    identifiant: string;
    nom_metier: string;
    acces_metier: string;
    competences:string;
    constructor(identifiant: string, nom_metier: string, acces_metier: string,competences:string) {
        this.identifiant = identifiant;
        this.nom_metier = nom_metier;
        this.acces_metier = acces_metier;
        this.competences = competences;
    }
}

export const jobs = computed(() => { 
    fetch('http://localhost:3000/api/jobs?page=1&per_page=4')
    .then((response) => response.json())
    .then((data:Jobs[]) => {
        console.log('Jobs fetched:', jobs.value);
      return data;
    })
    .catch((error) => console.error('Error fetching jobs:', error));
})