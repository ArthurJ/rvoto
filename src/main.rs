mod ranked_paris;
mod majority;
mod schulze;

use std::cmp::max;
use std::fs;

fn main() {
    let candidatos = load_candidates("candidatos.txt");
    //println!("Candidatos:\n{:#?}", candidatos);

    let cedulas = load_cedulas("urna.txt");
    //println!("Cédulas:");
    //for cedula in cedulas {
    //    println!("{:?}", cedula);
    //}
    println!();

    let preferencias = pref_matriz(cedulas.clone(), candidatos.len());
    //println!("Matriz de Preferências:");
    //print_matrix(preferencias.clone());

    raw_results_1x1(preferencias.clone(), candidatos.clone());
    println!("----------------------------------------------------------------\n");

    println!("----------------------------------------------------------------");
    let winner = majority::maioria(cedulas, candidatos.clone());
    println!("Opção escolhida pela maioria: {}", winner);
    println!("----------------------------------------------------------------");

    let winner = ranked_paris::ranked_pairs(preferencias.clone(), candidatos.clone());
    println!("----------------------------------------------------------------");
    println!("Opção escolhida por pares ranqueados: {}", winner);
    println!("----------------------------------------------------------------");

    let winner = schulze::schulze(preferencias.clone(), candidatos.clone());
    println!("----------------------------------------------------------------");
    println!("Opção escolhida pelo método de Schulze: {}", winner);
    println!("----------------------------------------------------------------");
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

fn pref_matriz(cedulas: Vec<Vec<usize>>, qtd_candidatos: usize) -> Vec<Vec<usize>>{
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

fn raw_results_1x1(prefs:Vec<Vec<usize>>, candidatos:Vec<String>){
    for (idx,canditato_result) in prefs.clone().iter().enumerate(){
        for (adv,_) in canditato_result.clone().iter().enumerate(){
            if adv==idx{continue}
            let nome = candidatos[idx].clone();
            let adversario = candidatos[adv].clone();
            if prefs[idx][adv] > prefs[adv][idx]{
                println!("{}({}) x {}({}) => {}", nome, prefs[idx][adv], adversario, prefs[adv][idx], nome);
            }else if prefs[idx][adv] == prefs[adv][idx] {
                println!("{}({}) x {}({}) => {}", nome, prefs[idx][adv], adversario, prefs[adv][idx], "Empate");
            }}}}

fn print_matrix<T>(matriz:Vec<Vec<T>>) where T: std::fmt::Display {
    let len = matriz.len();
    let mut max_digits = 1;
    for i in 0..len{
        for j in &matriz[i]{
            max_digits = max(max_digits, j.to_string().len());
        }}

    print_line(len, max_digits);
    println!();
    for i in 0..len{
        print!("| ");
        for (j, value) in matriz[i].iter().enumerate(){
            let to_print = match i==j{
                true => "°".to_string(),
                false => value.to_string(),
            };
            let str_ = format!("{: ^width$}", to_print, width=max_digits+1);
            print!("{} ", str_);
        }
        println!("|");
    }
    print_line(len, max_digits);

    fn print_line(len:usize, max_digits:usize){
        print!("  ");
        let dashes = format!("{:-^width$}", "", width=max_digits+1);
        for _ in 0..len{
            print!("{} ", dashes);
        }}
    println!("\n")
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