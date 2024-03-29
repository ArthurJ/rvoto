/*
https://en.wikipedia.org/wiki/Ranked_pairs

Primeiro é gerada uma matriz de contagem onde o valor de cada celula [i][j]
    é a soma das vitórias e derrotas do canditato i contra o canditado j.

Usando isso, criamos uma matriz de resultados onde
    a posição [i][j] fica com o valor 1 e a sua simétrica fica com -1 quando i vence j

Nessa matriz, encontramos a candidata vencedora (com mais vitórias).

Depois é feita a ordenação dos resultados 1x1, onde a candidata com mais vitórias fica em primeiro na lista.

Empate: A opção mais no início da lista de candidatas vence.

    O processo de ordenação é estável, por isso se a pontuação de duas candidatas forem iguais,
    quando a lista é ordenada a posição relativa entre elas se preservaria.
    No entanto o sorted_by gera resultado em ordem crescente, é necessario realizar um `rev` após o `sorted_by`
    para obter o rank com o vencedor no início.
    Isso vai afetar o resultado em caso de empate, por isso o código inclui um `rev` antes do sorted_by,
    que garantirá o nome mais alto na lista como vencedor em caso de empate.
*/

#[allow(unused_imports)]
use crate::{new_matrix};
use itertools::{iproduct, Itertools};

pub fn ranked_pairs(matriz_urna: &Vec<Vec<usize>>) -> (Vec<usize>,Vec<Vec<isize>>) {
    let pairwise = pairwise_results(matriz_urna);
    let result_mtx = result_matrix(pairwise, matriz_urna.len());

    let mut vote_sums:Vec<isize> = Vec::new();
    for line in &result_mtx {
        vote_sums.push(line.iter().sum());
    }

    let rank:Vec<usize>
        = vote_sums.iter()
            .enumerate()
            .rev() // necessário em caso de empate
            .sorted_by(|&(_,a),&(_,b)| a.cmp(b))
            .rev()
            .map(|(candidate,_)| candidate)
            .collect();

    (rank, result_mtx)
}

fn pairwise_results(prefs: &Vec<Vec<usize>>) -> Vec<(isize, usize, usize)> {
    let tally = create_tally(&prefs);

    let mut results_1x1: Vec<(isize, usize, usize)> = Vec::new();
    for (i, j) in iproduct!(0..prefs.len(), 0..prefs.len()) {
        if i <= j { continue }
        results_1x1.push(get_pair_result(&tally, i, j));
    }

    // println!("\Contagem:");
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
        let tally_line: Vec<isize> = line.iter().map(|&x| x as isize).collect();
        tally.insert(idx, tally_line);
    }
    let dim = prefs.len();
    for (i,j) in iproduct!(0..dim, 0..dim){
        tally[i][j]-=prefs[j][i] as isize;
    }
    tally
}