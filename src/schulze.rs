/*
Recebe as preferências,
calcula um grafo da "força de preferência" entre os candidatos
devolve o candidato preferido de acordo com o grafo (o nó de origem no grafo)

https://en.wikipedia.org/wiki/Schulze_method
*/

#[allow(unused_imports)]
use crate::{new_matrix, print_matrix};
use std::cmp::{max, min};


pub fn schulze(prefs: Vec<Vec<usize>>, candidates: Vec<String>) -> String{
    let dim = prefs.len();
    let paths = schulze_wfi(prefs);
    let mut winner_idx= candidates.len();

    //println!("\n\nVencedor de Schulze (1x1):\n");
    for i in 0..dim{
        let mut winning = true;
        for j in 0..dim{
            if i!=j{
                winning = winning && (paths[i][j]>=paths[j][i]);

                //match paths[i][j]>paths[j][i]{
                //    true => println!("{} x {} => {}", opcoes[i], opcoes[j], opcoes[i]),
                //    false => println!("{} x {} => {}", opcoes[i], opcoes[j], opcoes[j]),
                //}

                if !winning{break} /*Caso esteja imprimindo 1x1, comente essa linha*/
            }
        }
        if winning{
            winner_idx = i;
            break; /*Caso esteja imprimindo 1x1, comente essa linha*/
        }
    }

    println!("\n\nGrafo de preferência entre candidatos:");
    print_matrix(paths);

    candidates[winner_idx].clone()
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
            if i!=j{
                for k in 0..dim{
                    if i!=k && j!=k{
                        strength[j][k]=max(strength[j][k],
                                           min(strength[j][i], strength[i][k]))
                    }}}}}
    strength
}
