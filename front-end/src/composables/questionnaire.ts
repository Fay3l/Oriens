import { ref } from "vue";

export class Question{
  question: string;
  options: string[];
  response: string;
  constructor(question: string, options: string[]){
    this.question = question;
    this.options = options;
    this.response = "";
  }
}

export class Section{
  title: string;
  questions: Question[];
  constructor(title: string, questions: Question[]){
    this.title = title;
    this.questions = questions;
  }
}
export class Questionnaire {
  sections: Section[];
  constructor(sections: Section[]){
    this.sections = sections;
  }
}

const questionnaireData = new Questionnaire([
  new Section("Informations Générales", [
    new Question("Âge:", []),
    new Question("Genre:", ["Homme", "Femme", "Autre", "Préfère ne pas dire"]),
    new Question("Niveau d'éducation actuel:", ["Collège", "Lycée", "Études supérieures (Bac+1 à Bac+3)", "Études supérieures (Bac+4 et plus)", "Autre"])
  ]),
  new Section("Intérêts Personnels", [
    new Question("Quels sont vos passe-temps ou activités préférées?", ["Lire", "Jouer à des jeux vidéo", "Faire du sport", "Regarder des films/séries", "Faire de la musique", "Dessiner/Peindre", "Autre"]),
    new Question("Quels types de livres ou de sujets aimez-vous lire ou étudier?", ["Romans", "Sciences", "Histoire", "Technologie", "Art", "Autre"])
  ]),
  new Section("Compétences et Aptitudes", [
    new Question("Quelles compétences pensez-vous avoir?", ["Communication", "Travail en équipe", "Résolution de problèmes", "Créativité", "Compétences techniques (informatique, etc.)", "Autre"]),
    new Question("Quelles matières scolaires préférez-vous?", ["Mathématiques", "Sciences", "Langues", "Histoire-Géographie", "Arts", "Autre"])
  ]),
  new Section("Préférences Professionnelles", [
    new Question("Préférez-vous travailler:", ["En intérieur", "En extérieur", "Les deux"]),
    new Question("Préférez-vous travailler:", ["Seul", "En équipe", "Les deux"]),
    new Question("Quel type d'environnement de travail préférez-vous?", ["Bureau", "Laboratoire", "Atelier", "Plein air", "Autre"])
  ]),
  new Section("Objectifs et Valeurs", [
    new Question("Quels sont vos objectifs professionnels à long terme?", ["Stabilité", "Évolution de carrière", "Équilibre vie professionnelle/vie personnelle", "Autre"]),
    new Question("Quelles valeurs sont importantes pour vous dans un emploi?", ["Rémunération", "Reconnaissance", "Flexibilité", "Impact social", "Autre"])
  ]),
  new Section("Expériences et Stages", [
    new Question("Avez-vous déjà fait des stages ou des emplois à temps partiel? Si oui, lesquels?", ["Oui (précisez)", "Non"]),
    new Question("Quelles compétences ou expériences avez-vous acquises lors de ces stages ou emplois?", ["Communication", "Gestion du temps", "Compétences techniques", "Autre"])
  ]),
  new Section("Aspirations et Rêves", [
    new Question("Si vous pouviez choisir n'importe quel métier, quel serait-il?", ["Médecin", "Ingénieur", "Artiste", "Enseignant", "Autre"]),
    new Question("Pourquoi ce métier vous attire-t-il?", ["Passion pour le domaine", "Opportunités de carrière", "Impact sur la société", "Autre"])
  ])
]);

export const questionnaire = ref(questionnaireData);

export const data_jobs = ref([
  {
      categorie:'Ouvrier',
      titre:'Lorem ipsum dolor sit amet lorem',
      description:'Lorem ipsum dolor dit amet'
  },
  {
      categorie:'Ouvrier',
      titre:'Lorem ipsum dolor sit amet lorem',
      description:'Lorem ipsum dolor dit amet'
  },
  {
      categorie:'Ouvrier',
      titre:'Lorem ipsum dolor sit amet lorem',
      description:'Lorem ipsum dolor dit amet'
  },
  {
      categorie:'Ouvrier',
      titre:'Lorem ipsum dolor sit amet lorem',
      description:'Lorem ipsum dolor dit amet'
  },
  {
      categorie:'Ouvrier',
      titre:'Lorem ipsum dolor sit amet lorem',
      description:'Lorem ipsum dolor dit amet'
  },
  {
      categorie:'Ouvrier',
      titre:'Lorem ipsum dolor sit amet lorem',
      description:'Lorem ipsum dolor dit amet'
  },
])


export class ResponseQuiz {
  adjectif: string;
  description: string;
  formations: string[];
  metiers: string[];
  softskills: string[];
  constructor(adjectif: string, description: string, formations: string[], metiers: string[], softskills: string[]) {
    this.adjectif = adjectif;
    this.description = description;
    this.formations = formations;
    this.metiers = metiers;
    this.softskills = softskills;
  }
}