use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;
use actix_cors::Cors;

use crate::{sample, fingerprint, db};

async fn enregistrer_fichier(mut payload: Multipart, nom: &str) -> Result<String, String> {
    let mut nom_original = String::new();
    while let Some(Ok(mut field)) = payload.next().await {
        if nom_original.is_empty() {
            nom_original = field.content_disposition().get_filename().unwrap_or("inconnu").to_string();
        }
        let mut f = File::create(nom).map_err(|e| e.to_string())?;
        while let Some(Ok(chunk)) = field.next().await {
            f.write_all(&chunk).map_err(|e| e.to_string())?;
        }
    }
    Ok(nom_original)
}

pub async fn inserer_chanson(payload: Multipart) -> impl Responder {
    let nom_fichier = enregistrer_fichier(payload, "upload.mp3").await.unwrap();
    
    let meta = sample::obtenir_metadonnees("upload.mp3").unwrap();
    let spectro = fingerprint::generer_spectrogramme(meta);
    let pics = fingerprint::trouver_pics(&spectro, 5);
    let empreintes = fingerprint::generer_empreintes(&pics, 5, 200);

    let nom_propre = nom_fichier.replace(".mp3", "");
    let mut conn = db::initialiser_db("empreintes.db").unwrap();
    let res = db::utiliser_db(&mut conn, db::Commande::Inserer, Some(&nom_propre), &empreintes);

    HttpResponse::Ok().body(res.unwrap_or_else(|e| e.to_string()))
}

pub async fn reconnaitre_chanson(payload: Multipart) -> impl Responder {
    enregistrer_fichier(payload, "recherche.mp3").await.unwrap();

    let meta = sample::obtenir_metadonnees("recherche.mp3").unwrap();
    let spectro = fingerprint::generer_spectrogramme(meta);
    let pics = fingerprint::trouver_pics(&spectro, 5);
    let empreintes = fingerprint::generer_empreintes(&pics, 200);

    let mut conn = db::initialiser_db("empreintes.db").unwrap();
    let res = db::utiliser_db(&mut conn, db::Commande::Reconnaitre, None, &empreintes);

    HttpResponse::Ok().body(res.unwrap_or_else(|e| e.to_string()))
}

pub async fn demarrer_serveur_web() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/inserer", web::post().to(inserer_chanson))
            .route("/reconnaitre", web::post().to(reconnaitre_chanson))
    })
    .bind(("127.0.0.1", 8080))?.run().await
}