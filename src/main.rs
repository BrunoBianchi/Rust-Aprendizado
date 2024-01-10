use std::{io, str::from_utf8, vec, char::from_digit};

fn main() {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number ).expect("Error ao ler");
    let vec: Vec<char> = number.trim().chars().collect();//convertendo a string para um vetor de char
    //trim - > remover os espacos em branco
    //chars retorna os chars da string
    //collect coletar os chars,tranformando em uma colecao
    let mut i:usize = 0;
    let mut sum:i32 = 0;
   while i < vec.len() {
    sum +=  vec[i].to_string().parse::<i32>().unwrap(); // transformo o char em string e dps converto 
    //para int
    i+=1;
   }
   println!("Soma dos digitos: {}",sum);

   let mut fatorial:i128 = 1;
   let mut number_Int:i128 = 0;
   number_Int = number.trim().parse::<i128>().unwrap();
   while number_Int-1 > 0 {
    fatorial *= number_Int;
    number_Int-=1;
   }
   println!("Fatorial de {} eh:{}",number,fatorial);
}
