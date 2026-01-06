use minimp3::Decoder;
use std::fs::File;



pub fn obtenir_metadonnees(chemin: &str) -> Result<Vec<f32>, String> {
    const FREQUENCE_ECHANTILLONNAGE: u32 = 44100;
    const DUREE_MINIMALE_SECONDES: f32 = 8.0;

    let mut decodeur = Decoder::new(File::open(chemin).map_err(|_| "Impossible d'ouvrir le fichier audio")?);
    let mut metadonnees = Vec::new();

    while let Ok(trame) = decodeur.next_frame() {
        match trame.channels {
            1 => {
                metadonnees.extend(trame.data.iter().map(|&d| d as f32));
            }
            2 => {
                let taille = trame.data.len();
                let mut i = 0;
                while i + 1 < taille {
                    let gauche = trame.data[i] as f32;
                    let droite = trame.data[i + 1] as f32;
                    let moyenne = (gauche + droite) * 0.5;
                    metadonnees.push(moyenne);
                    i += 2;
                }
            }
            _ => {
                return Err("Le fichier audio n'est pas stéréo ou mono".to_string());
            }
        }
    }

    let duree = metadonnees.len() as f32 / FREQUENCE_ECHANTILLONNAGE as f32;
    if duree < DUREE_MINIMALE_SECONDES {
        return Err(format!(
            "Durée audio trop courte ({:.2} s < {:.2} s)",
            duree, DUREE_MINIMALE_SECONDES
        ));
    }

    Ok(metadonnees)
}





