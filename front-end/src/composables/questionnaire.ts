import { ref } from "vue";

export const questionnaire = ref({
    section1: {
      title: "Informations Générales",
      questions: [
        {
          question: "Âge:",
          options: ""
        },
        {
          question: "Genre:",
          options: ["Homme", "Femme", "Autre", "Préfère ne pas dire"]
        },
        {
          question: "Niveau d'éducation actuel:",
          options: ["Collège", "Lycée", "Études supérieures (Bac+1 à Bac+3)", "Études supérieures (Bac+4 et plus)", "Autre (précisez)"]
        }
      ]
    },
    section2: {
      title: "Intérêts Personnels",
      questions: [
        {
          question: "Quels sont vos passe-temps ou activités préférées?",
          options: ["Lire", "Jouer à des jeux vidéo", "Faire du sport", "Regarder des films/séries", "Faire de la musique", "Dessiner/Peindre", "Autre (précisez)"]
        },
        {
          question: "Quels types de livres ou de sujets aimez-vous lire ou étudier?",
          options: ["Romans", "Sciences", "Histoire", "Technologie", "Art", "Autre (précisez)"]
        }
      ]
    },
    section3: {
      title: "Compétences et Aptitudes",
      questions: [
        {
          question: "Quelles compétences pensez-vous avoir?",
          options: ["Communication", "Travail en équipe", "Résolution de problèmes", "Créativité", "Compétences techniques (informatique, etc.)", "Autre (précisez)"]
        },
        {
          question: "Quelles matières scolaires préférez-vous?",
          options: ["Mathématiques", "Sciences", "Langues", "Histoire-Géographie", "Arts", "Autre (précisez)"]
        }
      ]
    },
    section4: {
      title: "Préférences Professionnelles",
      questions: [
        {
          question: "Préférez-vous travailler:",
          options: ["En intérieur", "En extérieur", "Les deux"]
        },
        {
          question: "Préférez-vous travailler:",
          options: ["Seul", "En équipe", "Les deux"]
        },
        {
          question: "Quel type d'environnement de travail préférez-vous?",
          options: ["Bureau", "Laboratoire", "Atelier", "Plein air", "Autre (précisez)"]
        }
      ]
    },
    section5: {
      title: "Objectifs et Valeurs",
      questions: [
        {
          question: "Quels sont vos objectifs professionnels à long terme?",
          options: ["Stabilité", "Évolution de carrière", "Équilibre vie professionnelle/vie personnelle", "Autre (précisez)"]
        },
        {
          question: "Quelles valeurs sont importantes pour vous dans un emploi?",
          options: ["Rémunération", "Reconnaissance", "Flexibilité", "Impact social", "Autre (précisez)"]
        }
      ]
    },
    section6: {
      title: "Expériences et Stages",
      questions: [
        {
          question: "Avez-vous déjà fait des stages ou des emplois à temps partiel? Si oui, lesquels?",
          options: ["Oui (précisez)", "Non"]
        },
        {
          question: "Quelles compétences ou expériences avez-vous acquises lors de ces stages ou emplois?",
          options: ["Communication", "Gestion du temps", "Compétences techniques", "Autre (précisez)"]
        }
      ]
    },
    section7: {
      title: "Aspirations et Rêves",
      questions: [
        {
          question: "Si vous pouviez choisir n'importe quel métier, quel serait-il?",
          options: ["Médecin", "Ingénieur", "Artiste", "Enseignant", "Autre (précisez)"]
        },
        {
          question: "Pourquoi ce métier vous attire-t-il?",
          options: ["Passion pour le domaine", "Opportunités de carrière", "Impact sur la société", "Autre (précisez)"]
        }
      ]
    }
  });