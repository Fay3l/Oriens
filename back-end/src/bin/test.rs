// use std::fs;
// extern crate serde;
// extern crate quick_xml;
// use quick_xml::{
//     de::{from_reader, from_str}, events::Event, Reader
// };
// use serde::{Deserialize, Serialize};

use mistralai_client::v1::{chat::{ChatMessage, ChatMessageRole, ChatParams, ResponseFormat}, client::Client, constants::Model};
use serde::{Serialize,Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Suggestion {
  pub metiers: Vec<Metier>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Metier{
  pub nom_metier: String,
  pub description: String
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Configuration {
//     #[serde(rename = "metier")]
//     pub metiers: Vec<Metier>,
// }

// Onisep_Ideo_Fiches_Metiers_09122024.xml

fn main() {
    // let file = fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").expect("Cannot read");
    // let res:Configuration = from_str(&file).expect("Deserialize failed :");

    // for metier in res.metiers {
    //     if metier.nom_metier.contains("développeur"){
    //         println!("{:?}",metier);
    //     }
    // }
    // "content": 'Faire une suggestion de metiers avec les réponses du questionnaire. 
    //     Retourner une liste de metiers avec leur description et le nom du metier en format JSON par exemple 
        // [{
        // nom_metier: "",
        // description: ""
        // },{
        // nom_metier: "",
        // description: ""
        // }].'
    let api_key = "SUjoKWZpRQIPhbP2BoxkqT4XGhPpVASV";
    let client = Client::new(Some(api_key.to_string()), None, None, None).unwrap();
    let data = r#"
    [
    {
        "title": "Informations Générales",
        "questions": [
            {
                "question": "Âge:",
                "options": [],
                "response": "18"
            },
            {
                "question": "Genre:",
                "options": [
                    "Homme",
                    "Femme",
                    "Autre",
                    "Préfère ne pas dire"
                ],
                "response": "Homme"
            },
            {
                "question": "Niveau d'éducation actuel:",
                "options": [
                    "Collège",
                    "Lycée",
                    "Études supérieures (Bac+1 à Bac+3)",
                    "Études supérieures (Bac+4 et plus)",
                    "Autre (précisez)"
                ],
                "response": "Lycée"
            }
        ]
    },
    {
        "title": "Intérêts Personnels",
        "questions": [
            {
                "question": "Quels sont vos passe-temps ou activités préférées?",
                "options": [
                    "Lire",
                    "Jouer à des jeux vidéo",
                    "Faire du sport",
                    "Regarder des films/séries",
                    "Faire de la musique",
                    "Dessiner/Peindre",
                    "Autre (précisez)"
                ],
                "response": "Lire"
            },
            {
                "question": "Quels types de livres ou de sujets aimez-vous lire ou étudier?",
                "options": [
                    "Romans",
                    "Sciences",
                    "Histoire",
                    "Technologie",
                    "Art",
                    "Autre (précisez)"
                ],
                "response": "Sciences"
            }
        ]
    },
    {
        "title": "Compétences et Aptitudes",
        "questions": [
            {
                "question": "Quelles compétences pensez-vous avoir?",
                "options": [
                    "Communication",
                    "Travail en équipe",
                    "Résolution de problèmes",
                    "Créativité",
                    "Compétences techniques (informatique, etc.)",
                    "Autre (précisez)"
                ],
                "response": "Travail en équipe"
            },
            {
                "question": "Quelles matières scolaires préférez-vous?",
                "options": [
                    "Mathématiques",
                    "Sciences",
                    "Langues",
                    "Histoire-Géographie",
                    "Arts",
                    "Autre (précisez)"
                ],
                "response": "Sciences"
            }
        ]
    },
    {
        "title": "Préférences Professionnelles",
        "questions": [
            {
                "question": "Préférez-vous travailler:",
                "options": [
                    "En intérieur",
                    "En extérieur",
                    "Les deux"
                ],
                "response": "En intérieur"
            },
            {
                "question": "Préférez-vous travailler:",
                "options": [
                    "Seul",
                    "En équipe",
                    "Les deux"
                ],
                "response": "Seul"
            },
            {
                "question": "Quel type d'environnement de travail préférez-vous?",
                "options": [
                    "Bureau",
                    "Laboratoire",
                    "Atelier",
                    "Plein air",
                    "Autre (précisez)"
                ],
                "response": "Bureau"
            }
        ]
    },
    {
        "title": "Objectifs et Valeurs",
        "questions": [
            {
                "question": "Quels sont vos objectifs professionnels à long terme?",
                "options": [
                    "Stabilité",
                    "Évolution de carrière",
                    "Équilibre vie professionnelle/vie personnelle",
                    "Autre (précisez)"
                ],
                "response": "Évolution de carrière"
            },
            {
                "question": "Quelles valeurs sont importantes pour vous dans un emploi?",
                "options": [
                    "Rémunération",
                    "Reconnaissance",
                    "Flexibilité",
                    "Impact social",
                    "Autre (précisez)"
                ],
                "response": "Rénumération"
            }
        ]
    },
    {
        "title": "Expériences et Stages",
        "questions": [
            {
                "question": "Avez-vous déjà fait des stages ou des emplois à temps partiel? Si oui, lesquels?",
                "options": [
                    "Oui (précisez)",
                    "Non"
                ],
                "response": "Oui (précisez)"
            },
            {
                "question": "Quelles compétences ou expériences avez-vous acquises lors de ces stages ou emplois?",
                "options": [
                    "Communication",
                    "Gestion du temps",
                    "Compétences techniques",
                    "Autre (précisez)"
                ],
                "response": "Gestion du temps"
            }
        ]
    },
    {
        "title": "Aspirations et Rêves",
        "questions": [
            {
                "question": "Si vous pouviez choisir n'importe quel métier, quel serait-il?",
                "options": [
                    "Médecin",
                    "Ingénieur",
                    "Artiste",
                    "Enseignant",
                    "Autre (précisez)"
                ],
                "response": "Autre (précisez)"
            },
            {
                "question": "Pourquoi ce métier vous attire-t-il?",
                "options": [
                    "Passion pour le domaine",
                    "Opportunités de carrière",
                    "Impact sur la société",
                    "Autre (précisez)"
                ],
                "response": "Autre (précisez)"
            }
        ]
    }
]"#;
    let data_modified = data.replace("\n", " ").replace("\r", "");
    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content:format!("Pour la personne qui a répondu au questionnaire, trouver 3-4 métiers qui correspondent aux réponses. Retourner les métiers et les descriptions en objet JSON.{:?}",data.to_string()) ,
        tool_calls: None,

    }];
    let options = ChatParams {
        temperature: 0.7,
        random_seed: Some(42),
        response_format: Option::from(ResponseFormat::json_object()),
        ..Default::default()
    };

    let result = client.chat(model, messages, Some(options)).unwrap();
    println!("Assistant: {}", result.choices[0].message.content);

    // let prompt = format!(r#"{{
    //   "model": "open-mistral-7b",
    //   "messages": [
    //     {{
    //       "role": "user",
    //       "content": "{}"
    //     }}
    //   ],
    //   "response_format": {{"type": "json_object"}}
    // }}"#, data_modified);
    // let client = reqwest::Client::new();
    // let res = client
    //     .post("https://api.mistral.ai/v1/chat/completions")
    //     .header("Content-Type", "application/json")
    //     .header("Accept", "application/json")
    //     .bearer_auth("SUjoKWZpRQIPhbP2BoxkqT4XGhPpVASV")
    //     .body(prompt)
    //     .send()
    //     .await
    //     .expect("Erreur: ");

    // println!("{:?}", res.text().await.unwrap());
}
