use std::{io::{self, Write, Result}, collections::HashMap};
use termios::{Termios, tcsetattr, TCSANOW};

const SEED: &str = "On offering to help the blind man, the man who then stole his car, had not, at that precise moment, had any evil intention, quite the contrary, what he did was nothing more than obey those feelings of generosity and altruism which, as everyone knows, are the two best traits of human nature and to be found in much more hardened criminals than this one, a simple car-thief without any hope of advancing in his profession, exploited by the real owners of this enterprise, for it is they who take advantage of the needs of the poor.";

const ASCII_LIMIT: u8 = 33;
const ASCII_DERIVATION: u8 = 69;

fn main()-> Result<()>{
    let mut input = String::new();
    print!("\x1b[1F\x1b[2KInsert password to encrypt: ");
    io::stdout().flush()?;
    disable_echo(true)?;
    io::stdin().read_line(&mut input)?;

    let mut confirm_input = String::with_capacity(input.len());
    print!("\n\x1b[1F\x1b[2KPlease re-enter the password to confirm: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut confirm_input)?;

    if input != confirm_input {
        return main();
    }

    let map = map_seed(SEED);
    let coded = encrypt(&input, &map);
    print!("\n\x1b[1F\x1b[2KThe encrypted password is: \x1b[2;37;87;190;40m{coded}\x1b[0m");
    disable_echo(false)?;
    Ok(())

}

fn map_seed(seed: &str)-> HashMap<char, u8> {
    let mut map = HashMap::new(); 
    for char in seed.chars() {
        map.entry(char).and_modify(|count| *count += 1).or_insert(1);
    }
    map
}

fn encrypt(password: &str, map: &HashMap<char, u8>)-> String {
    let mut result = String::new();
    for char in password.chars() {
        match map.get(&char) {
            Some(c) => if *c < crate::ASCII_LIMIT {
                result.push((*c + crate::ASCII_DERIVATION) as char);
            } else {
                result.push(*c as char);
            },
            _ => result.push(char)
        } 

    }
    result
}

fn disable_echo(switch: bool)-> Result<()>{
    let mut termios = Termios::from_fd(0)?;

    if switch {
        termios.c_lflag &= !termios::ECHO;
    } else {
        termios.c_lflag |= termios::ECHO;
    }

    tcsetattr(0, TCSANOW, &termios)?;

    Ok(())
}
