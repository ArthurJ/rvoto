extern crate core;

mod ranked_pairs;
mod majority;
mod schulze;
mod printer;

use std::fmt::Display;
use std::fs;

#[allow(unused_imports)]
use majority::Contagem;
use crate::printer::{print_pairwise_results, show_matrix, show_rank};

fn main(){
    by_matrix();
}

fn by_matrix(){
    let base_path = "elections/sz_tie/".to_owned();
    let candidatos = load_candidates((base_path.clone()+"candidatas.txt").as_str());
    let preferencias = load_matrix((base_path+"urna.matrix").as_str());

    println!();
    println!("{titulo:-^80}",titulo=" Resultado Não-Processado: ");
    printer::show_raw_results(&preferencias, &candidatos);
    println!("{:-^80}", "");

    let (rp_result,rp_mtx) = ranked_pairs::ranked_pairs(&preferencias);
    report_condorcet(&candidatos, &rp_result, "Pares Ranqueados", &rp_mtx, "Matriz de Resultado");

    let (s_result, s_paths) = schulze::schulze(&preferencias);
    report_condorcet(&candidatos, &s_result, "Método de Schulze", &s_paths, "Grafo de Preferências");
}

fn by_ballots() {
    let base_path = "elections/25s/".to_owned();
    let candidatos = load_candidates((base_path.clone()+"candidatas.txt").as_str());
    let cedulas = load_cedulas((base_path+"urna.txt").as_str());
    let preferencias = pref_matriz(&cedulas, candidatos.len());

    println!();
    println!("{titulo:-^80}",titulo=" Resultado Não-Processado: ");
    printer::show_raw_results(&preferencias, &candidatos);
    println!("{:-^80}", "");

    println!("\n\n");
    println!("{titulo:-^80}",titulo=" Maioria Simples ");
    let majority_results = majority::maioria(&cedulas);
    println!("Votos por candidata:");
    for resultado in majority_results.iter(){
        println!("\t{}: {} votos.", candidatos[resultado.id], resultado.vote_count)
    }
    println!("\nPessoa vencedora por Maioria Simples: {:?}", candidatos[majority_results[0].id]);
    println!("{:-^80}", "");

    let (rp_result,rp_mtx) = ranked_pairs::ranked_pairs(&preferencias);
    report_condorcet(&candidatos, &rp_result, "Pares Ranqueados", &rp_mtx, "Matriz de Resultado");

    let (s_result, s_paths) = schulze::schulze(&preferencias);
    report_condorcet(&candidatos, &s_result, "Método de Schulze", &s_paths, "Grafo de Preferências");

}

fn report_condorcet<T>(options: &Vec<String>, result: &Vec<usize>, method_id:&str,
                       graph:&Vec<Vec<T>>, graph_id:&str) where T:Display{
    println!("\n\n");
    println!("{titulo:-^80}", titulo = (" ".to_owned()+ method_id.clone()+" ").as_str());
    println!("{}", graph_id);
    show_matrix(graph);
    println!();
    show_rank(result, &options);
    println!("\nPessoa vencedora por {}: {}", method_id, options[result[0]]);
    println!("{:-^80}", "");
}

fn load_matrix(path:&str) -> Vec<Vec<usize>>{
    load_cedulas(path)
}

fn load_cedulas(path: &str) -> Vec<Vec<usize>>{
    let contents = fs::read_to_string(path)
        .expect("Falha ao carregar lista de votos.");
    contents.split("\n")
        .filter(|k| !k.is_empty())
        .map(|x| x.split(",")
                .map(|i| i.trim().parse()
                    .expect(("\nValor problemático encontrado em cédula: ".to_owned() + i + "\n").as_str()))
                .collect::<Vec<usize>>()
    ).collect::<Vec<Vec<usize>>>()
}

fn load_candidates(path: &str) -> Vec<String>{
    let contents = fs::read_to_string(path)
        .expect("Falha ao carregar lista de votos.");
    contents.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn pref_matriz(cedulas: &Vec<Vec<usize>>, qtd_candidatos: usize) -> Vec<Vec<usize>>{
    /*instancia matriz*/
    let mut matriz:Vec<Vec<usize>> = Vec::new();
    for i in 0..qtd_candidatos{
        let mut line = Vec::new();
        for j in 0..qtd_candidatos{
            line.insert(j, 0);
        }
        matriz.insert(i,line);
    }

    /*preenche preferencias*/
    for cedula in cedulas{
        for (pos, candidato) in cedula.clone().iter().enumerate(){
            let cedula_atual = cedula.clone();
            let (perdeu_de, _) = cedula_atual.split_at(pos);
            let vence = (0..qtd_candidatos).filter(|x| !perdeu_de.contains(x));
            for vencido in vence{
                if vencido != *candidato{
                    matriz[*candidato][vencido]+=1;
                }}}}
    matriz
}

fn new_matrix<T:Clone>(dim: usize, filler:T) -> Vec<Vec<T>> {
    let mut clean_mtx:Vec<Vec<T>> = Vec::new();
    for i in 0..dim{
        clean_mtx.insert(i, Vec::new());
        for j in 0..dim{
            clean_mtx[i].insert(j, filler.clone());
        }}
    clean_mtx
}

