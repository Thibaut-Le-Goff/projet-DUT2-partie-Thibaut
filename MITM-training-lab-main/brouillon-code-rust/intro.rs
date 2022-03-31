fn main() {

    println!("Bonjour, dans cette épreuve Alice et Bob s'échangent des pings avec une suite de lettres, et de chiffres fesseant quatres caractères, dedans.");
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(5));

    println!("Votre objectif est d'intercepter les 'messages' qu'ils s'envoient entre eux.");
    thread::sleep(time::Duration::from_secs(3));

    println!("Quel est le nom de l'attaque informatique qui vous permettrais de faire cela ? (la réponse doit être sous la forme des initiales du nom de l'attaque, en majuscule, en quatre lettres) :");

    loop {
        use std::io::{stdin,stdout,Write};
        let mut reponse=String::new();
        println!("Réponse :");
        let _=stdout().flush();
        stdin().read_line(&mut reponse).expect("");

        if reponse.contains("MITM") {
                println!("Félicitation ! Voici le flag qui vous permettra de continuer l'épreuve : 3n_@vAnt_!, pour continuer, exécutez le fichier 'arp'.");
                break;
        } else {
                println!("Mauvaise réponse, veuillez réessayer.");
        }
    }
}