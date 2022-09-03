/*
Recebe as preferências,
calcula um grafo da "força de preferência" entre os candidatos
devolve o candidato preferido de acordo com o grafo (o nó de origem no grafo)

https://en.wikipedia.org/wiki/Schulze_method
*/

#[allow(unused_imports)]
use crate::{new_matrix, print_matrix};
use std::cmp::{max, min};
use itertools::Itertools;


pub fn schulze(prefs: Vec<Vec<usize>>, candidates: Vec<String>) -> String{
    let paths = schulze_wfi(prefs);
    let winner_path = calc_winner_path(paths.clone());

    println!("\n\nGrafo de preferência entre candidatos:");
    print_matrix(paths);
    print_schulze_1x1(winner_path.clone(),candidates.clone());
    print_winner_path(winner_path.clone(), candidates.clone());
    candidates[winner_path[0]].clone()
}

//Algoritmo de Floyd-Warshall para o método de Schulze
fn schulze_wfi(prefs: Vec<Vec<usize>>) -> Vec<Vec<usize>>{
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


fn calc_winner_path(path_mtx: Vec<Vec<usize>>) -> Vec<(usize)>{
    let dim = path_mtx.len();
    let mut winner_path = vec![0usize;dim];

    for i in 0..dim{
        for j in 0..dim {
            if i == j { continue }
            if path_mtx[i][j]>path_mtx[j][i]{
                winner_path[i] +=1;
            }}}

    winner_path = winner_path.iter().enumerate()
        .sorted_by(|&(_,a),&(_,b)| a.cmp(b))
        .rev()
        .map(|(idx,wins)| idx)
        .collect();

    winner_path
}

fn print_schulze_1x1(winner_path: Vec<usize>, candidates:Vec<String>){
    println!("Vencedor de Schulze (1x1):");

    for (i,winner) in winner_path.iter().enumerate(){
        for looser in winner_path.split_at(i+1).1{
            println!("\t{} x {} => {}", candidates[*winner], candidates[*looser], candidates[*winner])
        }}}

fn print_winner_path(path:Vec<usize>, candidates: Vec<String>){
    print!("\n(");
    for (j,idx) in path.iter().enumerate(){
        print!("{}",candidates[*idx]);
        if j<path.len()-1{
            print!(" -> ");
        }}
    println!(")\n")
}