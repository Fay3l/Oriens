// use std::fs;
// extern crate serde;
// extern crate quick_xml;
// use quick_xml::{
//     de::{from_reader, from_str}, events::Event, Reader
// };
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Metier {
//     pub identifiant: String,
//     pub nom_metier: String,
//     pub acces_metier: String
// }
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
    
}
