/*
Recebe as preferências,
calcula um grafo da "força de preferência" entre os candidatos
devolve o candidato preferido de acordo com o grafo (o nó de origem no grafo)

https://en.wikipedia.org/wiki/Schulze_method
https://pt.wikipedia.org/wiki/M%C3%A9todo_de_Schulze
*/

#[allow(unused_imports)]
use crate::{new_matrix};
use crate::printer::{show_matrix, show_rank};
use std::cmp::{max, min};
use itertools::Itertools;


pub fn schulze(matriz_urna: &Vec<Vec<usize>>, candidates: &Vec<String>) -> String{
    let paths = schulze_wfi(matriz_urna);
    let winner_path = calc_winner_path(&paths);

    show_rank(&winner_path, &candidates);
    println!("Grafo de Preferência entre Candidatas:");
    show_matrix(&paths);
    //print_pairwise_results(&winner_path, &candidates);
    candidates[winner_path[0]].clone()
}

// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn schulze_wfi(prefs: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let dim = prefs.len();
    let mut strength:Vec<Vec<usize>> = new_matrix(prefs.len(), 0);

    for i in 0..dim{
        for j in  0..dim{
            if prefs[i][j] > prefs[j][i]{
                strength[i][j] = prefs[i][j];
            }}}

    for i in 0..dim{
        for j in  0..dim{
            if i==j { continue }
            for k in 0..dim{
                if i!=k && j!=k{
                    strength[j][k]=max(strength[j][k],
                                       min(strength[j][i], strength[i][k]))
                }}}}
    strength
}

fn calc_winner_path(path_mtx: &Vec<Vec<usize>>) -> Vec<usize>{
    let dim = path_mtx.len();
    let mut victory_path = vec![0usize; dim];

    for i in 0..dim{
        for j in 0..dim {
            if i == j { continue }
            if path_mtx[i][j]>path_mtx[j][i]{
                victory_path[i] +=1;
            }}}

    victory_path.iter().enumerate()
        .sorted_by(|&(_,a),&(_,b)| a.cmp(b))
        .rev()
        .map(|(idx,_)| idx)
        .collect()
}


fn print_pairwise_results(winner_path: &Vec<usize>, candidates:&Vec<String>){
    println!("\nPreferência Geral:");

    for (i,winner) in winner_path.iter().enumerate(){
        for looser in winner_path.split_at(i+1).1{
            println!("\t{} x {} => {}", candidates[*winner], candidates[*looser], candidates[*winner])
        }}
    println!();
}