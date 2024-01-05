use std::collections::HashMap;

use super::pokemon::Pokemon;

macro_rules! define_pokemon {
    ($map:expr, $pokemon:ident) => {
        use super::$pokemon::*;
        $map.insert(stringify!($pokemon), $pokemon);
    };
}
pub struct Pokedex {
    pokemons: HashMap<&'static str, fn() -> Pokemon>
}

impl Pokedex {

    pub fn new() -> Pokedex {
        let mut pokemons: HashMap<&'static str, fn() -> Pokemon> = HashMap::new();

        // Pokemons
        define_pokemon!(pokemons, corpish);
        define_pokemon!(pokemons, darkrai);
        define_pokemon!(pokemons, magikarp);
        define_pokemon!(pokemons, ninetales);
        define_pokemon!(pokemons, pignite);
        define_pokemon!(pokemons, pikachu);
        define_pokemon!(pokemons, tepig);

        Pokedex { pokemons }
    }

    pub fn load_pokemon(&self, name: &str) -> Result<Pokemon, ()> {
        match self.pokemons.get(&name) {
            Some(func) => Ok(func()),
            None => Err(())
        }
    }
}

