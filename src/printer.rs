use std::cmp::max;

pub fn show_raw_results(prefs:&Vec<Vec<usize>>, candidatos:&Vec<String>){
    for (idx,canditato_result) in prefs.clone().iter().enumerate(){
        for (adv,_) in canditato_result.clone().iter().enumerate(){
            if adv==idx{continue}
            let nome = candidatos[idx].clone();
            let adversario = candidatos[adv].clone();
            if prefs[idx][adv] > prefs[adv][idx]{
                println!("\t{}({}) x {}({}) => {}", nome, prefs[idx][adv], adversario, prefs[adv][idx], nome);
            }else if prefs[idx][adv] == prefs[adv][idx] {
                println!("\t{}({}) x {}({}) => {}", nome, prefs[idx][adv], adversario, prefs[adv][idx], "Empate");
            }}}
    println!("\nMatriz de Preferências:");
    show_matrix(prefs);
}

fn create_matrix_bounds(len:usize, max_digits:usize){
    print!("  ");
    let dashes = format!("{:-^width$}", "", width=max_digits+1);
    for _ in 0..len{
        print!("{} ", dashes);
    }}

pub fn show_matrix<T>(matriz:&Vec<Vec<T>>) where T: std::fmt::Display {
    let len = matriz.len();
    let max_digits = max_length(matriz, len);

    create_matrix_bounds(len, max_digits);
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
    create_matrix_bounds(len, max_digits);

    println!()
}

fn max_length<T>(matriz: &Vec<Vec<T>>, len: usize) -> usize
    where T: std::fmt::Display {
    let mut max_digits = 1;
    for i in 0..len {
        for j in &matriz[i] {
            max_digits = max(max_digits, j.to_string().len());
        }
    }
    max_digits
}

pub fn show_rank(path:&Vec<usize>, candidates: &Vec<String>){
    print!("Rank:\n");
    for (j,idx) in path.iter().enumerate(){
        print!("{}",candidates[*idx]);
        if j<path.len()-1{
            print!(" -> ");
        }}
    println!("\n")
}