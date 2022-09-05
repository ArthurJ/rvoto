#[allow(unused_imports)]
use crate::{new_matrix};
use crate::printer::{show_matrix, show_rank};
use itertools::{iproduct, Itertools};

/*
https://en.wikipedia.org/wiki/Ranked_pairs

Primeiro é gerada uma matriz "tally"
onde o valor de cada celula ixj é a soma das vitórias e derrotas do canditato i contra o canditado j

depois é feita a ordenação dos resultados 1x1, a vitória mais expressiva é a primeira da lista

e usando isso, criamos uma matriz de resultados, onde se o canditato i vence o candidato j:
a posição ixj fica com o valor 1
a posição jxi fica com o valor -1

com essa matriz de resultado, encontramos o candidato com maior preferência (com maior source degree)
Esse é o vencedor

    Empates:
    Da maneira como está existe uma chance remota de acontecerem empates,
    nesse caso, o candidato mais a frente na lista fica com a vitória
    (isso não está nem será implementado aqui:)
    Uma forma de resolver é escolher a opção que tiver menos derrotas ignoradas pelo algoritmo
*/

pub fn ranked_pairs(matriz_urna: &Vec<Vec<usize>>, candidates: &Vec<String>) -> String {
    let pairwise = pairwise_results(matriz_urna);
    let result_mtx = result_matrix(pairwise, matriz_urna.len());

    let mut vote_sums:Vec<isize> = Vec::new();
    for line in result_mtx.clone() {
        vote_sums.push(line.iter().sum());
    }

    let rank:Vec<usize>
        = vote_sums.iter()
            .enumerate()
            .sorted_by(|&(_,a),&(_,b)| a.cmp(b))
            .rev()
            .map(|(candidate,_)| candidate)
            .collect();

    show_rank(&rank, candidates);
    println!("Matriz de Resultado (Pares Rankeados):");
    show_matrix(&result_mtx);
    candidates[rank[0]].clone()
}

fn pairwise_results(prefs: &Vec<Vec<usize>>) -> Vec<(isize, usize, usize)> {
    let tally = create_tally(&prefs);

    let mut results_1x1: Vec<(isize, usize, usize)> = Vec::new();
    for (i, j) in iproduct!(0..prefs.len(), 0..prefs.len()) {
        if i <= j { continue }
        results_1x1.push(get_pair_result(&tally, i, j));
    }

    // println!("\nTally:");
    // show_matrix(&tally);

    results_1x1.sort_by(|&(a, _, _), &(b, _, _)| b.cmp(&a));
    results_1x1
}

fn result_matrix(results_1x1:Vec<(isize, usize, usize)>, dim:usize) -> Vec<Vec<isize>>{
    let mut result_mtx:Vec<Vec<isize>> = new_matrix(dim, 0);

    for par in results_1x1 {
        let (_,a,b) = par;
        match result_mtx[a][b]{
            0=>{
                result_mtx[a][b]=1;
                result_mtx[b][a]=-1;},
            _=> continue, //qualquer não-zero indica que a posição já escrita
        }}
    result_mtx
}

fn get_pair_result(tally:&Vec<Vec<isize>>, opt1:usize, opt2:usize) -> (isize, usize, usize) {
    match tally[opt1][opt2]>0{
        true => (tally[opt1][opt2], opt1, opt2),
        false => (tally[opt2][opt1], opt2, opt1),
    }
}

fn create_tally(prefs:&Vec<Vec<usize>>) -> Vec<Vec<isize>> {
    let mut tally: Vec<Vec<isize>> = Vec::new();
    for (idx,line) in prefs.iter().enumerate(){
        let tally_line:Vec<isize> =
            (*line.iter().map(|x| *x as isize).collect::<Vec<isize>>()).to_vec();
        tally.insert(idx,tally_line);
    }
    let dim = prefs.len();
    for (i,j) in iproduct!(0..dim, 0..dim){
        tally[i][j]-=prefs[j][i] as isize;
    }
    tally
}