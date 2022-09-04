use std::collections::HashMap as Map;

use crate::printer::{show_rank};

pub fn maioria(urna: &Vec<Vec<usize>>, opcoes: &Vec<String>) -> String{
    let mut map:Map<usize, usize> = Map::new();
    for cedula in urna{
        match map.contains_key(&cedula[0]) {
            true => {map.insert(cedula[0],map[&cedula[0]] + 1)},
            false => {map.insert(cedula[0], 1)}
        };
    }
    let mut contagem = Vec::from_iter(map);
    contagem.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let rank = contagem.iter().map(|(candidato, _)| *candidato).collect();
    show_rank(&rank, &opcoes);

    opcoes[rank[0]].clone()
}