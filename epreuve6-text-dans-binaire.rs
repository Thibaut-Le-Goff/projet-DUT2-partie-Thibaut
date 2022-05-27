// Pour rendre cette épreuve réalisable, veuillez compiler ce code avec l'option -g : 
// rustc -g epreuve6-text-dans-binaire.rs

pub mod flag {
// création du module public "flag".

    pub fn flag1_c3st() -> &'static str {
        // Déclaration de la fonction flag1_c3st qui retourne (->) une copie de ce que'elle contient (&) sous 
	// la forme d'une suite de caractères (str) d'une façon permanente ('static, même si l'original 
	// disparaît, la copie reste).

        "Bienvenue dans l'épreuve numéro 6 !"
    }

    pub fn c0ol() -> &'static str {
        "Cette épreuve consiste à chercher les trois différents flags qui se cachent dans ce fichier binaire."
    }

    pub fn rvs1() -> &'static str {
        "Chacun des flags valent un point et sont composés d'un chiffre et de trois lettres... Bonne chance !"
    }
}

fn main () {
    use std::{thread, time};
	// Va chercher (use) dans la librairie standard (std) les modules thread et time.

    println!("{}", flag::flag1_c3st());
	// Écrit ce qui est stocké dans la function flag1_c3st qui est elle même stocké dans le module flag.

    thread::sleep(time::Duration::from_secs(2));
	// Va chercher dans le module thread la fonction sleep dans laquelle l'argument utilisé est la
	// fonction from_secs qui ce trouve dans le sous-module Duration qui est dans le module time. 

    println!("{}", flag::c0ol());
    thread::sleep(time::Duration::from_secs(4));

    println!("{}", flag::rvs1());
    thread::sleep(time::Duration::from_secs(4));
}
