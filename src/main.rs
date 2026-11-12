use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

fn nettoyer_telephone(tel: &str) -> String {
    let chiffres: String = tel.chars().filter(|c| c.is_ascii_digit()).collect();
    if chiffres.starts_with("212") && chiffres.len() == 12 {
        format!("0{}", &chiffres[3..])
    } else if chiffres.len() == 9 && (chiffres.starts_with('6') || chiffres.starts_with('7')) {
        format!("0{}", chiffres)
    } else {
        chiffres
    }
}

fn main() {
    println!("=================================================");
    println!("   NETTOYEUR & FORMATEUR DE LISTES E-COMMERCE    ");
    println!("=================================================\n");

    let args: Vec<String> = env::args().collect();
    let fichier_entree = args.get(1).map(|s| s.as_str()).unwrap_or("clients.csv");
    let fichier_sortie = "clients_propres.csv";

    let file = match File::open(fichier_entree) {
        Ok(f) => f,
        Err(_) => {
            println!("⚠️ Fichier '{}' introuvable.", fichier_entree);
            println!("Création d'un fichier d'exemple 'clients.csv' pour tester...");
            let exemple = "Nom,Email,Telephone\nAhmed, AHMED@gmail.com , +212 6 12 34 56 78\nSara,sara@yahoo.fr,0611223344\nAhmed,ahmed@gmail.com,0612345678\nYoussef,youssef@outlook.com,700112233\n";
            fs::write("clients.csv", exemple).expect("Impossible de créer le fichier exemple");
            File::open("clients.csv").unwrap()
        }
    };

    let reader = BufReader::new(file);
    let mut lignes_uniques = HashSet::new();
    let mut resultat = Vec::new();
    let mut total_lues = 0;
    let mut doublons = 0;

    for (index, line) in reader.lines().enumerate() {
        if let Ok(ligne) = line {
            let ligne_propre = ligne.trim();
            if ligne_propre.is_empty() {
                continue;
            }

            if index == 0 {
                resultat.push(ligne_propre.to_string());
                continue;
            }

            total_lues += 1;
            let colonnes: Vec<&str> = ligne_propre.split(',').collect();

            if let (Some(&nom_brut), Some(&email_brut), Some(&tel_brut)) =
                (colonnes.first(), colonnes.get(1), colonnes.get(2))
            {
                let nom = nom_brut.trim();
                let email = email_brut.trim().to_lowercase();
                let tel = nettoyer_telephone(tel_brut);

                let cle_unique = format!("{}-{}", email, tel);
                if lignes_uniques.contains(&cle_unique) {
                    doublons += 1;
                } else {
                    lignes_uniques.insert(cle_unique);
                    resultat.push(format!("{},{},{}", nom, email, tel));
                }
            }
        }
    }

    let mut output = File::create(fichier_sortie).expect("Erreur création fichier sortie");
    for l in &resultat {
        writeln!(output, "{}", l).unwrap();
    }

    println!("✅ Traitement terminé avec succès !");
    println!("📊 Lignes analysées : {}", total_lues);
    println!("🗑️ Doublons supprimés : {}", doublons);
    println!("📁 Fichier propre généré : '{}'", fichier_sortie);
}
