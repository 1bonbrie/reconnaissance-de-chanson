use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

pub enum Commande { Inserer, Reconnaitre }

pub fn initialiser_db(chemin: &str) -> Result<Connection> {
    let conn = Connection::open(chemin)?;
    conn.execute("CREATE TABLE IF NOT EXISTS empreintes (nom TEXT, f1 INTEGER, f2 INTEGER, dt INTEGER)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_empreinte ON empreintes(f1, f2, dt)", [])?;
    Ok(conn)
}

pub fn utiliser_db(conn: &mut Connection, cmd: Commande, nom: Option<&str>, empreintes: &[(usize, usize, usize)]) -> Result<String> {
    match cmd {
        Commande::Inserer => {
            let nom_chanson = nom.unwrap_or("inconnu");
            for (f1, f2, dt) in empreintes {
                conn.execute("INSERT INTO empreintes (nom, f1, f2, dt) VALUES (?1, ?2, ?3, ?4)", params![nom_chanson, f1, f2, dt])?;
            }
            Ok("Chanson ajoutée !".to_string())
        }
        Commande::Reconnaitre => {
            let mut scores: HashMap<String, usize> = HashMap::new();
            for (f1, f2, dt) in empreintes {
                let mut stmt = conn.prepare("SELECT nom FROM empreintes WHERE f1 = ? AND f2 = ? AND dt = ?")?;
                let noms = stmt.query_map(params![f1, f2, dt], |r| r.get::<_, String>(0))?;
                
                for n in noms {
                    *scores.entry(n?).or_insert(0) += 1;
                }
            }
            // On prend la chanson avec le plus de points
            let correspondance = scores.into_iter().max_by_key(|(_, score)| *score);
            match correspondance {
                Some((nom, pts)) if pts > 5 => Ok(format!("Match: {} ({} pts)", nom, pts)),
                _ => Ok("Aucun match trouvé".to_string())
            }
        }
    }
}