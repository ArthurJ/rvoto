use std::collections::HashMap as Map;

use crate::printer::{show_rank};

/*
Em caso de empate, quem tiver o nome mais alto na lista vence
*/

pub fn maioria(urna: &Vec<Vec<usize>>, candidates: &Vec<String>) -> String{
    let mut map:Map<usize, usize> = Map::new();
    for cedula in urna{
        match map.contains_key(&cedula[0]) {
            true => {map.insert(cedula[0],map[&cedula[0]] + 1)},
            false => {map.insert(cedula[0], 1)}
        };
    }
    let mut contagem = Vec::from_iter(map);
    contagem.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let rank:Vec<usize> = contagem.iter().map(|(candidato, _)| *candidato).collect();
    //show_rank(&rank, &candidates);

    println!("Votos por candidata:");
    for (candidato,votos) in contagem.iter(){
        println!("\t{}: {} votos.", candidates[*candidato], votos)
    }
    println!();

    candidates[rank[0]].clone()
}