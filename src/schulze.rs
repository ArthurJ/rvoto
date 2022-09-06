/*
Recebe as preferências,
calcula um grafo da "força de preferência" entre os candidatos
devolve o candidato preferido de acordo com o grafo (o nó de origem no grafo)

https://en.wikipedia.org/wiki/Schulze_method
https://pt.wikipedia.org/wiki/M%C3%A9todo_de_Schulze

Empate:
    Nesta implementação, a opção mais no início da lista vence.
    (O processo de ordenação é estável, por isso se a pontuação em duas posições forem iguais,
    quando a lista é ordenada a posição relativa entre elas se preserva)
*/

use crate::{new_matrix};
use crate::printer::{show_matrix};
use std::cmp::{max, min};
use itertools::Itertools;


pub fn schulze(matriz_urna: &Vec<Vec<usize>>) -> Vec<usize>{
    let paths = schulze_wfi(matriz_urna);
    let winner_path = calc_winner_path(&paths);

    println!("Grafo de Preferência entre Candidatas:");
    show_matrix(&paths);
    println!();
    winner_path
}

// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn schulze_wfi(prefs: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let dim = prefs.len();
    let mut strengths:Vec<Vec<usize>> = new_matrix(prefs.len(), 0);

    for i in 0..dim{
        for j in  0..dim{
            if prefs[i][j] > prefs[j][i]{
                strengths[i][j] = prefs[i][j];
            }}}

    for i in 0..dim{
        for j in  0..dim{
            if i==j { continue }
            for k in 0..dim{
                if i!=k && j!=k{
                    strengths[j][k]=max(strengths[j][k],
                                        min(strengths[j][i], strengths[i][k]))
                }}}}
    strengths
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