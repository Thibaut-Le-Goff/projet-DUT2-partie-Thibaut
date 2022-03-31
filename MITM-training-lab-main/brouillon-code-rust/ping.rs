fn main() {
    use std::process::Command;
/*
Bloque les echos reply afin que le participant soit obligé
de faire deux arpspoof, le flag qu'Alice envoie à Bob est
visible dans l'echo reply que Bob envoie à Alice si le 
participant ne fait qu'un arpspoof en se faisant passer pour
Alice.
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
            .expect("iptable command failed to start");
/*
Change d'utilisateur afin que le container soit présentable,
si le participant rentre dans le container.
*/
    Command::new("su")
            .arg("Alice")

            .spawn()
            .expect("su command failed to start");

    loop {
        /*
        ping le container que le participant contrôlera afin que Bob
        sois enregistré dans sa table arp afin de retrouver son addresse
        ip via un ipneigh
        */
        Command::new("bash")
//                  .arg("02020202020202020202020")
                  .arg("./ping.sh")
                  .arg("666c61673a204d4072633020")
//                .arg("1")
//                .arg("-p")
                /* 
                suite d'espace afin d'éviter que le participant évite
                de prendre les données transmissent comme étant
                le flag
                */
//                .arg("02020202020202020202020")
//                .arg("172.17.0.4")
//                .arg(">")
//                .arg("/dev/null")

                .spawn()
                .expect("ping command failed to start");
        /*
        ping Bob afin de transmetre la flag
        
        Command::new("ping")
                .arg("-c")
                .arg("40")
                .arg("-p")
                // le flag :
                .arg("666c61673a204d4072633020")
                .arg("172.17.0.3")
//                .arg(">")
//                .arg("/dev/null")

                .spawn()
                .expect("ls command failed to start");
          */
    }
}
