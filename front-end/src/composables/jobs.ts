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