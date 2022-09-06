//para manter a ordem dos candidatos estável garantindo comportamento em caso de empate
use std::collections::BTreeMap as Map;

/*
Em caso de empate, quem tiver o nome mais alto na lista vence
*/

#[derive(Debug)]
pub struct Contagem{
    pub id: usize,
    pub vote_count: usize
}

pub fn maioria(urna: &Vec<Vec<usize>>) -> Vec<Contagem>{
    let mut map:Map<usize, usize> = Map::new();
    for cedula in urna{
        match map.contains_key(&cedula[0]) {
            true => {map.insert(cedula[0],map[&cedula[0]] + 1)},
            false => {map.insert(cedula[0], 1)}
        };
    }
    let mut contagem = Vec::from_iter(map);
    contagem.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    contagem.iter()
        .map(|(cid,vcount)| Contagem{ id:*cid, vote_count:*vcount}).collect()
}