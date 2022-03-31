fn main() {
    println!("Quel est le flag qui vous a, précédemment, était donné pour continuer l'épreuve ? :");

    loop {
        use std::io::{stdin,stdout,Write};
        let mut reponse=String::new();
        println!("Réponse :");
        let _=stdout().flush();
        stdin().read_line(&mut reponse).expect("");

        if reponse.contains("3n_@vAnt_!") {
                println!("L'attaque informatique qui vous permettra d'intercepter les communications entre Alice et Bob est bien celle du Man-In-The-Middle.");
                use std::{thread, time};
                thread::sleep(time::Duration::from_secs(5));            
                println!("Maintenant, vous allez réaliser cette attaque en utilisant le commande arpspoof sous la forme :");
                thread::sleep(time::Duration::from_secs(3));            
                println!("sudo arpspoof <arg1> eth0 <arg2> <ip de la cible> <ip destination> > /dev/null 2>&1 &");
                thread::sleep(time::Duration::from_secs(3));
                println!("<arg1> correspond à l'argument pour spécifier l'interface à utiliser, qui est eth0.");
                thread::sleep(time::Duration::from_secs(2));            
                println!("<arg2> correspond à l'argument pour spécifier l'adresse ip de la cible.");
                thread::sleep(time::Duration::from_secs(2));            
                println!("Vous devez trouver un moyen pour connaître ces arguments et pour savoir les adresses ip de Bob et d'Alice, eux et vous sont dans le même réseau.");
                thread::sleep(time::Duration::from_secs(5));            
                println!("Vous devez aussi trouver un moyen de sniffer les paquets qu'ils s'envoient et de lire leurs contenus, ils ne sont pas chiffrés, se sont des trames ICMP.");
                thread::sleep(time::Duration::from_secs(5));            
                println!("Le mot de passe d'Eve et 'Eve'.");
                thread::sleep(time::Duration::from_secs(2));
                println!("Attention, le '> /dev/null 2>&1 &' à la fin de la commande sert à envoyer le flux vers /dev/null et de faire la commande en arrière-plan.");
                thread::sleep(time::Duration::from_secs(5));            
                println!("Après avoir sniffé les communications d'Alice et Bob, exécutez le fichier 'fin'.");
                break;
        } else {
                println!("Mauvaise réponse, veuillez réessayer.");
        }
    }
}