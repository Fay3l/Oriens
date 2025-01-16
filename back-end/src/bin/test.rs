use quick_xml::events::Event;
use quick_xml::Reader;


#[derive(Debug)]
struct Metier {
    nom_metier: String,
    formats_courts: Vec<String>,
    acces_metier: String,
}
// Onisep_Ideo_Fiches_Metiers_09122024.xml
fn main() {
    // Ouvrir le fichier XML

    // Créer un lecteur XML
    let mut xml_reader = Reader::from_file("Onisep_Ideo_Fiches_Metiers_09122024.xml").unwrap();
    xml_reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut metier = Metier {
        nom_metier: String::new(),
        formats_courts: Vec::new(),
        acces_metier: String::new(),
    };

    let mut in_nom_metier = false;
    let mut in_formats_courts = false;
    let mut in_acces_metier = false;
    let mut in_format_court = false;

    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"nom_metier" => {
                        in_nom_metier = true;
                    }
                    b"formats_courts" => {
                        in_formats_courts = true;
                    }
                    b"acces_metier" => {
                        in_acces_metier = true;
                    }
                    b"format_court" => {
                        if in_formats_courts {
                            in_format_court = true;
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                if in_nom_metier {
                    metier.nom_metier = e.unescape().unwrap().to_string();
                }
                if in_acces_metier {
                    metier.acces_metier = e.unescape().unwrap().to_string();
                }
                if in_format_court {
                    metier.formats_courts.push(e.unescape().unwrap().to_string());
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"nom_metier" => {
                        in_nom_metier = false;
                    }
                    b"formats_courts" => {
                        in_formats_courts = false;
                    }
                    b"acces_metier" => {
                        in_acces_metier = false;
                    }
                    b"format_court" => {
                        if in_formats_courts {
                            in_format_court = false;
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break, // Sortir de la boucle à la fin du fichier
            Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    // Afficher les valeurs extraites
    println!("{:?}", metier);
}