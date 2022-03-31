fn main() {
    use std::process::Command;
/*
Bloque les echos reply afin que le participant soit obligé
de faire deux arpspoof, le flag que Bob envoie à Alice est
visible dans l'echo reply qu'Alice envoie à Bob si le 
participant ne fait qu'un arpspoof en se faisant passer pour
Bob.
*/
    Command::new("iptables")
            .arg("-A")
            .arg("OUTPUT")
            .arg("-p")
            .arg("icmp")
            .arg("--icmp-type")
            .arg("echo-reply")
            .arg("-j")
            .arg("DROP")

            .spawn()
            .expect("ls command failed to start");
/*
Change d'utilisateur afin que le container soit présentable,
si le participant rentre dans le container.
*/
    Command::new("su")
            .arg("Bob")

            .spawn()
            .expect("ls command failed to start");

    loop {
        /*
        ping le container que le participant contrôlera afin que Bob
        sois enregistré dans sa table arp afin de retrouver son addresse
        ip via un ipneigh
        */
        Command::new("ping")
                .arg("-c")
                .arg("1")
                .arg("-p")
                /* 
                suite d'espace afin d'éviter que le participant évite
                de prendre les données transmissent comme étant
                le flag
                */
                .arg("02020202020202020202020")
                .arg("172.17.0.4")
                .arg(">")
                .arg("/dev/null")

                .spawn()
                .expect("ls command failed to start");
        /*
        ping Alice afin de transmetre la flag
        */
        Command::new("ping")
                .arg("-c")
                .arg("40")
                .arg("-p")
                // le flag :
                .arg("464c41473a205028297c4f20")
                .arg("172.17.0.2")
                .arg(">")
                .arg("/dev/null")

                .spawn()
                .expect("ls command failed to start");
    }
}
