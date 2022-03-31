fn main() {

    println!("Quel sont les message qu'Alice et Bob ce sont transmis, la réponse doit être leur message collés, sous la forme de huit lettres et chiffres ?");

    loop {
        use std::io::{stdin,stdout,Write};
        let mut reponse=String::new();
        println!("Réponse :");
        let _=stdout().flush();
        stdin().read_line(&mut reponse).expect("");

        if reponse.contains("a4e73db8") || reponse.contains("3db8a4e7") {
                println!("");
                println!("");
                println!("   ______________________________________________");
                println!("  |                                             |");
                println!("  |               Félicitations !               |");
                println!("  |     Le flag final de cette épreuve est :    |");
                println!("  |                M@n_|n_Bla(k                 |");
                println!("  |_____________________________________________|");
                println!("");
                println!("");
                println!("Pensez à supprimer les containers et les images via la commande 'docker system prune -f -a'.");
                break;
        } else {
                println!("Mauvaise réponse, veuillez réessayer.");
        }
    }
}