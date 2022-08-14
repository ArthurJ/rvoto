use std::collections::HashMap as Map;

pub fn maioria(urna: Vec<Vec<usize>>, opcoes: Vec<String>) -> String{
    let mut map:Map<usize, usize> = Map::new();
    for cedula in urna{
        match map.contains_key(&cedula[0]) {
            true => {map.insert(cedula[0],map[&cedula[0]] + 1)},
            false => {map.insert(cedula[0], 1)}
        };
    }
    let mut result = Vec::from_iter(map);
    result.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    opcoes[result[0].0].clone()
}
