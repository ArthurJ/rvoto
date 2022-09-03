use std::collections::HashMap as Map;

pub fn maioria(urna: Vec<Vec<usize>>, opcoes: Vec<String>) -> String{
    let mut map:Map<usize, usize> = Map::new();
    for cedula in urna{
        match map.contains_key(&cedula[0]) {
            true => {map.insert(cedula[0],map[&cedula[0]] + 1)},
            false => {map.insert(cedula[0], 1)}
        };
    }
    let mut contagem = Vec::from_iter(map);
    contagem.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    print_rank(contagem.clone(), opcoes.clone());
    opcoes[contagem[0].0].clone()
}

fn print_rank(contagem:Vec<(usize, usize)>, opcoes: Vec<String>){
    println!("Em ordem: ");
    for (j,idx) in contagem{
        print!("{}, ",opcoes[j]);
    }
    println!("\n")
}