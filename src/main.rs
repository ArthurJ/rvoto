extern crate core;

mod ranked_pairs;
mod majority;
mod schulze;
mod printer;

use std::fs;

fn main() {
    let candidatos = load_candidates("candidatas.txt");

    let cedulas = load_cedulas("urna.txt");

    let preferencias = pref_matriz(&cedulas, candidatos.len());

    println!();
    println!("{titulo:-^80}",titulo=" Resultado Não-Processado: ");
    printer::show_raw_results(&preferencias, &candidatos);
    println!("{:-^80}", "");

    println!("\n\n");
    println!("{titulo:-^80}",titulo=" Maioria Simples ");
    let winner = majority::maioria(&cedulas, &candidatos);
    println!("\nPessoa vencedora por Maioria Simples: {}", winner);
    println!("{:-^80}", "");

    println!("\n\n");
    println!("{titulo:-^80}",titulo=" Pares Ranqueados ");
    let winner = ranked_pairs::ranked_pairs(&preferencias, &candidatos);
    println!("\nPessoa vencedora por Pares Ranqueados: {}", winner);
    println!("{:-^80}", "");

    println!("\n\n");
    println!("{titulo:-^80}",titulo=" Método de Schulze ");
    let winner = schulze::schulze(&preferencias, &candidatos);
    println!("\nPessoa vencedora pelo Método de Schulze: {}", winner);
    println!("{:-^80}", "");
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

fn pref_matriz(cedulas: &Vec<Vec<usize>>, qtd_candidatos: usize) -> Vec<Vec<usize>>{
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

